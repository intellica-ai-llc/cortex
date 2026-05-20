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
