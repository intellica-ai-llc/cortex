CREATE TABLE IF NOT EXISTS demo_employees (
    person_id TEXT PRIMARY KEY, name TEXT, role TEXT, department TEXT, supervisor TEXT
);
INSERT INTO demo_employees VALUES
  ('E-001','James Smith','Maintenance Supervisor','Operations','Maria Wilson'),
  ('E-002','Tom Chen','Reliability Engineer','Engineering','Maria Wilson'),
  ('E-003','Lisa Park','Maintenance Technician','Operations','James Smith'),
  ('E-004','Ahmed Hassan','Maintenance Technician','Operations','James Smith'),
  ('E-005','Rachel Green','Compliance Officer','Compliance','CFO'),
  ('E-006','Maria Wilson','COO','Executive','CEO'),
  ('E-007','David Brown','Maintenance Planner','Operations','James Smith'),
  ('E-008','Sophie Martin','Operator','Operations','James Smith'),
  ('E-009','Kenji Tanaka','Operator','Operations','James Smith'),
  ('E-010','Priya Patel','Finance Manager','Finance','CFO')
ON CONFLICT DO NOTHING;
