# Database Migration & Backup Strategy

## 1. Migrating Data from Shuttle to Aiven

Since Shuttle uses a shared Postgres database, you can export your data using `pg_dump`.

### Prerequisites
- `pg_dump` installed locally (usually part of PostgreSQL tools).
- Connection string for your Shuttle database (get it from Shuttle dashboard or `Secrets.toml` if you saved it).
- Connection string for your new Aiven database.

### Steps

1.  **Export Data from Shuttle**
    ```bash
    pg_dump "postgres://<shuttle_user>:<shuttle_pass>@pg.shuttle.rs/<shuttle_db>" > backup.sql
    ```

2.  **Import Data to Aiven**
    ```bash
    psql "postgres://<aiven_user>:<aiven_pass>@<aiven_host>:<port>/<aiven_db>?sslmode=require" < backup.sql
    ```

## 2. Daily Backup: Aiven to Neon

To ensure you have a daily copy of your Aiven database on Neon, you can use a GitHub Action.

### GitHub Action Setup

1.  Create a new file `.github/workflows/db-backup.yml`.
2.  Add the following content:

```yaml
name: Database Backup Sync

on:
  schedule:
    - cron: '0 3 * * *' # Run at 3 AM UTC daily
  workflow_dispatch: # Allow manual trigger

jobs:
  backup-and-restore:
    runs-on: ubuntu-latest
    steps:
      - name: Install PostgreSQL Client
        run: |
          sudo apt-get update
          sudo apt-get install -y postgresql-client

      - name: Backup Aiven Database
        env:
          AIVEN_DB_URL: ${{ secrets.AIVEN_DB_URL }}
        run: |
          pg_dump "$AIVEN_DB_URL" --clean --if-exists --no-owner --no-privileges > backup.sql

      - name: Restore to Neon Database
        env:
          NEON_DB_URL: ${{ secrets.NEON_DB_URL }}
        run: |
          psql "$NEON_DB_URL" < backup.sql
```

3.  **Configure Secrets in GitHub**:
    - Go to your repository Settings -> Secrets and variables -> Actions.
    - Add `AIVEN_DB_URL`: Your Aiven connection string.
    - Add `NEON_DB_URL`: Your Neon connection string.

This action will dump the Aiven database and restore it to Neon every day.
