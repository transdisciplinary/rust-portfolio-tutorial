use artist_portfolio::{create_router, AppState};
use artist_portfolio::upload::CloudinaryConfig;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let cloudinary_config = CloudinaryConfig::new(
        env::var("CLOUDINARY_CLOUD_NAME").expect("CLOUDINARY_CLOUD_NAME must be set"),
        env::var("CLOUDINARY_API_KEY").expect("CLOUDINARY_API_KEY must be set"),
        env::var("CLOUDINARY_API_SECRET").expect("CLOUDINARY_API_SECRET must be set"),
    );

    let state = AppState {
        pool,
        cloudinary: cloudinary_config,
    };

    let is_production = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()) == "production";
    
    let app = create_router(state, is_production);

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string()).parse::<u16>().unwrap_or(8000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", addr);
    
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
