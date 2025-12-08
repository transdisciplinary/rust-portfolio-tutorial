# 🚀 Deployment Guide


This project handles two types of deployment: **Dynamic** (Backend) and **Static** (Frontend).

## 1. Hosting Provider Setup

### A. Neon (Database)
1.  Create a Postgres project on neon.tech.
2.  Get the **Connection String** (Pooled).
3.  Save it as `DATABASE_URL`.

### B. Render (Backend)
1.  Create a "Web Service" connected to your GitHub repo.
2.  **Runtime**: Rust.
3.  **Build Command**: `cargo build --release` (or use the Dockerfile).
4.  **Environment Variables**:
    *   `DATABASE_URL`: (From Neon)
    *   `CLOUDINARY_*`: (Your keys)
    *   `APP_ENVIRONMENT`: `production`

### C. Cloudflare Pages (Frontend)
1.  Connect to your GitHub repo -> `gh-pages` branch.
2.  **Custom Domain**: Set up your domain (e.g., `stefmeul.net`).
3.  **Redirect Rule**:
    *   Since the main domain serves static files, `/admin` requests would 404.
    *   Create a Rule: If path starts with `/admin`, **Dynamic Redirect** to your Render URL.

## 2. GitHub Actions (The Glue)

The file `.github/workflows/deploy-static.yml` automates the "Scale-to-Zero" logic.

### Trigger
It runs when:
1.  You push to `main` (optional).
2.  **Repository Dispatch**: You click "Deploy Live" in the Admin Panel.

### Work Flow
1.  **Checkout Code**.
2.  **Install Rust**.
3.  **Run Generator**: `cargo run --bin gen_static`.
    *   This fetches data from Neon.
    *   Generates HTML files in `dist/`.
4.  **Deploy**: Pushes the `dist/` folder to the `gh-pages` branch.

Cloudflare sees the update to `gh-pages` and instantly publishes the new site!
