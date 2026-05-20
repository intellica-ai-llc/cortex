use serde::{Deserialize, Serialize};

/// Behavioural‑signal‑driven onboarding adaptation.
pub struct AdaptiveChecklist;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub description: String,
    pub completed: bool,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub adaptive: bool,   // reorders based on user signals
}

impl AdaptiveChecklist {
    pub fn new() -> Self { Self }
    pub fn generate(&self, _role: &str) -> Vec<ChecklistItem> {
        vec![
            ChecklistItem { id: "1".into(), description: "Ask your first cross‑system query".into(), completed: false, completed_at: None, adaptive: false },
            ChecklistItem { id: "2".into(), description: "Explore your personalised dashboard".into(), completed: false, completed_at: None, adaptive: true },
        ]
    }
}
