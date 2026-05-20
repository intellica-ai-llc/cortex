use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Converts source RDBMS data types to PostgreSQL equivalents.
pub struct SchemaConverter {
    type_map: HashMap<String, String>,
}

impl SchemaConverter {
    pub fn new() -> Self {
        let mut type_map = HashMap::new();
        // Oracle → PostgreSQL
        type_map.insert("VARCHAR2".into(), "VARCHAR".into());
        type_map.insert("NUMBER".into(), "NUMERIC".into());
        type_map.insert("DATE".into(), "TIMESTAMPTZ".into());
        type_map.insert("CLOB".into(), "TEXT".into());
        type_map.insert("BLOB".into(), "BYTEA".into());
        // SQL Server → PostgreSQL
        type_map.insert("NVARCHAR".into(), "VARCHAR".into());
        type_map.insert("DATETIME".into(), "TIMESTAMPTZ".into());
        type_map.insert("BIT".into(), "BOOLEAN".into());
        // DB2 → PostgreSQL
        type_map.insert("CHARACTER".into(), "VARCHAR".into());
        type_map.insert("TIMESTAMP".into(), "TIMESTAMPTZ".into());
        Self { type_map }
    }

    /// Convert a single data type.
    pub fn convert_type(&self, source_type: &str) -> String {
        self.type_map.get(&source_type.to_uppercase()).cloned().unwrap_or_else(|| source_type.to_string())
    }
}
