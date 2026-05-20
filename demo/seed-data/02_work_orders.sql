CREATE TABLE IF NOT EXISTS demo_work_orders (
    wo_id TEXT PRIMARY KEY, asset_id TEXT REFERENCES demo_assets(asset_id),
    wo_type TEXT, description TEXT, status TEXT, priority TEXT,
    reported_date DATE, completed_date DATE, supervisor TEXT, location TEXT
);
INSERT INTO demo_work_orders VALUES
  ('WO-5521','A-104','PM','Annual PM inspection of cooling tower CT-04 per SOP-7891','Closed','Medium','2025-05-01','2025-05-02','jsmith','Area-4'),
  ('WO-5522','A-106','CM','Unplanned outage: GT-01 exhaust temperature high – suspected blade degradation','Open','Critical','2026-04-10',NULL,'mwilson','Area-1'),
  ('WO-5523','A-106','EM','Emergency repair: Replace stage-2 turbine blades on GT-01','InProgress','Critical','2026-04-11',NULL,'mwilson','Area-1'),
  ('WO-5524','A-109','PM','Quarterly transformer oil sampling and DGA T-01','Open','High','2026-05-12',NULL,'tchen','Area-2'),
  ('WO-5525','A-107','CM','GT-02 vibration exceeds 12 mm/s – schedule inspection','Open','High','2026-04-15',NULL,'mwilson','Area-1'),
  ('WO-5526','A-103','PM','Semi-annual pump alignment check CP-01','InProgress','Medium','2026-05-08',NULL,'jsmith','Area-3'),
  ('WO-5527','A-114','CM','DCS-01 controller module communication intermittent','Open','Critical','2026-05-05',NULL,'tchen','Area-2'),
  ('WO-5528','A-108','PM','Steam turbine ST-01 annual valve stroking','Open','High','2026-05-15',NULL,'mwilson','Area-1'),
  ('WO-5529','A-115','INSP','Quarterly vibration probe calibration VM-07','Closed','Medium','2025-04-20','2025-04-20','jsmith','Area-3'),
  ('WO-5530','A-111','PM','Monthly emergency generator load test EG-01','InProgress','Medium','2026-05-10',NULL,'tchen','Area-4'),
  ('WO-5531','A-118','CM','Boiler BLR-03 tube leak suspected – pressure drop detected','Open','Critical','2026-05-03',NULL,'mwilson','Area-5'),
  ('WO-5532','A-101','PM','Weekly pump vibration check BP-01','Closed','Low','2026-05-06','2026-05-06','jsmith','Area-5'),
  ('WO-5533','A-117','CM','IDF-02 fan bearing temperature trending up – lubricate and inspect','Open','Medium','2026-05-07',NULL,'tchen','Area-5'),
  ('WO-5534','A-119','PM','Chiller CH-01 seasonal start-up inspection','Open','Medium','2026-05-20',NULL,'jsmith','Area-3'),
  ('WO-5535','A-112','SAF','CAC-01 safety valve certification expired – recertification required','Open','High','2026-04-01',NULL,'tchen','Area-3')
ON CONFLICT DO NOTHING;
