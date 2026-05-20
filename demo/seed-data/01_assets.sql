CREATE TABLE IF NOT EXISTS demo_assets (
    asset_id TEXT PRIMARY KEY, name TEXT, asset_type TEXT, manufacturer TEXT,
    model TEXT, serial_number TEXT, install_date DATE, criticality TEXT
);
INSERT INTO demo_assets VALUES
  ('A-101','Boiler Feed Pump BP-01','Rotating','Sulzer','SJD-200','SN-8842','2019-03-15','High'),
  ('A-102','Boiler Feed Pump BP-02','Rotating','Sulzer','SJD-200','SN-8843','2019-03-15','High'),
  ('A-103','Condensate Pump CP-01','Rotating','Flowserve','FPC-450','SN-9912','2020-06-01','High'),
  ('A-104','Cooling Tower CT-04','Static','SPX Cooling','Marley MD','SN-4500','2018-11-20','Medium'),
  ('A-105','Heat Exchanger HE-07','Static','Alfa Laval','M15-BFM','SN-3300','2021-02-10','Medium'),
  ('A-106','Gas Turbine GT-01','Rotating','GE Vernova','7HA.03','SN-7700','2017-09-01','Critical'),
  ('A-107','Gas Turbine GT-02','Rotating','GE Vernova','7HA.03','SN-7701','2018-04-15','Critical'),
  ('A-108','Steam Turbine ST-01','Rotating','Siemens Energy','SST-600','SN-5500','2019-07-22','Critical'),
  ('A-109','Main Transformer T-01','Electrical','ABB','TX-500/230','SN-1120','2016-05-10','Critical'),
  ('A-110','Switchgear SWG-03','Electrical','Schneider','PIX-36','SN-2300','2020-01-15','High'),
  ('A-111','Emergency Generator EG-01','Rotating','Caterpillar','C175-20','SN-4400','2018-12-01','High'),
  ('A-112','Compressed Air Compressor CAC-01','Rotating','Atlas Copco','ZH-1000','SN-6600','2021-08-20','Medium'),
  ('A-113','Fire Water Pump FW-01','Rotating','Patterson','HSC-300','SN-7700','2015-03-10','High'),
  ('A-114','DCS Controller DCS-01','Instrumentation','Emerson','DeltaV PK','SN-8800','2019-01-05','Critical'),
  ('A-115','Vibration Monitor VM-07','Instrumentation','Bently Nevada','3500/42M','SN-9900','2020-04-18','Medium'),
  ('A-116','Motor Control Centre MCC-02','Electrical','Eaton','Freedom 2100','SN-1150','2021-06-30','Medium'),
  ('A-117','Induced Draft Fan IDF-02','Rotating','Howden','AX-2800','SN-2250','2019-09-12','High'),
  ('A-118','Boiler BLR-03','Static','Babcock & Wilcox','FM-120','SN-3350','2017-04-22','Critical'),
  ('A-119','Chiller CH-01','Static','Trane','CVHE-500','SN-4450','2022-01-10','Medium'),
  ('A-120','UPS System UPS-01','Electrical','Vertiv','Liebert EXL','SN-5550','2020-10-05','High')
ON CONFLICT DO NOTHING;
