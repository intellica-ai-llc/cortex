CREATE TABLE IF NOT EXISTS base_snapshot_config (
    source          TEXT PRIMARY KEY,
    table_name      TEXT NOT NULL,
    merge_interval_min INTEGER DEFAULT 15,
    merge_strategy  TEXT DEFAULT 'merge_on_read',
    last_merge_at   TIMESTAMPTZ,
    row_count       BIGINT DEFAULT 0,
    UNIQUE(source, table_name)
);
