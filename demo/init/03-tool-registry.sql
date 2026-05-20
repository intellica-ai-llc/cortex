-- Pre-register demo tools
INSERT INTO tools (id, name, description, input_schema, output_schema) VALUES
  ('a0000000-0000-0000-0000-000000000001', 'maximo_get_work_order',     'Retrieve a Maximo work order by ID',           '{"type":"object","properties":{"work_order_id":{"type":"string"}}}', '{"type":"object","properties":{"work_order_id":{"type":"string"},"asset_id":{"type":"string"},"status":{"type":"string"},"priority":{"type":"string"}}}'),
  ('a0000000-0000-0000-0000-000000000002', 'maximo_list_open_work_orders','List open Maximo work orders with optional filters','{"type":"object","properties":{"asset_id":{"type":"string"},"location":{"type":"string"}}}','{"type":"array","items":{"type":"object"}}'),
  ('a0000000-0000-0000-0000-000000000003', 'oracle_hr_get_employee',    'Get employee record from Oracle HR',            '{"type":"object","properties":{"person_id":{"type":"string"}}}','{"type":"object"}'),
  ('a0000000-0000-0000-0000-000000000004', 'snowflake_query_costs',     'Query maintenance cost data from Snowflake',    '{"type":"object","properties":{"query":{"type":"string"}}}','{"type":"array","items":{"type":"object"}}'),
  ('a0000000-0000-0000-0000-000000000005', 'jira_get_issues',           'Get Jira issues linked to assets',              '{"type":"object","properties":{"asset_id":{"type":"string"}}}','{"type":"array","items":{"type":"object"}}'),
  ('a0000000-0000-0000-0000-000000000006', 'github_list_prs',           'List GitHub pull requests by repo',             '{"type":"object","properties":{"repo":{"type":"string"}}}','{"type":"array","items":{"type":"object"}}'),
  ('a0000000-0000-0000-0000-000000000007', 'slack_post_alert',          'Post an alert to a Slack channel',              '{"type":"object","properties":{"channel":{"type":"string"},"text":{"type":"string"}}}','{"type":"object"}'),
  ('a0000000-0000-0000-0000-000000000008', 'backup_browse_tables',      'Browse tables from backup data',                '{"type":"object","properties":{"source":{"type":"string"}}}','{"type":"array","items":{"type":"object"}}')
ON CONFLICT DO NOTHING;

INSERT INTO source_systems (system_name, system_type, vendor, fields_discovered, fields_absorbed, absorption_phase, license_cost_annual) VALUES
  ('IBM Maximo', 'EAM', 'IBM', 800, 380, 'absorbing', 250000),
  ('Oracle HR',  'HR',  'Oracle', 300, 0, 'observing', 120000),
  ('Snowflake',  'Analytics', 'Snowflake', 150, 150, 'absorbed', 180000),
  ('Jira',       'Issue Tracking', 'Atlassian', 50, 50, 'absorbed', 45000),
  ('Slack',      'Communication', 'Slack', 10, 10, 'absorbed', 32000)
ON CONFLICT DO NOTHING;
