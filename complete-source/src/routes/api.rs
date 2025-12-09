use axum::{
    extract::{Multipart, State},
    response::{IntoResponse, Json},
    http::{StatusCode, HeaderMap},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use tower_sessions::Session;
use crate::csrf::verify_csrf_token;
use crate::AppState;

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub original_name: String,
}

pub async fn upload_handler(
    State(state): State<AppState>,
    session: Session,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // CSRF Check
    let csrf_header = headers.get("X-CSRF-Token").and_then(|v| v.to_str().ok()).unwrap_or("");
    if !verify_csrf_token(&session, csrf_header).await {
         return (StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }

    let mut uploaded_files = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap_or_default();
        
        if data.is_empty() { continue; }

        let url = crate::upload::upload_file(&state.cloudinary, data.to_vec(), &file_name, "auto").await;
        if let Ok(url) = url {
            uploaded_files.push(json!({
                "original_name": file_name,
                "url": url
            }));
        }
    }

    if uploaded_files.is_empty() {
        return (StatusCode::BAD_REQUEST, "No files uploaded").into_response();
    }

    // Return the first file for simplicity in this specific JS impl, or list
    Json(uploaded_files[0].clone()).into_response()
}

#[derive(Deserialize)]
pub struct ReorderUpdate {
    pub id: i32,
    pub sort_order: i32,
}

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub updates: Vec<ReorderUpdate>,
}

pub async fn reorder_handler(
    State(pool): State<PgPool>,
    session: Session,
    headers: HeaderMap,
    Json(payload): Json<ReorderRequest>,
) -> impl IntoResponse {
    // CSRF Check
    let csrf_header = headers.get("X-CSRF-Token").and_then(|v| v.to_str().ok()).unwrap_or("");
    if !verify_csrf_token(&session, csrf_header).await {
         return (StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }

    for update in payload.updates {
        sqlx::query("UPDATE blocks SET sort_order = $1 WHERE id = $2")
            .bind(update.sort_order)
            .bind(update.id)
            .execute(&pool)
            .await
            .ok();
    }

    StatusCode::OK.into_response()
}
