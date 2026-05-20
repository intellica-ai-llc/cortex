CREATE TABLE IF NOT EXISTS tenant_registry (
    tenant_id       TEXT PRIMARY KEY,
    database_url    TEXT NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT now(),
    is_active       BOOLEAN DEFAULT TRUE
);
