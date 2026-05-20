//! Cortex Vault – sovereign backup extraction engine.
//!
//! Directly parses native database backup files (RMAN, .bak, IXF,
//! pg_dump, mysqldump) without running the source database.
//! Implements a universal trait with adapters for each RDBMS.
//!
//! The dual‑mode architecture (Option A: intermediate Oracle via
//! Data Pump, Option B: direct block‑level parsing) guarantees zero
//! vendor lock‑in while respecting EU Data Act portability rights.

pub mod vault_trait;
pub mod oracle_datapump;
pub mod oracle_direct;
pub mod sqlserver_backup;
pub mod db2_ixf;
pub mod postgres_backup;
pub mod mysql_backup;
pub mod schema_converter;
pub mod procedural_translator;
pub mod incremental_extractor;
pub mod encryption_bridge;
pub mod checksum_validator;

pub use vault_trait::*;
