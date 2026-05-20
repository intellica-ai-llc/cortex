use crate::talent::Talent;

/// Knowledge Agent — NL query interface for all data (v2).
///
/// Translates natural language into cross-system queries, joins results,
/// and presents them through the Interface of One.
pub struct KnowledgeAgent;

impl KnowledgeAgent {
    pub fn talent() -> Talent {
        let mut t = Talent::new("knowledge", "Knowledge Agent",
            "Natural language query interface for all connected data sources");
        t.add_capability("nl_to_sql");
        t.add_capability("cross_system_join");
        t.add_capability("visualisation_generation");
        t.add_capability("query_optimisation");
        t.add_boundary("All queries must pass RBAC and field-level audit; never expose PII to unauthorised users");
        t
    }

    /// Translate a natural language query to execution plan.
    pub fn translate_query(nl: &str) -> KnowledgeQueryPlan {
        KnowledgeQueryPlan {
            original: nl.to_string(),
            sub_queries: vec![],
            join_keys: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnowledgeQueryPlan {
    pub original: String,
    pub sub_queries: Vec<SubQuery>,
    pub join_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SubQuery {
    pub target_system: String,
    pub query: String,
    pub timeout_ms: u64,
}
