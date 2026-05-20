#!/bin/bash
set -e
cd "$(dirname "$0")/.."
echo "Resetting Cortex demo..."
docker compose down -v
docker compose up -d db
echo "Waiting for PostgreSQL..."
until docker compose exec -T db pg_isready -U cortex -d cortex_demo 2>/dev/null; do sleep 1; done
docker compose run --rm seeder
docker compose up -d cortex
echo "Demo reset complete. Dashboard at http://localhost:8787/admin"
