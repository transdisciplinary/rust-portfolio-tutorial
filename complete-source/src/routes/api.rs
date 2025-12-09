use axum::{
    extract::{Multipart, State},
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::upload::{CloudinaryConfig, upload_file};

#[derive(Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub original_name: String,
}

pub async fn upload_handler(
    State(cloudinary): State<CloudinaryConfig>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut url = String::new();
    let mut original_name = String::new();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or_default().to_string();
        
        if name == "file" {
            if let Some(fname) = field.file_name() {
                original_name = fname.to_string();
                let bytes = field.bytes().await.unwrap_or_default();
                if !bytes.is_empty() {
                    // Determine resource type based on extension
                    let lower_name = original_name.to_lowercase();
                    let resource_type = if lower_name.ends_with(".mp3") || lower_name.ends_with(".wav") || lower_name.ends_with(".ogg") || lower_name.ends_with(".m4a") || lower_name.ends_with(".aac") {
                        "video" // Cloudinary treats audio as video
                    } else if lower_name.ends_with(".pdf") {
                        "raw" // Use raw for PDFs to avoid delivery blocking issues
                    } else {
                        "auto"
                    };

                    match upload_file(&cloudinary, bytes.to_vec(), &original_name, resource_type).await {
                        Ok(uploaded_url) => url = uploaded_url,
                        Err(e) => return Json(json!({ "error": e })),
                    }
                }
            }
        }
    }

    if url.is_empty() {
        return Json(json!({ "error": "No file uploaded" }));
    }

    Json(json!(UploadResponse {
        url,
        original_name,
    }))
}

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub updates: Vec<BlockOrderUpdate>,
}

#[derive(Deserialize)]
pub struct BlockOrderUpdate {
    pub id: Uuid,
    pub sort_order: i32,
}

pub async fn reorder_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<ReorderRequest>,
) -> impl IntoResponse {
    for update in payload.updates {
        let _ = sqlx::query("UPDATE content_blocks SET sort_order = $1 WHERE id = $2")
            .bind(update.sort_order)
            .bind(update.id)
            .execute(&pool)
            .await;
    }

    Json(json!({ "success": true }))
}
