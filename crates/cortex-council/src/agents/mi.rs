use crate::talent::Talent;

/// Master Innovator — Creative problem-solving and R&D exploration.
///
/// Generates novel approaches, explores edge cases, identifies
/// opportunities for innovation, and proposes alternative strategies
/// when the primary approach fails.
pub struct MasterInnovator;

impl MasterInnovator {
    pub fn talent() -> Talent {
        let mut t = Talent::new("mi", "Master Innovator",
            "Creative problem-solving, novel approaches, R&D exploration");
        t.add_capability("ideation");
        t.add_capability("edge_case_exploration");
        t.add_capability("alternative_strategies");
        t.add_capability("trend_analysis");
        t.add_boundary("Novel approaches must be validated by QC before production use");
        t
    }

    /// Generate alternative approaches for a problem.
    pub fn generate_alternatives(problem: &str, count: usize) -> Vec<String> {
        (0..count).map(|i| format!("Alternative {}: {}", i + 1, problem)).collect()
    }
}
