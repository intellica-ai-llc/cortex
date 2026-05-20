use crate::talent::Talent;

/// Database Expert — Schema design, query optimisation, data integrity.
///
/// Based on FlexSQL (May 4, 2026): flexible database exploration and
/// execution. The DB agent incrementally discovers schema structure,
/// grounds decisions in actual data values, and can revise its approach
/// based on what it finds—at any point during reasoning.
pub struct DatabaseExpert;

impl DatabaseExpert {
    pub fn talent() -> Talent {
        let mut t = Talent::new("db", "Database Expert",
            "Schema design, query optimisation, data integrity");
        t.add_capability("schema_discovery");
        t.add_capability("query_optimisation");
        t.add_capability("index_management");
        t.add_capability("migration_planning");
        t.add_capability("flexible_exploration"); // FlexSQL pattern
        t.add_boundary("Never execute DROP, TRUNCATE, or ALTER without CryptoHITL approval");
        t
    }

    /// Discover schema for a database connection (FlexSQL pattern).
    pub async fn discover_schema(connection_string: &str) -> Vec<TableSchema> {
        // In production: connect, query information_schema, build semantic map.
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<ColumnInfo>,
    pub primary_keys: Vec<String>,
    pub foreign_keys: Vec<ForeignKeyRef>,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub struct ForeignKeyRef {
    pub column: String,
    pub ref_table: String,
    pub ref_column: String,
}
