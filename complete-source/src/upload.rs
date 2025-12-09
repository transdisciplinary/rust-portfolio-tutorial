use serde::Deserialize;
use sha1::{Digest, Sha1};

#[derive(Clone)]
pub struct CloudinaryConfig {
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
}

impl CloudinaryConfig {
    pub fn new(cloud_name: String, api_key: String, api_secret: String) -> Self {
        Self {
            cloud_name,
            api_key,
            api_secret,
        }
    }
}

#[derive(Deserialize)]
struct CloudinaryResponse {
    secure_url: String,
}

pub async fn upload_file(
    config: &CloudinaryConfig,
    file_bytes: Vec<u8>,
    filename: &str,
    resource_type: &str, // "image", "video", or "auto"
) -> Result<String, String> {
    let client = reqwest::Client::new();
    
    let timestamp = chrono::Utc::now().timestamp().to_string();
    let to_sign = format!("timestamp={}{}", timestamp, config.api_secret);
    
    let mut hasher = Sha1::new();
    hasher.update(to_sign);
    let signature = format!("{:x}", hasher.finalize());
    
    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name(filename.to_string());

    let form = reqwest::multipart::Form::new()
        .text("api_key", config.api_key.clone())
        .text("timestamp", timestamp)
        .text("signature", signature)
        .part("file", part);

    let url = format!("https://api.cloudinary.com/v1_1/{}/{}/upload", config.cloud_name, resource_type);
    
    let resp = client.post(&url)
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Upload failed: {}", text));
    }

    let data: CloudinaryResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data.secure_url)
}
