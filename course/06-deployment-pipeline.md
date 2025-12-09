# Module 06: The Hybrid Deployment Pipeline

This is how we achieve the "Scale-to-Zero" magic.

## 1. Deploy the Backend (Render)
1.  Connect your repo to Render.
2.  Set `DATABASE_URL` (from Neon).
3.  Deploy.
Now you have a dynamic Admin Panel. But it sleeps after 15 minutes.

## 2. The Static Generator (`gen_static.rs`)
This is a Rust binary we wrote (`src/bin/gen_static.rs`).
*   It runs locally (or in CI).
*   It fetches ALL data from the database.
*   It uses `Askama` to generate HTML files for every page.
*   It saves them to `dist/`.

## 3. The CI/CD Pipeline (GitHub Actions)
Our workflow (`.github/workflows/deploy.yml`) automates this.

**The Trigger**:
*   Push to `main`.
*   OR "Repository Dispatch" (Clicking Deploy in Admin).

**The Steps**:
1.  **Checkout Code**.
2.  **Build Generator**: `cargo run --bin gen_static`.
3.  **Deploy to pages**: It takes the `dist/` folder and pushes it to the `gh-pages` branch.

## 4. The Frontend (Cloudflare Pages)
Cloudflare watches the `gh-pages` branch.
When GitHub Actions pushes new HTML there, Cloudflare updates the global CDN.

**Result**:
*   Your site is fast.
*   Your site is free (mostly).
*   Your admin panel is secure.

**Congratulations! You have built a production-grade, scale-to-zero Rust portfolio.**
