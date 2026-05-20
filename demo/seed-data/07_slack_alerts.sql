CREATE TABLE IF NOT EXISTS demo_slack_alerts (
    alert_id TEXT PRIMARY KEY, channel TEXT, message TEXT, sent_at TIMESTAMPTZ,
    triggered_by TEXT
);
INSERT INTO demo_slack_alerts VALUES
  ('SL-001','#ops-alerts','🚨 CRITICAL: GT-01 forced outage. Exhaust temperature 680°C (limit 620°C). WO-5522 created.','2026-04-10 08:45:00+00','DCS-01'),
  ('SL-002','#ops-alerts','⚠️ GT-02 vibration 12.4 mm/s exceeds 10 mm/s threshold. Inspection scheduled. WO-5525.','2026-04-15 14:20:00+00','VM-07')
ON CONFLICT DO NOTHING;
