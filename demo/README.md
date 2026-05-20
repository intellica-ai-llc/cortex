# Cortex MLP Demo — Run in 60 Seconds

## Quick Start
```bash
cd demo
docker compose up -d
Dashboard: http://localhost:8787/admin
Health: http://localhost:8787/health
MCP Gateway: POST http://localhost:8787/mcp

Requirements
Docker Desktop or Docker Engine 24+

4 GB RAM available

No cloud access required (fully local)

What's Inside
Cortex binary (sovereign MCP gateway)

PostgreSQL 16 with pgvector (TraceDB)

Demo seeder (90 work orders, 20 assets, 15 PMs, 10 employees, 500 provenance capsules)

Demo license (all features enabled, limited to localhost)

Five-Act Self-Guided Journey
Open story/self-guided.html or visit http://localhost:8787/admin.

Reset Demo Data
bash
./tools/reset-demo.sh
Statistics
bash
./tools/demo-stats.sh
