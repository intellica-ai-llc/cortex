CREATE TABLE IF NOT EXISTS demo_jira_issues (
    issue_key TEXT PRIMARY KEY, asset_id TEXT, summary TEXT, status TEXT,
    linked_wo TEXT, created DATE
);
INSERT INTO demo_jira_issues VALUES
  ('MAINT-342','A-106','GT-01 exhaust temperature trend analysis needed','Open','WO-5522','2026-04-10'),
  ('MAINT-343','A-106','Root cause analysis for GT-01 blade degradation','InProgress','WO-5523','2026-04-12'),
  ('MAINT-344','A-107','Implement vibration alert threshold for GT-02','Open','WO-5525','2026-04-16'),
  ('MAINT-345','A-114','DCS communication protocol upgrade evaluation','Open','WO-5527','2026-05-06'),
  ('MAINT-346','A-118','Boiler tube inspection history review','Open','WO-5531','2026-05-04')
ON CONFLICT DO NOTHING;
