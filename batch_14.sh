#!/bin/bash
# ============================================================
# BATCH 14: MIGRATIONS, DOCS, CI/CD, DOCKER, CONFIG & TESTS
# Complete infrastructure, deployment, testing, and documentation
# ============================================================
# Grounded in:
#   · PostgreSQL best practices for idempotent migrations.
#   · GitHub Actions CI with caching, clippy, audit, and fuzz.
#   · Docker multi-stage builds with Rust 1.78+ and distroless.
#   · OpenTelemetry integration patterns.
#   · IETF SCITT, VAP, and AAT conformance test patterns.
# ============================================================
set -e

# Root directories
mkdir -p migrations/{v1_base,v9_tracedb,v10_mirror,v11_hardening}
mkdir -p docs
mkdir -p tests/{integration,fuzzing,conformance}
mkdir -p .github/workflows

# ============================================================
# MIGRATIONS
# ============================================================

# v1_base – initial schema
cat > migrations/v1_base/001_initial.sql << 'SQLEOF'
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
SQLEOF

# v9_tracedb – decision traces, absorbed fields, behavioral workflows
cat > migrations/v9_tracedb/V1__decision_traces.sql << 'SQLEOF'
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
SQLEOF

cat > migrations/v9_tracedb/V2__absorbed_fields.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS absorbed_fields (
    field_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_application  TEXT NOT NULL,
    source_database     TEXT NOT NULL,
    source_schema       TEXT NOT NULL,
    source_table        TEXT NOT NULL,
    source_column       TEXT NOT NULL,
    semantic_label      TEXT,
    field_description   TEXT,
    embedding           VECTOR(1536),
    ontology_category   TEXT,
    field_type          TEXT NOT NULL,
    field_length        INTEGER,
    is_nullable         BOOLEAN DEFAULT TRUE,
    validation_rules    JSONB,
    first_observed_at   TIMESTAMPTZ,
    last_observed_at    TIMESTAMPTZ,
    observation_count   INTEGER DEFAULT 0,
    unique_users        INTEGER DEFAULT 0,
    absorption_status   TEXT DEFAULT 'observing'
                        CHECK (absorption_status IN ('observing','mirroring','absorbed','genesis','replaced','retired')),
    cdc_connector_id    TEXT,
    cdc_sync_started    TIMESTAMPTZ,
    cdc_last_sync       TIMESTAMPTZ,
    cdc_sync_latency_ms INTEGER,
    cortex_table        TEXT,
    cortex_column       TEXT,
    schema_version      INTEGER DEFAULT 1,
    last_schema_change  TIMESTAMPTZ,
    evolution_history   JSONB,
    contains_pii        BOOLEAN DEFAULT FALSE,
    pii_type            TEXT,
    retention_policy    TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(source_application, source_database, source_schema, source_table, source_column)
);
CREATE INDEX IF NOT EXISTS idx_af_source ON absorbed_fields(source_application, source_table);
CREATE INDEX IF NOT EXISTS idx_af_status ON absorbed_fields(absorption_status);
CREATE INDEX IF NOT EXISTS idx_af_embedding ON absorbed_fields USING ivfflat(embedding vector_cosine_ops);
SQLEOF

cat > migrations/v9_tracedb/V3__behavioral_workflows.sql << 'SQLEOF'
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
SQLEOF

