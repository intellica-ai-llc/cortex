CREATE TABLE IF NOT EXISTS tool_versions (
    tool_id     TEXT PRIMARY KEY,
    major       INTEGER NOT NULL,
    minor       INTEGER NOT NULL,
    patch       INTEGER NOT NULL,
    last_checked TIMESTAMPTZ DEFAULT now(),
    drift_detected BOOLEAN DEFAULT FALSE
);
