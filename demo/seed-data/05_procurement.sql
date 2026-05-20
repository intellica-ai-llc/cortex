CREATE TABLE IF NOT EXISTS demo_procurement (
    cost_id TEXT PRIMARY KEY, wo_id TEXT, cost_type TEXT, amount DECIMAL(12,2), vendor TEXT
);
INSERT INTO demo_procurement VALUES
  ('C-001','WO-5521','Parts',1250.00,'Industrial Parts Co'),
  ('C-002','WO-5521','Labour',4800.00,'Internal'),
  ('C-003','WO-5522','Emergency Procurement',0,NULL),
  ('C-004','WO-5523','Parts',87500.00,'GE Vernova'),
  ('C-005','WO-5523','Labour',32000.00,'Turbine Specialists LLC'),
  ('C-006','WO-5523','Crane Rental',15000.00,'HeavyLift Inc'),
  ('C-007','WO-5527','Parts',4500.00,'Emerson Electric'),
  ('C-008','WO-5529','Parts',800.00,'Bently Nevada'),
  ('C-009','WO-5529','Labour',2400.00,'Internal'),
  ('C-010','WO-5531','Emergency Procurement',125000.00,'Babcock & Wilcox')
ON CONFLICT DO NOTHING;
