use serde::{Deserialize, Serialize};

pub struct Synthesiser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergentResult {
    pub consensus: String,
    pub confidence: f64,
}

impl Synthesiser {
    pub fn synthesise(&self, s: &str, a: &str, c: &str) -> ConvergentResult {
        ConvergentResult {
            consensus: format!("Synthesised from:\n- {}\n- {}\n- {}", s, a, c),
            confidence: 0.85,
        }
    }
}
