# Module 05: Admin Panel & Authentication

We need a secure way to manage our content.

## 1. Authentication Strategy
We don't use JWTs (they are for APIs). We use **Sessions** (`tower-sessions`).
*   When you log in, the server sets a secure, HTTP-only Cookie.
*   The server remembers "Session ID 123 = User ID 1".

## 2. Password Hashing
NEVER store plain text passwords. We use `argon2`.

```rust
// Verify Password
pub fn verify_password(password: &str, hash: &str) -> bool {
    // Argon2 verifies the hash...
}
```

## 3. Protecting Routes
In `src/lib.rs`, we apply a middleware to the `/admin` scope.
If a user tries to access `/admin/dashboard` without a session, the middleware intercepts them and redirects to `/admin/login`.

## 4. The Admin Dashboard
The dashboard (`src/routes/admin.rs`) is where the magic happens.
*   **Projects List**: Shows all projects.
*   **Create Project**: A detailed form.
*   **Deploy Button**: This is special.

### The Deploy Button
When you click "Deploy Live", we trigger a **GitHub Action**.
```javascript
// A simple POST request
form action="/admin/deploy" method="POST"
```
The server receives this `POST`, uses the GitHub API, and says "Hey GitHub, run the 'deploy' workflow!".
