# Rust Scale-to-Zero Artist Portfolio


Welcome to the **Rust Scale-to-Zero Artist Portfolio** tutorial project!

This repository demonstrates how to build a high-performance, cost-effective, and aesthetically pleasing artist portfolio using **Rust**, **HTMX**, and modern cloud infrastructure.

## 🚀 Key Concepts

*   **Scale-to-Zero**: The backend (hosted on Render) spins down when not in use, saving money.
*   **Static Frontend**: The public-facing site is statically generated HTML (hosted on Cloudflare Pages), ensuring instant load times and 100% uptime even if the backend is asleep.
*   **Rust Backend**: A robust CMS built with Rust for managing content securely.
*   **No-Framework Interface**: A dynamic UI built with vanilla CSS and HTMX, avoiding heavy JavaScript frameworks like React.

## 🛠️ Tech Stack

| Component | Technology | Description |
| :--- | :--- | :--- |
| **Backend Language** | [Rust](https://www.rust-lang.org/) | Memory-safe, fast system programming language. |
| **Web Framework** | [Axum](https://github.com/tokio-rs/axum) | Ergonomic and modular web application framework. |
| **Database** | [PostgreSQL (Neon)](https://neon.tech/) | Serverless Postgres database. |
| **ORM** | [SQLx](https://github.com/launchbadge/sqlx) | Async, pure Rust SQL crate with compile-time checked queries. |
| **Templating** | [Askama](https://github.com/djc/askama) | Type-safe, compiled Jinja-like templates. |
| **Frontend Interactivity** | [HTMX](https://htmx.org/) | High power tools for HTML. |
| **Hosting (Backend)** | [Render](https://render.com/) | Cloud application hosting. |
| **Hosting (Frontend)** | [Cloudflare Pages](https://pages.cloudflare.com/) | Static site hosting (free). |
| **Image Hosting** | [Cloudinary](https://cloudinary.com/) | Media management and optimization. |

## 📚 Tutorial Modules

Navigate through the files in this directory to understand how the project is built:

1.  [**Architecture**](./architecture.md): Visualizing the "Scale-to-Zero" and Hybrid deployment flow.
2.  [**Backend Guide**](./backend.md): Deep dive into the Rust/Axum code structure.
3.  [**Frontend Guide**](./frontend.md): How Askama templates and HTMX work together.
4.  [**Deployment Guide**](./deployment.md): Steps to deploy correctly to Render and Cloudflare.

## 🏃‍♂️ Quick Start (Local Development)

1.  **Prerequisites**: Install Rust, Cargo, and Docker.
2.  **Environment**: Copy `.env.example` to `.env` and fill in DB/Cloudinary credentials.
3.  **Run DB**: `docker compose up -d` (if using local DB).
4.  **Run App**: `cargo run`
5.  **Visit**: `http://localhost:8000`
