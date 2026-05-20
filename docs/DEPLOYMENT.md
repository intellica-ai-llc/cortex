# Cortex Deployment Guide
## Prerequisites
- Linux server (Ubuntu 22.04+ / RHEL 9+) with 2+ CPU cores, 4+ GB RAM, 20+ GB disk.
- PostgreSQL 15+ with pgvector extension.
## Quick Start
```bash
curl -fsSL https://install.intellica.io | bash
cortex init --license <key>
cortex serve
