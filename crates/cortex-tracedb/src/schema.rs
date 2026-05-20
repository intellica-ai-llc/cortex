use sqlx::PgPool;
use tracing::info;

/// Run all TraceDB migrations (idempotent).
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Running TraceDB migrations...");

    sqlx::query(CREATE_DECISION_TRACES)
        .execute(pool)
        .await?;

    sqlx::query(CREATE_ABSORBED_FIELDS)
        .execute(pool)
        .await?;

    sqlx::query(CREATE_BEHAVIORAL_WORKFLOWS)
        .execute(pool)
        .await?;

    // Create indexes (independent statements; IF NOT EXISTS handled by PostgreSQL)
    for idx in INDEXES.iter() {
        sqlx::query(idx).execute(pool).await?;
    }

    info!("TraceDB migrations complete");
    Ok(())
}

// ── 1. DECISION TRACES (AER + DES compliant) ──
pub const CREATE_DECISION_TRACES: &str = r#"
CREATE TABLE IF NOT EXISTS decision_traces (
    trace_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL,
    session_id          UUID NOT NULL,
    agent_id            UUID,
    timestamp           TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- AER Core Fields (intent, observation, inference, evidence chain)
    intent              TEXT NOT NULL,
    observation         JSONB NOT NULL,
    inference           JSONB,
    evidence_chain      JSONB,

    -- DES Compliance (Decision Event Schema)
    decision_type       TEXT NOT NULL,
    actor_type          TEXT NOT NULL,              -- 'human', 'agent', 'hybrid'
    governance_tier     TEXT DEFAULT 'full',        -- 'lightweight', 'sampled', 'full'
    policy_version      TEXT,

    -- Behavioral Abstraction (From Logs to Agents)
    behavioral_token    TEXT NOT NULL,              -- MODIFY_Field, SUBMIT_Form, QUERY_Database, etc.
    source_application  TEXT NOT NULL,
    source_schema_ref   UUID,                      -- FK to absorbed_fields(field_id)
    source_value_before JSONB,
    source_value_after  JSONB,

    -- AER Versioned Plans
    plan_version        INTEGER DEFAULT 1,
    revision_rationale  TEXT,
    confidence_score    FLOAT CHECK (confidence_score >= 0 AND confidence_score <= 1),
    delegation_chain    JSONB,
    verdict             JSONB,

    -- Context Graph Linkage
    parent_trace_ids    UUID[],
    child_trace_ids     UUID[],
    content_hash        TEXT,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
"#;

// ── 2. ABSORBED FIELDS (Auto‑Evolving Schema) ──
pub const CREATE_ABSORBED_FIELDS: &str = r#"
CREATE TABLE IF NOT EXISTS absorbed_fields (
    field_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Source Identification
    source_application  TEXT NOT NULL,
    source_database     TEXT NOT NULL,
    source_schema       TEXT NOT NULL,
    source_table        TEXT NOT NULL,
    source_column       TEXT NOT NULL,

    -- Schema Grounding (EvoAgent‑SQL pattern)
    semantic_label      TEXT,
    field_description   TEXT,
    embedding           VECTOR(1536),
    ontology_category   TEXT,

    -- Type & Constraint Discovery
    field_type          TEXT NOT NULL,
    field_length        INTEGER,
    is_nullable         BOOLEAN DEFAULT TRUE,
    validation_rules    JSONB,

    -- Observation Statistics
    first_observed_at   TIMESTAMPTZ,
    last_observed_at    TIMESTAMPTZ,
    observation_count   INTEGER DEFAULT 0,
    unique_users        INTEGER DEFAULT 0,

    -- Six‑Phase Absorption Status
    absorption_status   TEXT DEFAULT 'observing'
                        CHECK (absorption_status IN ('observing','mirroring','absorbed','genesis','replaced','retired')),

    -- CDC Integration (GoldenGate 26ai)
    cdc_connector_id    TEXT,
    cdc_sync_started    TIMESTAMPTZ,
    cdc_last_sync       TIMESTAMPTZ,
    cdc_sync_latency_ms INTEGER,

    -- Auto‑Evolution (ThemisDB pattern)
    cortex_table        TEXT,
    cortex_column       TEXT,
    schema_version      INTEGER DEFAULT 1,
    last_schema_change  TIMESTAMPTZ,
    evolution_history   JSONB,

    -- Governance
    contains_pii        BOOLEAN DEFAULT FALSE,
    pii_type            TEXT,
    retention_policy    TEXT,

    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE(source_application, source_database, source_schema, source_table, source_column)
);
"#;

// ── 3. BEHAVIORAL WORKFLOWS (From Logs to Agents methodology) ──
pub const CREATE_BEHAVIORAL_WORKFLOWS: &str = r#"
CREATE TABLE IF NOT EXISTS behavioral_workflows (
    workflow_id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL,
    source_application  TEXT NOT NULL,

    behavioral_tokens   TEXT[] NOT NULL,
    token_count         INTEGER GENERATED ALWAYS AS (array_length(behavioral_tokens, 1)) STORED,
    workflow_graph      JSONB,                         -- DAG from Jo & Hyun depth‑based layout

    frequency           INTEGER DEFAULT 1,
    total_duration_ms   BIGINT,

    converted_to_skill  BOOLEAN DEFAULT FALSE,
    skill_id            UUID,
    absorption_phase    TEXT DEFAULT 'observing',

    first_observed      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_observed       TIMESTAMPTZ NOT NULL DEFAULT now(),

    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
"#;

// ── Indexes ──
const INDEXES: &[&str] = &[
    // decision_traces
    "CREATE INDEX IF NOT EXISTS idx_dt_user_time ON decision_traces(user_id, timestamp DESC);",
    "CREATE INDEX IF NOT EXISTS idx_dt_behavioral_token ON decision_traces(behavioral_token);",
    "CREATE INDEX IF NOT EXISTS idx_dt_source_app ON decision_traces(source_application);",
    "CREATE INDEX IF NOT EXISTS idx_dt_parent_traces ON decision_traces USING gin(parent_trace_ids);",
    "CREATE INDEX IF NOT EXISTS idx_dt_intent_fts ON decision_traces USING gin(to_tsvector('english', intent));",
    // absorbed_fields
    "CREATE INDEX IF NOT EXISTS idx_af_source ON absorbed_fields(source_application, source_table);",
    "CREATE INDEX IF NOT EXISTS idx_af_status ON absorbed_fields(absorption_status);",
    // behavioral_workflows
    "CREATE INDEX IF NOT EXISTS idx_bw_user_app ON behavioral_workflows(user_id, source_application);",
    "CREATE INDEX IF NOT EXISTS idx_bw_tokens ON behavioral_workflows USING gin(behavioral_tokens);",
    "CREATE INDEX IF NOT EXISTS idx_bw_frequency ON behavioral_workflows(frequency DESC);",
];
