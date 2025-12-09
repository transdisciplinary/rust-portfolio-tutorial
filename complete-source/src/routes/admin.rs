use axum::{
    extract::{Path, State, Query},
    response::{IntoResponse, Redirect, Html, Response},
    Form,
    http::HeaderMap,
};
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use time::Date;
use crate::models::{Project, ContentBlock, BlockContent, User};
use crate::templates::{DashboardTemplate, ProjectFormTemplate, ProjectBlocksTemplate, BlockFormTemplate, SettingsTemplate, PagesListTemplate, PageFormTemplate};
use tower_sessions::Session;
use crate::csrf::{get_or_create_csrf_token, verify_csrf_token};

#[derive(Deserialize)]
pub struct DeleteForm {
    pub authenticity_token: String,
}

#[derive(Deserialize)]
pub struct DeployForm {
    pub authenticity_token: String,
}


#[derive(Deserialize)]
pub struct ProjectForm {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub authenticity_token: String,
}

pub async fn dashboard(
    State(pool): State<PgPool>,
    session: Session,
) -> impl IntoResponse {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY start_date DESC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let csrf_token = get_or_create_csrf_token(&session).await;
    DashboardTemplate { 
        projects,
        authenticity_token: csrf_token,
    }
}

pub async fn new_project(session: Session) -> impl IntoResponse {
    let csrf_token = get_or_create_csrf_token(&session).await;
    ProjectFormTemplate { 
        project: None,
        authenticity_token: csrf_token,
    }
}

pub async fn create_project(
    State(pool): State<PgPool>,
    session: Session,
    Form(payload): Form<ProjectForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &payload.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    let start_date = parse_date(&payload.start_date);
    let end_date = parse_date_option(payload.end_date);

    sqlx::query(
        "INSERT INTO projects (title, slug, description, start_date, end_date) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(payload.title)
    .bind(payload.slug)
    .bind(payload.description)
    .bind(start_date)
    .bind(end_date)
    .execute(&pool)
    .await
    .unwrap();

    Redirect::to("/admin/dashboard").into_response()
}

pub async fn edit_project(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    session: Session,
) -> impl IntoResponse {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .ok();

    let csrf_token = get_or_create_csrf_token(&session).await;
    ProjectFormTemplate { 
        project,
        authenticity_token: csrf_token,
    }
}

pub async fn update_project(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    session: Session,
    Form(payload): Form<ProjectForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &payload.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    let start_date = parse_date(&payload.start_date);
    let end_date = parse_date_option(payload.end_date);

    sqlx::query(
        "UPDATE projects SET title = $1, slug = $2, description = $3, start_date = $4, end_date = $5, updated_at = NOW() WHERE id = $6"
    )
    .bind(payload.title)
    .bind(payload.slug)
    .bind(payload.description)
    .bind(start_date)
    .bind(end_date)
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();

    Redirect::to("/admin/dashboard").into_response()
}

pub async fn delete_project(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    session: Session,
    Form(form): Form<DeleteForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &form.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Redirect::to("/admin/dashboard").into_response()
}

// --- Block CRUD ---

pub async fn project_blocks(
    State(pool): State<PgPool>,
    Path(project_id): Path<Uuid>,
    session: Session,
) -> impl IntoResponse {
    // Fetch project title
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&pool)
        .await
        .unwrap(); 

    let blocks = sqlx::query_as::<_, ContentBlock>(
        "SELECT * FROM content_blocks WHERE project_id = $1 ORDER BY sort_order ASC"
    )
    .bind(project_id)
    .fetch_all(&pool)
    .await
    .unwrap(); 

    let csrf_token = get_or_create_csrf_token(&session).await;
    ProjectBlocksTemplate {
        project_id,
        project_title: project.title,
        blocks,
        authenticity_token: csrf_token,
    }
}

#[derive(Deserialize)]
pub struct NewBlockQuery {
    #[serde(rename = "type")]
    pub block_type: String,
}

