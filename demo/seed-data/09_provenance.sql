CREATE TABLE IF NOT EXISTS demo_provenance (
    capsule_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_name TEXT, action_kind TEXT, tool_name TEXT, intent_text TEXT,
    merkle_hash TEXT, signature TEXT, scitt_receipt TEXT, created_at TIMESTAMPTZ DEFAULT now()
);

-- Generate 500 realistic capsules across 3 agents
DO $$
DECLARE
    agents TEXT[] := ARRAY['MAE','MI','PCA','DB','BUG','QC','MNT','RI'];
    actions TEXT[] := ARRAY['ToolCall','Decision','Inference','MemoryAccess'];
    tools  TEXT[] := ARRAY['maximo_get_work_order','maximo_list_open_work_orders','oracle_hr_get_employee','snowflake_query_costs','jira_get_issues'];
    i INTEGER;
    a TEXT; ac TEXT; t TEXT;
BEGIN
    FOR i IN 1..500 LOOP
        a := agents[1 + (i % array_length(agents,1))];
        ac := actions[1 + (i % array_length(actions,1))];
        t := tools[1 + (i % array_length(tools,1))];
        INSERT INTO demo_provenance (agent_name, action_kind, tool_name, intent_text, merkle_hash, signature, scitt_receipt, created_at)
        VALUES (
            a, ac, t,
            'Demo query ' || i || ': cross-system insight',
            'mh:' || encode(gen_random_bytes(32),'hex'),
            'sig:' || encode(gen_random_bytes(64),'hex'),
            'scitt:receipt:demo:' || i || ':' || to_char(now(),'YYYY-MM-DD'),
            now() - (random() * interval '30 days')
        );
    END LOOP;
END $$;
