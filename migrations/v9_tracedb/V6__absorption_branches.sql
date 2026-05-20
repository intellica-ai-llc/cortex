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