pub async fn new_block(
    Path(project_id): Path<Uuid>,
    session: Session,
    Query(query): Query<NewBlockQuery>,
) -> impl IntoResponse {
    let csrf_token = get_or_create_csrf_token(&session).await;
    BlockFormTemplate {
        project_id,
        block_id: None,
        block_type: query.block_type,
        sort_order: 0,
        content: String::new(),
        authenticity_token: csrf_token,
    }
}

#[derive(Deserialize)]
pub struct BlockForm {
    pub block_type: String,
    pub sort_order: i32,
    pub content: String,
    pub authenticity_token: String,
}

pub async fn create_block(
    State(pool): State<PgPool>,
    Path(project_id): Path<Uuid>,
    headers: HeaderMap,
    session: Session,
    Form(form): Form<BlockForm>,
) -> Response {
    if !verify_csrf_token(&session, &form.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    // Debug logging
    println!("=== CREATE BLOCK DEBUG ===");
    println!("Block type: {}", form.block_type);
    println!("Content length: {}", form.content.len());
    println!("Content preview: {:?}", &form.content.chars().take(200).collect::<String>());
    
    let content_enum = form_to_block_content(&form.block_type, &form.content);

    sqlx::query(
        "INSERT INTO content_blocks (id, project_id, block_type, content, sort_order) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(Uuid::new_v4())
    .bind(project_id)
    .bind(form.block_type)
    .bind(sqlx::types::Json(content_enum))
    .bind(form.sort_order)
    .execute(&pool)
    .await
    .unwrap();

    // Check if this is an HTMX request
    if headers.get("hx-request").is_some() {
        // Return the blocks list HTML for HTMX to swap
        // We reuse recent token. Ideally for HTMX we might want a new one if rotated.
        let csrf_token = get_or_create_csrf_token(&session).await;
        render_blocks_list(pool, project_id, &csrf_token).await.into_response()
    } else {
        // Regular form submission, redirect
        Redirect::to(&format!("/admin/projects/{}/blocks", project_id)).into_response()
    }
}

pub async fn edit_block(
    State(pool): State<PgPool>,
    Path(block_id): Path<Uuid>,
    session: Session,
) -> impl IntoResponse {
    let block = sqlx::query_as::<_, ContentBlock>(
        "SELECT * FROM content_blocks WHERE id = $1"
    )
    .bind(block_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let (content_str, _extra) = match block.content.0 {
        BlockContent::Text(s) => (s, String::new()),
        BlockContent::Video(s) => (s, String::new()),
        BlockContent::Gallery(urls) => (serde_json::to_string(&urls).unwrap_or_default(), String::new()),
        BlockContent::Audio(items) => (serde_json::to_string(&items).unwrap_or_default(), String::new()),
        BlockContent::File(items) => (serde_json::to_string(&items).unwrap_or_default(), String::new()),
    };

    let csrf_token = get_or_create_csrf_token(&session).await;
    BlockFormTemplate {
        project_id: block.project_id,
        block_id: Some(block.id),
        block_type: block.block_type,
        sort_order: block.sort_order,
        content: content_str,
        authenticity_token: csrf_token,
    }
}

pub async fn update_block(
    State(pool): State<PgPool>,
    Path(block_id): Path<Uuid>,
    headers: HeaderMap,
    session: Session,
    Form(form): Form<BlockForm>,
) -> Response {
    if !verify_csrf_token(&session, &form.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    // Fetch block to get project_id
    let block = sqlx::query_as::<_, ContentBlock>(
        "SELECT * FROM content_blocks WHERE id = $1"
    )
    .bind(block_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let content_enum = form_to_block_content(&form.block_type, &form.content);

    sqlx::query(
        "UPDATE content_blocks SET sort_order = $1, content = $2 WHERE id = $3"
    )
    .bind(form.sort_order)
    .bind(sqlx::types::Json(content_enum))
    .bind(block_id)
    .execute(&pool)
    .await
    .unwrap();

    // Check if this is an HTMX request
    if headers.get("hx-request").is_some() {
        // Return the blocks list HTML for HTMX to swap
        // Return the blocks list HTML for HTMX to swap
        let csrf_token = get_or_create_csrf_token(&session).await;
        render_blocks_list(pool, block.project_id, &csrf_token).await.into_response()
    } else {
        // Regular form submission, redirect
        Redirect::to(&format!("/admin/projects/{}/blocks", block.project_id)).into_response()
    }
}

pub async fn delete_block(
    State(pool): State<PgPool>,
    Path(block_id): Path<Uuid>,
    session: Session,
    Form(form): Form<DeleteForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &form.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    let block = sqlx::query_as::<_, ContentBlock>(
        "SELECT * FROM content_blocks WHERE id = $1"
    )
    .bind(block_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    sqlx::query("DELETE FROM content_blocks WHERE id = $1")
        .bind(block_id)
        .execute(&pool)
        .await
        .unwrap();

    Redirect::to(&format!("/admin/projects/{}/blocks", block.project_id)).into_response()
}

// --- Settings ---

pub async fn settings(
    State(pool): State<PgPool>,
    session: Session,
) -> impl IntoResponse {
    // Assuming single user system, fetch the first user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users LIMIT 1")
        .fetch_one(&pool)
        .await
        .ok();

    let current_username = user.map(|u| u.username).unwrap_or_default();

    let csrf_token = get_or_create_csrf_token(&session).await;
    SettingsTemplate { 
        current_username,
        authenticity_token: csrf_token,
    }
}

#[derive(Deserialize)]
pub struct CredentialsForm {
    pub username: String,
    pub password: String,
    pub authenticity_token: String,
}

pub async fn update_credentials(
    State(pool): State<PgPool>,
    session: Session,
    Form(payload): Form<CredentialsForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &payload.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    // Hash password
    let password_hash = crate::models::hash_password(&payload.password).unwrap();

    // Update the first user found (or specific ID if we had session)
    // For now, we update the single user record
    // If no user exists, we insert one? Or assume one exists.
    // Let's try to update, if 0 rows affected, insert.
    
    let rows_affected = sqlx::query(
        "UPDATE users SET username = $1, password_hash = $2 WHERE id = (SELECT id FROM users LIMIT 1)"
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .execute(&pool)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        // Insert default user if none exists
        sqlx::query(
            "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)"
        )
        .bind(Uuid::new_v4())
        .bind(&payload.username)
        .bind(&password_hash)
        .execute(&pool)
        .await
        .unwrap();
    }

    Redirect::to("/admin/dashboard").into_response()
}

// --- Pages ---

pub async fn pages_list(
    State(pool): State<PgPool>,
    session: Session,
) -> impl IntoResponse {
    let pages = sqlx::query_as::<_, crate::models::Page>("SELECT * FROM pages ORDER BY slug ASC")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let csrf_token = get_or_create_csrf_token(&session).await;
    crate::templates::PagesListTemplate { 
        pages,
        authenticity_token: csrf_token,
    }
}

pub async fn edit_page(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    session: Session,
) -> impl IntoResponse {
    let page = sqlx::query_as::<_, crate::models::Page>("SELECT * FROM pages WHERE slug = $1")
        .bind(slug)
        .fetch_one(&pool)
        .await
        .unwrap();

    let csrf_token = get_or_create_csrf_token(&session).await;
    crate::templates::PageFormTemplate { 
        page,
        authenticity_token: csrf_token,
    }
}

#[derive(Deserialize)]
pub struct PageForm {
    pub title: String,
    pub content: String,
    pub authenticity_token: String,
}

pub async fn update_page(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    session: Session,
    Form(form): Form<PageForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &form.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    sqlx::query("UPDATE pages SET title = $1, content = $2, updated_at = NOW() WHERE slug = $3")
        .bind(form.title)
        .bind(form.content)
        .bind(slug)
        .execute(&pool)
        .await
        .unwrap();
    Redirect::to("/admin/pages").into_response()
}

// --- Helpers ---

fn parse_date(s: &str) -> Date {
    Date::parse(s, &time::format_description::well_known::Iso8601::DEFAULT).unwrap_or(Date::MIN)
}

fn parse_date_option(s: Option<String>) -> Option<Date> {
    s.and_then(|d| Date::parse(&d, &time::format_description::well_known::Iso8601::DEFAULT).ok())
}

fn form_to_block_content(block_type: &str, content: &str) -> BlockContent {
    match block_type.to_lowercase().as_str() {
        "text" => BlockContent::Text(content.to_string()),
        "video" => BlockContent::Video(content.to_string()),
        "gallery" => {
            let items: Vec<String> = serde_json::from_str(content).unwrap_or_default();
            BlockContent::Gallery(items)
        },
        "audio" => {
            let items: Vec<(String, String)> = serde_json::from_str(content).unwrap_or_default();
            BlockContent::Audio(items)
        },
        "file" => {
            let items: Vec<(String, String)> = serde_json::from_str(content).unwrap_or_default();
            BlockContent::File(items)
        },
        _ => BlockContent::Text(content.to_string()),
    }
}

// Helper function to render the blocks list for HTMX responses
async fn render_blocks_list(pool: PgPool, project_id: Uuid, csrf_token: &str) -> impl IntoResponse {
    let blocks = sqlx::query_as::<_, ContentBlock>(
        "SELECT * FROM content_blocks WHERE project_id = $1 ORDER BY sort_order ASC"
    )
    .bind(project_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    // Render just the blocks list partial matching the template structure
    let mut html = String::new();
    for block in blocks {
        let preview = match &block.content.0 {
            BlockContent::Text(content) => {
                // Truncate HTML for preview, removing tags
                let text_only: String = content
                    .chars()
                    .filter(|c| !matches!(c, '<' | '>'))
                    .take(50)
                    .collect();
                text_only
            },
            BlockContent::Video(url) => format!("Video: {}", url),
            BlockContent::Gallery(urls) => format!("{} images", urls.len()),
            BlockContent::Audio(items) => format!("{} audio files", items.len()),
            BlockContent::File(items) => format!("{} files", items.len()),
        };

        html.push_str(&format!(
            r#"<div class="block-item" data-id="{}">
            <div class="drag-handle material-icons">drag_indicator</div>
            <div class="block-info">
                <span class="block-type">{}</span>
                <span class="block-preview">{}</span>
            </div>
            <div class="block-actions">
                <a href="/admin/blocks/{}" class="icon-btn" title="Edit">
                    <span class="material-icons">edit</span>
                </a>
                <form method="POST" action="/admin/blocks/delete/{}" class="inline-form confirm-delete">
                    <input type="hidden" name="authenticity_token" value="{}">
                    <button type="submit" class="icon-btn delete" title="Delete">
                        <span class="material-icons">delete</span>
                    </button>
                </form>
            </div>
        </div>
"#,
            block.id,
            block.block_type,
            preview,
            block.id,
            block.id,
            csrf_token
        ));
    }

    Html(html)
}


// --- Deployment ---

pub async fn trigger_deploy(
    State(_pool): State<PgPool>,
    session: Session,
    Form(form): Form<DeployForm>,
) -> impl IntoResponse {
    if !verify_csrf_token(&session, &form.authenticity_token).await {
        return (axum::http::StatusCode::FORBIDDEN, "Invalid CSRF Token").into_response();
    }
    let github_token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let owner = std::env::var("GITHUB_OWNER").unwrap_or_default();
    let repo = std::env::var("GITHUB_REPO").unwrap_or_default();

    if github_token.is_empty() || owner.is_empty() || repo.is_empty() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            Html("Missing GitHub configuration (GITHUB_TOKEN, GITHUB_OWNER, GITHUB_REPO)".to_string()),
        ).into_response();
    }

    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/dispatches", owner, repo);

    let res = client
        .post(&url)
        .header("Authorization", format!("token {}", github_token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "shuttle-app")
        .json(&serde_json::json!({
            "event_type": "deploy_static"
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                Redirect::to("/admin/dashboard?deploy=success").into_response()
            } else {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!("GitHub API Error: {} - {}", status, text)),
                ).into_response()
            }
        }
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("Failed to send request: {}", e)),
        ).into_response(),
    }
}
