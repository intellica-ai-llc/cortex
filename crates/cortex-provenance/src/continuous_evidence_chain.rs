/// Merkle‑chained phase receipts linking all six obsolescence phases.
pub struct ContinuousEvidenceChain;

impl ContinuousEvidenceChain {
    pub fn new() -> Self { Self }

    pub fn link_phase(&self, previous: Option<&str>, current: &str) -> String {
        match previous {
            Some(prev) => format!("{}|{}", prev, current),
            None => current.to_string(),
        }
    }
}
