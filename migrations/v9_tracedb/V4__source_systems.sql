CREATE TABLE IF NOT EXISTS source_systems (
    system_id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_name         TEXT NOT NULL,
    system_type         TEXT NOT NULL,
    vendor              TEXT,
    db_connection_string TEXT,
    mcp_connector_name   TEXT,
    total_tables         INTEGER,
    total_columns        INTEGER,
    fields_discovered    INTEGER DEFAULT 0,
    fields_absorbed      INTEGER DEFAULT 0,
    absorption_pct       FLOAT GENERATED ALWAYS AS (
        CASE WHEN total_columns > 0 THEN (fields_absorbed::FLOAT / total_columns::FLOAT) * 100 ELSE 0 END
    ) STORED,
    absorption_phase     TEXT DEFAULT 'observing',
    projected_retirement_date DATE,
    actual_retirement_date    DATE,
    license_cost_annual  DECIMAL(12,2),
    license_savings_ytd  DECIMAL(12,2) DEFAULT 0,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_ss_phase ON source_systems(absorption_phase);
