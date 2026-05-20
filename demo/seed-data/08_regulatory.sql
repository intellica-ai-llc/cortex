CREATE TABLE IF NOT EXISTS demo_regulatory (
    filing_id TEXT PRIMARY KEY, regulation TEXT, filing_name TEXT, due_date DATE,
    status TEXT, penalty_exposure TEXT
);
INSERT INTO demo_regulatory VALUES
  ('REG-001','NERC CIP-015-1','Real-time computational trace audit – Q2 2026','2026-06-30','Pending','$1M/day'),
  ('REG-002','EPA Clean Air Act','Title V emissions compliance report','2026-07-15','Pending','$37,500/day'),
  ('REG-003','FERC','Form 1 Annual Report of Major Electric Utilities','2027-04-18','Pending','Significant'),
  ('REG-004','NERC CIP-015-1','Cybersecurity incident response plan review','2026-09-30','Pending','$1M/day'),
  ('REG-005','OSHA','Process Safety Management audit','2026-08-15','Pending','$15,625/day'),
  ('REG-006','NERC PRC-005-6','Protection system maintenance programme audit','2026-12-31','Pending','$1M/day')
ON CONFLICT DO NOTHING;
