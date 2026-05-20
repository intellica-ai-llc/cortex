CREATE TABLE IF NOT EXISTS decision_traces (
    trace_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL,
    session_id          UUID NOT NULL,
    agent_id            UUID,
    timestamp           TIMESTAMPTZ NOT NULL DEFAULT now(),
    intent              TEXT NOT NULL,
    observation         JSONB NOT NULL,
    inference           JSONB,
    evidence_chain      JSONB,
    decision_type       TEXT NOT NULL,
    actor_type          TEXT NOT NULL,
    governance_tier     TEXT DEFAULT 'full',
    policy_version      TEXT,
    behavioral_token    TEXT NOT NULL,
    source_application  TEXT NOT NULL,
    source_schema_ref   UUID,
    source_value_before JSONB,
    source_value_after  JSONB,
    plan_version        INTEGER DEFAULT 1,
    revision_rationale  TEXT,
    confidence_score    FLOAT CHECK (confidence_score >= 0 AND confidence_score <= 1),
    delegation_chain    JSONB,
    verdict             JSONB,
    parent_trace_ids    UUID[],
    child_trace_ids     UUID[],
    content_hash        TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_dt_user_time ON decision_traces(user_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_dt_behavioral_token ON decision_traces(behavioral_token);
CREATE INDEX IF NOT EXISTS idx_dt_source_app ON decision_traces(source_application);
CREATE INDEX IF NOT EXISTS idx_dt_parent_traces ON decision_traces USING gin(parent_trace_ids);
CREATE INDEX IF NOT EXISTS idx_dt_intent_fts ON decision_traces USING gin(to_tsvector('english', intent));
