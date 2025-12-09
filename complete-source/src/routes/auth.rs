use axum::{
    response::{IntoResponse, Redirect},
    Form,
    extract::State,
};
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::User;
use tower_sessions::Session;

use crate::templates::LoginTemplate;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login_page() -> impl IntoResponse {
    LoginTemplate { error: None }
}

pub async fn login_handler(
    State(pool): State<PgPool>,
    session: Session,
    Form(payload): Form<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

    if let Some(user) = user {
        if crate::models::verify_password(&payload.password, &user.password_hash) {
            session.insert("user_id", user.id).await.unwrap();
            return Redirect::to("/admin/dashboard");
        }
    }

    Redirect::to("/admin/login?error=Invalid credentials")
}

pub async fn logout_handler(session: Session) -> impl IntoResponse {
    session.flush().await.unwrap();
    Redirect::to("/admin/login")
}
