CREATE TABLE IF NOT EXISTS evidence_chain_receipts (
    receipt_id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phase           TEXT NOT NULL,
    source_system   TEXT NOT NULL,
    merkle_root     TEXT NOT NULL,
    signature       BYTEA NOT NULL,
    scitt_receipt   TEXT,
    created_at      TIMESTAMPTZ DEFAULT now()
);
