CREATE TABLE IF NOT EXISTS cdc_append_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source          TEXT NOT NULL,
    table_name      TEXT NOT NULL,
    operation       TEXT NOT NULL,
    primary_key     TEXT NOT NULL,
    old_values      JSONB,
    new_values      JSONB,
    transaction_id  TEXT NOT NULL,
    lsn             TEXT,
    ingested_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_cdc_source ON cdc_append_log(source);
CREATE INDEX IF NOT EXISTS idx_cdc_table_time ON cdc_append_log(table_name, ingested_at);
