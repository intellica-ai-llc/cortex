pub mod mae;
pub mod mi;
pub mod pca;
pub mod db;
pub mod mm;
pub mod bug;
pub mod qc;
pub mod mnt;

// Specialist agents (v2–v9)
pub mod ri;
pub mod whisper;
pub mod observational;
pub mod schema_grounding;
pub mod knowledge;
pub mod converge;
pub mod forge;

// Observation agents (v9)
pub mod engineer;
pub mod observer;
pub mod analyst;
pub mod pii_redaction;

// Operations agents (v10+)
pub mod mirror_agent;

// Gap-closure agents
pub mod rare_workflow_detector;
pub mod business_rule_extractor;
