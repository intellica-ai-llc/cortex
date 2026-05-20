CREATE TABLE IF NOT EXISTS mirror_sync_state (
    source              TEXT PRIMARY KEY,
    sync_mode           TEXT DEFAULT 'streaming',
    current_backpressure INTEGER DEFAULT 0,
    backpressure_sustained_s INTEGER DEFAULT 0,
    event_rate_per_sec  INTEGER DEFAULT 0,
    last_checksum_at    TIMESTAMPTZ,
    last_checksum_match_rate FLOAT,
    pending_schema_changes JSONB DEFAULT '[]',
    frozen_schema_version TEXT,
    compaction_debt_gb  FLOAT DEFAULT 0,
    total_rows_mirrored BIGINT DEFAULT 0,
    rows_behind         BIGINT DEFAULT 0,
    freshness_status    TEXT GENERATED ALWAYS AS (
        CASE
            WHEN current_backpressure > 0 THEN 'micro_batch'
            WHEN sync_latency_ms <= 100 THEN 'live'
            WHEN sync_latency_ms <= 5000 THEN 'near-real-time'
            WHEN sync_latency_ms <= 300000 THEN 'delayed'
            ELSE 'stale'
        END
    ) STORED,
    sync_latency_ms     INTEGER DEFAULT 0
);
