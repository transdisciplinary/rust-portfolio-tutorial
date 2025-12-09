# Artist Portfolio

A high-performance, cost-effective, and aesthetically pleasing artist portfolio website built with Rust.

## 🚀 Key Features

*   **Scale-to-Zero Architecture**: Backend (Render) sleeps when idle to minimize costs.
*   **Static Frontend**: Public site (Cloudflare Pages) is always online and extremely fast.
*   **Rust Backend**: Robust Admin CMS built with Axum & SQLx.
*   **Client-Side Optimization**: Images are compressed/converted to WebP in the browser before upload.
*   **No-Framework UI**: Vanilla CSS and HTMX for a lightweight, responsive feel.

## 🛠 Tech Stack

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Backend** | [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum) | Fast, safe web server. |
| **Database** | [PostgreSQL (Neon)](https://neon.tech/) | Serverless database. |
| **Templating** | [Askama](https://github.com/djc/askama) | Type-safe HTML templates. |
| **Hosting (Back)** | [Render](https://render.com/) | Hosting for the Admin CMS. |
| **Hosting (Front)** | [Cloudflare Pages](https://pages.cloudflare.com/) | Static site hosting. |

## 📚 Documentation

Detailed documentation is available in the `tutorial/` folder:

*   [**Architecture**](tutorial/architecture.md): Diagrams of the system flow.
*   [**Backend Guide**](tutorial/backend.md): Rust code explanation.
*   [**Frontend Guide**](tutorial/frontend.md): HTMX & CSS architecture.
*   [**Deployment Guide**](tutorial/deployment.md): Steps to deploy to Render/Cloudflare.

## 🏃‍♂️ Quick Start

1.  **Prerequisites**: Install Rust, Cargo, and Docker.
2.  **Environment**: Copy `Secrets.toml` (if using locally) or set env vars.
3.  **Run Server (Admin Panel)**:
    ```sh
    cargo run
    ```
    Visits `http://localhost:8000`.

4.  **Generate Static Site**:
    ```sh
    cargo run --bin gen_static
    ```
    This creates a `dist/` folder with the full static website.

## 🛡️ Security

*   Passwords hashed with **Argon2**.
*   Secure session management via `tower-sessions`.
*   Admin routes protected by middleware.
