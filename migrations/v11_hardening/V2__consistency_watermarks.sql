CREATE TABLE IF NOT EXISTS consistency_watermarks (
    source          TEXT PRIMARY KEY,
    latest_lsn      TEXT,
    latest_timestamp TIMESTAMPTZ,
    updated_at      TIMESTAMPTZ DEFAULT now()
);
