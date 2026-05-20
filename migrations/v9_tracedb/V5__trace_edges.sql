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
