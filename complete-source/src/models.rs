use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::Date;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Page {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub updated_at: time::OffsetDateTime,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false, // Invalid hash format (e.g. legacy SHA1)
    };
    
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct ContentBlock {
    pub id: Uuid,
    pub project_id: Uuid,
    pub block_type: String, // text, gallery, video, audio, file
    pub content: sqlx::types::Json<BlockContent>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum BlockContent {
    Text(String),
    Gallery(Vec<String>), // List of image URLs
    Video(String),        // Embed URL
    Audio(Vec<(String, String)>), // List of (url, title)
    File(Vec<(String, String)>), // List of (url, description)
}

impl BlockContent {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            BlockContent::Text(s) => Some(s),
            _ => None,
        }
    }
}
