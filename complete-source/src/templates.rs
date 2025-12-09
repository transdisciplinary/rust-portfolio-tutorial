use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum::http::StatusCode;
use crate::models::{Project, ContentBlock, BlockContent, Page};
use uuid::Uuid;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub grouped_projects: Vec<(i32, Vec<Project>)>,
    pub footer: String,
}

#[derive(Template)]
#[template(path = "admin/pages_list.html")]
pub struct PagesListTemplate {
    pub pages: Vec<Page>,
}

#[derive(Template)]
#[template(path = "admin/page_form.html")]
pub struct PageFormTemplate {
    pub page: Page,
}

#[derive(Template)]
#[template(path = "project.html")]
pub struct ProjectTemplate {
    pub project: Project,
    pub blocks: Vec<ContentBlock>,
    pub next_project: Option<Project>,
    pub prev_project: Option<Project>,
    pub footer: String,
}

#[derive(Template)]
#[template(path = "admin/project_form.html")]
pub struct ProjectFormTemplate {
    pub project: Option<Project>,
}

#[derive(Template)]
#[template(path = "admin/project_blocks.html")]
pub struct ProjectBlocksTemplate {
    pub project_id: Uuid,
    pub project_title: String,
    pub blocks: Vec<ContentBlock>,
}

#[derive(Template)]
#[template(path = "admin/block_form.html")]
pub struct BlockFormTemplate {
    pub project_id: Uuid,
    pub block_id: Option<Uuid>,
    pub block_type: String,
    pub sort_order: i32,
    pub content: String,
}

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
pub struct DashboardTemplate {
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate {
    pub page: Page,
    pub footer: String,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub page: Page,
    pub footer: String,
}

#[derive(Template)]
#[template(path = "admin/settings.html")]
pub struct SettingsTemplate {
    pub current_username: String,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}


// Manual implementation of IntoResponse for templates to avoid askama_axum dependency issues
impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for ProjectTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for DashboardTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for ProjectFormTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for ProjectBlocksTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for BlockFormTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for LoginTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for ContactTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for AboutTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for SettingsTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for PagesListTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}

impl IntoResponse for PageFormTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response(),
        }
    }
}


