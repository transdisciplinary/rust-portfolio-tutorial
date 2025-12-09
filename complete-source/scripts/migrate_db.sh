#!/bin/bash
set -e

# Usage: ./scripts/migrate_db.sh "POSTGRES_SOURCE_URL" "POSTGRES_DEST_URL"

SOURCE_DB="$1"
DEST_DB="$2"

if [ -z "$SOURCE_DB" ] || [ -z "$DEST_DB" ]; then
    echo "Usage: ./scripts/migrate_db.sh \"<source_db_url>\" \"<dest_db_url>\""
    exit 1
fi


if command -v docker &> /dev/null; then
    echo " Docker found! Using Postgres v17 container to avoid version mismatch..."
    
    echo " Dumping data from source (Shuttle)..."
    docker run --rm -i postgres:17-alpine pg_dump "$SOURCE_DB" --no-owner --no-acl --clean --if-exists > dump.sql

    echo " Restoring data to destination (Neon)..."
    # We pipe the file content into the docker container
    docker run --rm -i postgres:17-alpine psql "$DEST_DB" < dump.sql

else
    echo " Docker not found. Using local tools (may fail if versions mismatch)..."
    
    echo " Dumping data from source..."
    pg_dump "$SOURCE_DB" --no-owner --no-acl --clean --if-exists > dump.sql

    echo " Restoring data to destination..."
    psql "$DEST_DB" < dump.sql
fi

echo " Migration complete!"
rm -f dump.sql

