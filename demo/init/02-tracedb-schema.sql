-- TraceDB core tables (compact for demo)
CREATE TABLE IF NOT EXISTS tools (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    description     TEXT NOT NULL,
    description_embedding VECTOR(1536),
    input_schema    JSONB NOT NULL,
    output_schema   JSONB,
    plan_required   TEXT DEFAULT 'free',
    rate_limit_rpm  INTEGER DEFAULT 60,
    is_active       BOOLEAN DEFAULT true,
    tool_hash       TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS decision_traces (
    trace_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL,
    session_id          UUID NOT NULL DEFAULT gen_random_uuid(),
    agent_id            UUID,
    timestamp           TIMESTAMPTZ NOT NULL DEFAULT now(),
    intent              TEXT NOT NULL,
    observation         JSONB NOT NULL DEFAULT '{}',
    inference           JSONB,
    evidence_chain      JSONB,
    decision_type       TEXT NOT NULL DEFAULT 'ToolCall',
    actor_type          TEXT NOT NULL DEFAULT 'agent',
    behavioral_token    TEXT NOT NULL DEFAULT 'QUERY_Database',
    source_application  TEXT NOT NULL DEFAULT 'maximo',
    source_value_before JSONB,
    source_value_after  JSONB,
    plan_version        INTEGER DEFAULT 1,
    confidence_score    FLOAT CHECK (confidence_score >= 0 AND confidence_score <= 1),
    parent_trace_ids    UUID[],
    merkle_hash         TEXT,
    signature           BYTEA,
    scitt_receipt       TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS absorbed_fields (
    field_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_application  TEXT NOT NULL,
    source_table        TEXT NOT NULL,
    source_column       TEXT NOT NULL,
    semantic_label      TEXT,
    field_type          TEXT NOT NULL DEFAULT 'TEXT',
    observation_count   INTEGER DEFAULT 0,
    absorption_status   TEXT DEFAULT 'observing',
    UNIQUE(source_application, source_table, source_column)
);

CREATE TABLE IF NOT EXISTS source_systems (
    system_id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_name         TEXT NOT NULL,
    system_type         TEXT NOT NULL,
    vendor              TEXT,
    fields_discovered   INTEGER DEFAULT 0,
    fields_absorbed     INTEGER DEFAULT 0,
    absorption_phase    TEXT DEFAULT 'observing',
    license_cost_annual DECIMAL(12,2) DEFAULT 0
);

CREATE TABLE IF NOT EXISTS behavioral_workflows (
    workflow_id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL,
    source_application  TEXT NOT NULL,
    behavioral_tokens   TEXT[] NOT NULL,
    frequency           INTEGER DEFAULT 1,
    converted_to_skill  BOOLEAN DEFAULT FALSE
);
