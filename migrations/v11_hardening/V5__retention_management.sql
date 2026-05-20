CREATE TABLE IF NOT EXISTS retention_policies (
    policy_id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_name     TEXT NOT NULL,
    target_table    TEXT NOT NULL,
    retention_days  INTEGER NOT NULL,
    legal_hold      BOOLEAN DEFAULT FALSE,
    hold_reason     TEXT,
    created_at      TIMESTAMPTZ DEFAULT now()
);
