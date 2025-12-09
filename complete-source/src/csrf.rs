use tower_sessions::Session;
use uuid::Uuid;

pub async fn get_or_create_csrf_token(session: &Session) -> String {
    if let Ok(Some(token)) = session.get::<String>("csrf_token").await {
        token
    } else {
        let token = Uuid::new_v4().to_string();
        session.insert("csrf_token", &token).await.ok();
        token
    }
}

pub async fn verify_csrf_token(session: &Session, token: &str) -> bool {
    if let Ok(Some(stored_token)) = session.get::<String>("csrf_token").await {
        stored_token == token
    } else {
        false
    }
}
