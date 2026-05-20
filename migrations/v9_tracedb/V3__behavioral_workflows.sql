CREATE TABLE IF NOT EXISTS behavioral_workflows (
    workflow_id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL,
    source_application  TEXT NOT NULL,
    behavioral_tokens   TEXT[] NOT NULL,
    token_count         INTEGER GENERATED ALWAYS AS (array_length(behavioral_tokens, 1)) STORED,
    workflow_graph      JSONB,
    frequency           INTEGER DEFAULT 1,
    total_duration_ms   BIGINT,
    converted_to_skill  BOOLEAN DEFAULT FALSE,
    skill_id            UUID,
    absorption_phase    TEXT DEFAULT 'observing',
    first_observed      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_observed       TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_bw_user_app ON behavioral_workflows(user_id, source_application);
CREATE INDEX IF NOT EXISTS idx_bw_tokens ON behavioral_workflows USING gin(behavioral_tokens);
CREATE INDEX IF NOT EXISTS idx_bw_frequency ON behavioral_workflows(frequency DESC);
