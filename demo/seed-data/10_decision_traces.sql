INSERT INTO decision_traces (user_id, session_id, agent_id, intent, observation, decision_type, actor_type, behavioral_token, source_application, confidence_score, merkle_hash)
SELECT
  gen_random_uuid(), gen_random_uuid(), gen_random_uuid(),
  'Cross-system query: ' || i,
  ('{"systems_queried":["maximo","oracle","snowflake"],"fields_accessed":["work_order_id","asset_id","status","cost_amount","person_id"]}')::jsonb,
  'ToolCall', 'agent', 'QUERY_Database', 'maximo', 0.94,
  'mh:' || encode(gen_random_bytes(32),'hex')
FROM generate_series(1,300) AS i
ON CONFLICT DO NOTHING;

-- Absorption progress: simulate 380 observed fields for Maximo
INSERT INTO absorbed_fields (source_application, source_table, source_column, semantic_label, field_type, observation_count, absorption_status)
SELECT 'maximo', 'WORKORDER', col, col, 'TEXT', (random()*50+10)::int,
  CASE WHEN random() < 0.68 THEN 'absorbed' ELSE 'mirroring' END
FROM unnest(ARRAY['wonum','assetnum','status','priority','description','location','reportdate','completedate','supervisor','worktype','pmnum','failureclass','problemcode','resolution','labourhours','labourcost','materialcost','totalcost','createdby','assignedto','safetyplan','isolationpoints','permitrequired','lockouttagout','criticalspares','estimatedduration','actualduration','causecode','actioncode','remarks']) AS col
ON CONFLICT DO NOTHING;

-- Simulated workflows
INSERT INTO behavioral_workflows (user_id, source_application, behavioral_tokens, frequency, converted_to_skill)
SELECT gen_random_uuid(), 'maximo',
  ARRAY['MODIFY_Field','MODIFY_Field','SUBMIT_Form','QUERY_Database'],
  (random()*30+5)::int,
  random() < 0.7
FROM generate_series(1,12)
ON CONFLICT DO NOTHING;
