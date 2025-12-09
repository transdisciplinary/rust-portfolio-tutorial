pub mod models;
pub mod routes;
pub mod templates;
pub mod upload;

#[cfg(test)]
mod test_json;

use axum::{
    routing::{get, post},
    Router,
    extract::{DefaultBodyLimit, Request, Path},
    middleware::{self, Next},
    response::{Response, Redirect, IntoResponse},
    http::{StatusCode, header},
    body::Body,
};
use sqlx::PgPool;
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
use upload::CloudinaryConfig;
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/static");

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub cloudinary: CloudinaryConfig,
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for CloudinaryConfig {
    fn from_ref(state: &AppState) -> Self {
        state.cloudinary.clone()
    }
}

pub fn create_router(state: AppState, is_production: bool) -> Router {
    let session_store = MemoryStore::default();
    
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(is_production)
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)));

    Router::new()
        .route("/", get(routes::public::index))
        .route("/contact", get(routes::public::contact))
        .route("/about", get(routes::public::about))
        .route("/project/{slug}", get(routes::public::project_details))
        .route("/admin/login", get(routes::auth::login_page).post(routes::auth::login_handler))
        .route("/admin/logout", get(routes::auth::logout_handler))
        // Protected Admin Routes
        .nest("/admin", admin_routes())
        .route("/static/{*path}", get(static_handler))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50MB limit
        .layer(session_layer)
        .with_state(state)
}

async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    
    if let Some(file) = STATIC_DIR.get_file(path) {
        let mime_type = mime_guess::from_path(path).first_or_octet_stream();
        (
            [(header::CONTENT_TYPE, mime_type.as_ref())],
            Body::from(file.contents().to_vec()) // Clone the bytes
        ).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(routes::admin::dashboard))
        .route("/settings", get(routes::admin::settings))
        .route("/settings/update", post(routes::admin::update_credentials))
        .route("/projects", post(routes::admin::create_project))
        .route("/projects/new", get(routes::admin::new_project))
        .route("/projects/edit/{id}", get(routes::admin::edit_project).post(routes::admin::update_project))
        .route("/projects/delete/{id}", post(routes::admin::delete_project))
        .route("/projects/{id}/blocks/new", get(routes::admin::new_block))
        .route("/blocks/{id}", get(routes::admin::edit_block).post(routes::admin::update_block))
        .route("/blocks/delete/{id}", post(routes::admin::delete_block))
        .route("/projects/{id}/blocks", get(routes::admin::project_blocks).post(routes::admin::create_block))
        // Pages
        .route("/pages", get(routes::admin::pages_list))
        .route("/pages/edit/{slug}", get(routes::admin::edit_page))
        .route("/pages/update/{slug}", post(routes::admin::update_page))
        // API Routes
        .route("/api/upload", post(routes::api::upload_handler))
        .route("/api/reorder", post(routes::api::reorder_handler))
        .route("/deploy", post(routes::admin::trigger_deploy))
        .route_layer(middleware::from_fn(auth_middleware))
        .route_layer(middleware::from_fn(no_cache_middleware))
}

async fn no_cache_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert("Cache-Control", "no-store, no-cache, must-revalidate, proxy-revalidate".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());
    response
}

async fn auth_middleware(session: Session, req: Request, next: Next) -> Response {
    if session.get::<uuid::Uuid>("user_id").await.unwrap_or(None).is_some() {
        next.run(req).await
    } else {
        Redirect::to("/admin/login").into_response()
    }
}
