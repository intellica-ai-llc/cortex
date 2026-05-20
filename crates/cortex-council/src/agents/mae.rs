use crate::talent::Talent;

/// Master Architect Essence — Strategic planning and architecture.
///
/// The MAE agent is the first among equals in the agent council.
/// It decomposes high-level initiatives into structured missions,
/// designs the solution architecture, and coordinates the other
/// seven agents through the E²R tree search loop.
pub struct MasterArchitectEssence;

impl MasterArchitectEssence {
    /// Create the MAE talent definition.
    pub fn talent() -> Talent {
        let mut t = Talent::new("mae", "Master Architect Essence",
            "Strategic planning, architecture design, and initiative decomposition");
        t.add_capability("initiative_decomposition");
        t.add_capability("architecture_design");
        t.add_capability("agent_orchestration");
        t.add_capability("risk_assessment");
        t.add_boundary("Never override a human decision without CryptoHITL approval");
        t
    }

    /// Decompose a high-level initiative into a mission plan.
    pub fn decompose_initiative(description: &str) -> Vec<String> {
        // In production: LLM-based decomposition with constraint validation.
        // Returns a list of sub-task descriptions for the E²R tree.
        vec![
            format!("Analyse: {}", description),
            format!("Design solution for: {}", description),
            format!("Validate plan for: {}", description),
        ]
    }
}
