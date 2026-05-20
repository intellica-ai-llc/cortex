CREATE TABLE IF NOT EXISTS demo_pm_schedules (
    pm_id TEXT PRIMARY KEY, asset_id TEXT REFERENCES demo_assets(asset_id),
    pm_type TEXT, frequency TEXT, last_completed DATE, next_due DATE, status TEXT
);
INSERT INTO demo_pm_schedules VALUES
  ('PM-001','A-104','Inspection','Annual','2025-05-02','2026-05-02','Overdue'),
  ('PM-002','A-106','Inspection','Quarterly','2026-03-15','2026-06-15','Current'),
  ('PM-003','A-109','Oil Sampling','Quarterly','2026-02-10','2026-05-12','Due'),
  ('PM-004','A-111','Load Test','Monthly','2026-04-10','2026-05-10','Current'),
  ('PM-005','A-101','Vibration','Weekly','2026-05-06','2026-05-13','Current'),
  ('PM-006','A-114','Calibration','Semi-Annual','2025-12-01','2026-06-01','Current'),
  ('PM-007','A-108','Valve Stroking','Annual','2025-05-15','2026-05-15','Due'),
  ('PM-008','A-103','Alignment','Semi-Annual','2025-11-08','2026-05-08','Due'),
  ('PM-009','A-119','Seasonal Start-Up','Annual','2025-05-20','2026-05-20','Due'),
  ('PM-010','A-115','Calibration','Quarterly','2026-01-20','2026-04-20','Overdue'),
  ('PM-011','A-107','Inspection','Quarterly','2026-03-01','2026-06-01','Current'),
  ('PM-012','A-118','Inspection','Semi-Annual','2025-11-22','2026-05-22','Due'),
  ('PM-013','A-116','Thermography','Annual','2025-06-30','2026-06-30','Current'),
  ('PM-014','A-112','Oil Change','Quarterly','2026-02-20','2026-05-20','Due'),
  ('PM-015','A-120','Battery Test','Monthly','2026-04-01','2026-05-01','Overdue')
ON CONFLICT DO NOTHING;