cat > migrations/v9_tracedb/V4__source_systems.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS source_systems (
    system_id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_name         TEXT NOT NULL,
    system_type         TEXT NOT NULL,
    vendor              TEXT,
    db_connection_string TEXT,
    mcp_connector_name   TEXT,
    total_tables         INTEGER,
    total_columns        INTEGER,
    fields_discovered    INTEGER DEFAULT 0,
    fields_absorbed      INTEGER DEFAULT 0,
    absorption_pct       FLOAT GENERATED ALWAYS AS (
        CASE WHEN total_columns > 0 THEN (fields_absorbed::FLOAT / total_columns::FLOAT) * 100 ELSE 0 END
    ) STORED,
    absorption_phase     TEXT DEFAULT 'observing',
    projected_retirement_date DATE,
    actual_retirement_date    DATE,
    license_cost_annual  DECIMAL(12,2),
    license_savings_ytd  DECIMAL(12,2) DEFAULT 0,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_ss_phase ON source_systems(absorption_phase);
SQLEOF

cat > migrations/v9_tracedb/V5__trace_edges.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS trace_edges (
    edge_id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_trace_id       UUID NOT NULL REFERENCES decision_traces(trace_id),
    to_trace_id         UUID NOT NULL REFERENCES decision_traces(trace_id),
    edge_type           TEXT NOT NULL,
    on_insert_behavior  TEXT,
    content_hash        TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(from_trace_id, to_trace_id, edge_type)
);
SQLEOF

cat > migrations/v9_tracedb/V6__absorption_branches.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS absorption_branches (
    branch_id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_system_id    UUID REFERENCES source_systems(system_id),
    base_branch_id      UUID,
    created_by_agent_id UUID,
    branch_purpose      TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    merged_at           TIMESTAMPTZ,
    merge_status        TEXT DEFAULT 'active' CHECK (merge_status IN ('active','merged','abandoned'))
);
SQLEOF

cat > migrations/v9_tracedb/V7__retirement_certificates.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS retirement_certificates (
    certificate_id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_system_id    UUID REFERENCES source_systems(system_id),
    fields_absorbed     INTEGER NOT NULL,
    workflows_migrated  INTEGER NOT NULL,
    data_integrity_hash TEXT NOT NULL,
    compliance_frameworks TEXT[],
    issued_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    signed_by           UUID,
    signature           BYTEA NOT NULL,
    scitt_receipt       TEXT
);
SQLEOF

# v10_mirror
cat > migrations/v10_mirror/V1__mirror_sync_state_enhanced.sql << 'SQLEOF'
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
SQLEOF

cat > migrations/v10_mirror/V2__cdc_append_log.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS cdc_append_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source          TEXT NOT NULL,
    table_name      TEXT NOT NULL,
    operation       TEXT NOT NULL,
    primary_key     TEXT NOT NULL,
    old_values      JSONB,
    new_values      JSONB,
    transaction_id  TEXT NOT NULL,
    lsn             TEXT,
    ingested_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_cdc_source ON cdc_append_log(source);
CREATE INDEX IF NOT EXISTS idx_cdc_table_time ON cdc_append_log(table_name, ingested_at);
SQLEOF

cat > migrations/v10_mirror/V3__base_snapshot_config.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS base_snapshot_config (
    source          TEXT PRIMARY KEY,
    table_name      TEXT NOT NULL,
    merge_interval_min INTEGER DEFAULT 15,
    merge_strategy  TEXT DEFAULT 'merge_on_read',
    last_merge_at   TIMESTAMPTZ,
    row_count       BIGINT DEFAULT 0,
    UNIQUE(source, table_name)
);
SQLEOF

# v11_hardening
cat > migrations/v11_hardening/V1__tool_versions.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS tool_versions (
    tool_id     TEXT PRIMARY KEY,
    major       INTEGER NOT NULL,
    minor       INTEGER NOT NULL,
    patch       INTEGER NOT NULL,
    last_checked TIMESTAMPTZ DEFAULT now(),
    drift_detected BOOLEAN DEFAULT FALSE
);
SQLEOF

cat > migrations/v11_hardening/V2__consistency_watermarks.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS consistency_watermarks (
    source          TEXT PRIMARY KEY,
    latest_lsn      TEXT,
    latest_timestamp TIMESTAMPTZ,
    updated_at      TIMESTAMPTZ DEFAULT now()
);
SQLEOF

cat > migrations/v11_hardening/V3__multi_tenant_tracedb.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS tenant_registry (
    tenant_id       TEXT PRIMARY KEY,
    database_url    TEXT NOT NULL,
    created_at      TIMESTAMPTZ DEFAULT now(),
    is_active       BOOLEAN DEFAULT TRUE
);
SQLEOF

cat > migrations/v11_hardening/V4__continuous_evidence_chain.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS evidence_chain_receipts (
    receipt_id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phase           TEXT NOT NULL,
    source_system   TEXT NOT NULL,
    merkle_root     TEXT NOT NULL,
    signature       BYTEA NOT NULL,
    scitt_receipt   TEXT,
    created_at      TIMESTAMPTZ DEFAULT now()
);
SQLEOF

cat > migrations/v11_hardening/V5__retention_management.sql << 'SQLEOF'
CREATE TABLE IF NOT EXISTS retention_policies (
    policy_id       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_name     TEXT NOT NULL,
    target_table    TEXT NOT NULL,
    retention_days  INTEGER NOT NULL,
    legal_hold      BOOLEAN DEFAULT FALSE,
    hold_reason     TEXT,
    created_at      TIMESTAMPTZ DEFAULT now()
);
SQLEOF

echo "--- migrations complete (15 SQL files) ---"

# ============================================================
# DOCUMENTATION
# ============================================================
cat > docs/ARCHITECTURE.md << 'DOCEOF'
# Intellecta Cortex Architecture
See the complete architecture specification for detailed design.
DOCEOF

cat > docs/DEPLOYMENT.md << 'DOCEOF'
# Cortex Deployment Guide
## Prerequisites
- Linux server (Ubuntu 22.04+ / RHEL 9+) with 2+ CPU cores, 4+ GB RAM, 20+ GB disk.
- PostgreSQL 15+ with pgvector extension.
## Quick Start
```bash
curl -fsSL https://install.intellica.io | bash
cortex init --license <key>
cortex serve