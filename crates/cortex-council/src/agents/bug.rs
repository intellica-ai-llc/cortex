use crate::talent::Talent;

/// Debugging Agent — Root-cause analysis and error tracing.
///
/// Investigates failures, traces error provenance through the
/// TraceCaps chain, and proposes fixes.
pub struct DebuggingAgent;

impl DebuggingAgent {
    pub fn talent() -> Talent {
        let mut t = Talent::new("bug", "Debugging Agent",
            "Root-cause analysis, error tracing, fix verification");
        t.add_capability("root_cause_analysis");
        t.add_capability("error_tracing");
        t.add_capability("fix_verification");
        t.add_capability("regression_testing");
        t.add_boundary("Never apply fixes to production without QC approval");
        t
    }

    /// Trace an error through the provenance chain.
    pub fn trace_error(error_id: &str, _capsules: &[serde_json::Value]) -> ErrorTrace {
        ErrorTrace {
            error_id: error_id.to_string(),
            root_cause: None,
            affected_agents: vec![],
            suggested_fix: None,
        }
    }
}

pub struct ErrorTrace {
    pub error_id: String,
    pub root_cause: Option<String>,
    pub affected_agents: Vec<String>,
    pub suggested_fix: Option<String>,
}
