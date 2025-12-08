# ⚙️ Backend Guide (Rust + Axum)

The backend is the "brain" of the Admin Panel. It handles authentication, data storage, and image uploads.

## Project Structure

```bash
src/
├── bin/
│   ├── gen_static.rs  # The Static Site Generator
│   ├── standalone.rs  # Local dev entry point
├── routes/
│   ├── admin.rs       # Admin CRUD logic
│   ├── auth.rs        # Login/Logout logic
│   ├── public.rs      # Read-only routes
├── models.rs          # Database structs (SQLx)
├── lib.rs             # Core application setup
```

## 1. The Web Server (Axum)

We use **Axum** because it's fast and ergonomic. Routes are defined in `src/lib.rs`:

```rust
// create_router function
Router::new()
    .route("/", get(routes::public::index))
    // Protected Admin Routes
    .nest("/admin", admin_routes())
    .route_layer(middleware::from_fn(auth_middleware))
```

### Middleware
We use middleware to protect the admin panel. If a user isn't logged in, they are redirected to `/admin/login`.

## 2. Database Access (SQLx)

We use **SQLx** to talk to Postgres. It provides compile-time checked queries, meaning if you write bad SQL, the code won't compile!

```rust
// Example: Fetching projects
let projects = sqlx::query_as::<_, Project>("SELECT * FROM projects")
    .fetch_all(&pool)
    .await?;
```

## 3. Image Uploads

We handle multipart uploads in `src/routes/api.rs`.
*   Images are received as bytes.
*   They are sent to **Cloudinary** using their API.
*   The returned URL is saved to the database.

> **Note**: We use client-side optimization (in the frontend) so the server receives smaller, pre-compressed WebP files.

## 4. Static Site Generator (`gen_static.rs`)

This is a special binary. Instead of running a server, it:
1.  Connects to the DB.
2.  Fetches all projects.
3.  "Renders" the Askama templates to strings.
4.  Writes those strings to `.html` files in a `dist/` folder.
