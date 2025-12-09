# Module 03: Building the Backend

The heart of our system is the Rust backend. It serves the Admin Panel and handles the API logic.

## 1. Project Structure
In `Cargo.toml`, we define our dependencies: `axum`, `sqlx`, `tokio`, `askama`.

## 2. The Application State
We need to share our Database Pool across all routes.
In `src/lib.rs`:
```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
```

## 3. The Router
This is where we wire everything up.
```rust
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Public Routes (if we had a dynamic frontend)
        .route("/", get(routes::public::index))
        
        // Admin Routes (Protected)
        .nest("/admin", admin_routes())
        
        // Static Files (CSS/JS)
        .nest_service("/static", ServeDir::new("static"))
        
        .with_state(state)
}
```

## 4. Admin CRUD
Let's look at `src/routes/admin.rs`.
We use **Handlers** to process requests.
```rust
pub async fn dashboard(State(pool): State<PgPool>) -> impl IntoResponse {
    // 1. Fetch data from DB
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(&pool).await.unwrap();
        
    // 2. Render Template
    DashboardTemplate { projects }
}
```
Notice how `sqlx` checks our SQL syntax at compile time!

## 5. Handling Uploads
In `src/routes/api.rs`, we handle file uploads.
*   We use `axum::extract::Multipart` to stream the file.
*   We send the bytes to Cloudinary.
*   We save the resulting URL to our database context.

In the next module, we'll see how we render this data to the user.
