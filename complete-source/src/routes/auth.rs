use axum::{
    response::{IntoResponse, Redirect},
    Form,
    extract::{State, Query},
};
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::User;
use tower_sessions::Session;
use crate::csrf::{get_or_create_csrf_token, verify_csrf_token};

use crate::templates::LoginTemplate;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub authenticity_token: String,
}

#[derive(Deserialize)]
pub struct LoginQuery {
    pub error: Option<String>,
}

pub async fn login_page(session: Session, Query(params): Query<LoginQuery>) -> impl IntoResponse {
    let csrf_token = get_or_create_csrf_token(&session).await;
    LoginTemplate { 
        error: params.error, 
        authenticity_token: csrf_token 
    }
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    session: Session,
    Form(payload): Form<LoginRequest>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &payload.authenticity_token).await {
        return Redirect::to("/admin/login?error=Session expired (CSRF)").into_response();
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

    if let Some(user) = user {
        if crate::models::verify_password(&payload.password, &user.password_hash) {
            session.insert("user_id", user.id).await.unwrap();
            return Redirect::to("/admin/dashboard").into_response();
        }
    }

    Redirect::to("/admin/login?error=Invalid credentials").into_response()
}

pub async fn logout_handler(session: Session) -> impl IntoResponse {
    session.flush().await.unwrap();
    Redirect::to("/admin/login")
}
