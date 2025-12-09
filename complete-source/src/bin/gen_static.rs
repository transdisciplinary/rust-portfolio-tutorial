use artist_portfolio::routes::public;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use tokio::fs;
use askama::Template;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env
    dotenvy::dotenv().ok();
    
    // Connect DB
    // Try to get DATABASE_URL from env, otherwise try to read from Secrets.toml as a fallback (simple parsing)
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("DATABASE_URL not set, trying to read Secrets.toml...");
            let secrets = fs::read_to_string("Secrets.toml").await?;
            let mut found_url = None;
            for line in secrets.lines() {
                if let Some(rest) = line.trim().strip_prefix("DATABASE_URL=") {
                    found_url = Some(rest.trim_matches('"').to_string());
                    break;
                }
                if let Some(rest) = line.trim().strip_prefix("DATABASE_URL =") {
                    found_url = Some(rest.trim().trim_matches('"').to_string());
                    break;
                }
            }
            found_url.expect("Could not find DATABASE_URL in env or Secrets.toml")
        }
    };

    println!("Connecting to database...");
    let pool = PgPoolOptions::new().connect(&db_url).await?;

    // Create dist dir
    let dist = Path::new("dist");
    if dist.exists() {
        println!("Cleaning dist directory...");
        fs::remove_dir_all(dist).await?;
    }
    fs::create_dir_all(dist).await?;
    
    // Static assets
    println!("Copying static assets...");
    if Path::new("static").exists() {
        copy_dir("static", dist.join("static"))?;
    } else {
        println!("Warning: 'static' directory not found.");
    }

    // Index
    println!("Generating Index...");
    let index_tmpl = public::get_index_template(&pool).await;
    write_file(dist.join("index.html"), index_tmpl.render()?).await?;

    // Projects
    println!("Generating Project pages...");
    for (_year, projects) in index_tmpl.grouped_projects.iter() {
        for project in projects {
            println!("  Generating project: {}", project.slug);
            let tmpl = public::get_project_details_template(&pool, &project.slug).await;
            if let Some(t) = tmpl {
               let p_dir = dist.join("project").join(&project.slug);
               fs::create_dir_all(&p_dir).await?;
               write_file(p_dir.join("index.html"), t.render()?).await?;
            }
        }
    }

    // About
    println!("Generating About page...");
    let about = public::get_about_template(&pool).await;
    let about_dir = dist.join("about");
    fs::create_dir_all(&about_dir).await?;
    write_file(about_dir.join("index.html"), about.render()?).await?;

    // Contact
    println!("Generating Contact page...");
    let contact = public::get_contact_template(&pool).await;
    let contact_dir = dist.join("contact");
    fs::create_dir_all(&contact_dir).await?;
    write_file(contact_dir.join("index.html"), contact.render()?).await?;


    // Admin Redirect
    println!("Generating Admin Redirect...");
    let admin_url = env::var("ADMIN_URL").unwrap_or_else(|_| "https://artist-portfolio.shuttleapp.rs/admin".to_string());
    let admin_dir = dist.join("admin");
    fs::create_dir_all(&admin_dir).await?;
    let redirect_html = format!(r#"<!DOCTYPE html>
<html>
<head>
<meta http-equiv="refresh" content="0; url={}/login">
<title>Redirecting to Admin...</title>
</head>
<body>
<p>Redirecting to <a href="{}/login">Admin Dashboard</a>...</p>
<script>window.location.href = "{}/login";</script>
</body>
</html>"#, admin_url, admin_url, admin_url);
    write_file(admin_dir.join("index.html"), redirect_html).await?;

    println!("Static site generated successfully in ./dist");
    Ok(())
}

async fn write_file(path: impl AsRef<Path>, content: String) -> std::io::Result<()> {
    fs::write(path, content).await
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
