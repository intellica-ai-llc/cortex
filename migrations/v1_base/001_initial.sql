-- Cortex v1 base schema: tools, provenance capsules, agent states
CREATE TABLE IF NOT EXISTS tools (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    description     TEXT NOT NULL,
    description_embedding VECTOR(1536),
    input_schema    JSONB NOT NULL,
    output_schema   JSONB,
    connector_id    UUID,
    plan_required   TEXT DEFAULT 'free',
    rate_limit_rpm  INTEGER DEFAULT 60,
    is_active       BOOLEAN DEFAULT true,
    tool_hash       TEXT NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_tools_embedding ON tools USING ivfflat(description_embedding);
CREATE INDEX IF NOT EXISTS idx_tools_connector ON tools(connector_id);
CREATE INDEX IF NOT EXISTS idx_tools_hash ON tools(tool_hash);

CREATE TABLE IF NOT EXISTS provenance_capsules (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id        TEXT NOT NULL,
    action_kind     TEXT NOT NULL,
    intent_text     TEXT,
    tool_name       TEXT,
    input_hash      TEXT NOT NULL,
    output_hash     TEXT,
    risk_score      FLOAT NOT NULL DEFAULT 0.0,
    parent_ids      UUID[],
    merkle_hash     TEXT NOT NULL,
    signature       BYTEA NOT NULL,
    vap_level       TEXT,
    scitt_receipt   TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_provenance_agent ON provenance_capsules(agent_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_provenance_merkle ON provenance_capsules(merkle_hash);
