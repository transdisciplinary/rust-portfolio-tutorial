use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::PgPool;
use crate::templates::{IndexTemplate, ProjectTemplate, ContactTemplate, AboutTemplate};
use crate::models::{Project, ContentBlock, Page};

async fn get_footer(pool: &PgPool) -> String {
    sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = 'footer'")
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
        .map(|p| p.content)
        .unwrap_or_else(|| "<p>&copy; 2024</p>".to_string())
}

pub async fn get_index_template(pool: &PgPool) -> IndexTemplate {
    let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY start_date DESC")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    // Group projects by year
    let mut years_map: std::collections::BTreeMap<i32, Vec<Project>> = std::collections::BTreeMap::new();
    for project in projects {
        let year = project.start_date.year();
        years_map.entry(year).or_insert_with(Vec::new).push(project);
    }

    // Convert to vec of (year, projects) tuples, sorted by year descending
    let mut grouped_projects: Vec<(i32, Vec<Project>)> = years_map
        .into_iter()
        .collect();
    grouped_projects.reverse(); // Most recent year first

    let footer = get_footer(pool).await;

    IndexTemplate { grouped_projects, footer }
}

pub async fn index(State(pool): State<PgPool>) -> impl IntoResponse {
    get_index_template(&pool).await
}

pub async fn get_project_details_template(pool: &PgPool, slug: &str) -> Option<ProjectTemplate> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE slug = $1")
        .bind(slug)
        .fetch_one(pool)
        .await
        .ok()?;

    let blocks = sqlx::query_as::<_, ContentBlock>(
        "SELECT * FROM content_blocks WHERE project_id = $1 ORDER BY sort_order ASC"
    )
    .bind(project.id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    // Get next project (older date)
    let next_project = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE start_date < $1 ORDER BY start_date DESC LIMIT 1"
    )
    .bind(project.start_date)
    .fetch_optional(pool)
    .await
    .unwrap_or(None);

    // Get previous project (newer date)
    let prev_project = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE start_date > $1 ORDER BY start_date ASC LIMIT 1"
    )
    .bind(project.start_date)
    .fetch_optional(pool)
    .await
    .unwrap_or(None);

    let footer = get_footer(pool).await;

    Some(ProjectTemplate { 
        project, 
        blocks,
        next_project,
        prev_project,
        footer,
    })
}

pub async fn project_details(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    ) -> impl IntoResponse {
    if let Some(template) = get_project_details_template(&pool, &slug).await {
        template.into_response()
    } else {
        // TODO: 404 Page
        "Project not found".into_response()
    }
}

pub async fn get_contact_template(pool: &PgPool) -> ContactTemplate {
    let page = sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = 'contact'")
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
        .unwrap_or(Page {
            slug: "contact".to_string(),
            title: "Contact".to_string(),
            content: "<p>Contact info missing.</p>".to_string(),
            updated_at: time::OffsetDateTime::now_utc(),
        });

    let footer = get_footer(pool).await;

    ContactTemplate { page, footer }
}

pub async fn contact(State(pool): State<PgPool>) -> impl IntoResponse {
    get_contact_template(&pool).await
}

pub async fn get_about_template(pool: &PgPool) -> AboutTemplate {
     let page = sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = 'about'")
        .fetch_optional(pool)
        .await
        .unwrap_or(None)
        .unwrap_or(Page {
            slug: "about".to_string(),
            title: "About".to_string(),
            content: "<p>About info missing.</p>".to_string(),
            updated_at: time::OffsetDateTime::now_utc(),
        });

    let footer = get_footer(pool).await;

    AboutTemplate { page, footer }
}

pub async fn about(State(pool): State<PgPool>) -> impl IntoResponse {
    get_about_template(&pool).await
}
