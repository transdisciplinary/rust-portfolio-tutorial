# Module 02: Setup & Installation

Let's get your development environment ready. You can follow along with the code in `../complete-source/`.

## 1. Install Rust
If you haven't already, install Rust using `rustup`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Install Docker
We need Docker to run our local PostgreSQL database easily.
*   [Download Docker Desktop](https://www.docker.com/products/docker-desktop/)

## 3. Database Setup
We use `docker-compose` to spin up a local database.

Create a `docker-compose.yml` (or run manually):
```yaml
version: '3.8'
services:
  db:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: portfolio
    ports:
      - "5432:5432"
```
Run it: `docker-compose up -d`

## 4. Install SQLx CLI
This tool handles our database migrations.
```bash
cargo install sqlx-cli
```

## 5. Environment Config
Create a `.env` file or `Secrets.toml` (if using Shuttle, but we are using standard Envs now).
```env
DATABASE_URL=postgres://postgres:password@localhost:5432/portfolio
CLOUDINARY_CLOUD_NAME=your_name
CLOUDINARY_API_KEY=your_key
CLOUDINARY_API_SECRET=your_secret
```

## 6. Run Migrations
Apply the schema to your database:
```bash
sqlx migrate run
# Or if you are using the complete-source, run the script:
./scripts/migrate_db.sh
```

Now you are ready to write some Rust!
