use crate::talent::Talent;
use crate::CouncilError;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// OMC Talent Market — community-driven agent recruitment.
///
/// OMC (arXiv:2604.22446): "A community-driven Talent Market enables
/// on-demand recruitment, allowing the organisation to close
/// capability gaps and reconfigure itself dynamically during
/// execution."
///
/// The market maintains a registry of available specialist profiles
/// that can be instantiated as Talents on demand.
pub struct TalentMarket {
    /// Available talent profiles indexed by role.
    profiles: RwLock<HashMap<String, TalentProfile>>,
    /// Currently active (instantiated) talents from the market.
    active_market_talents: RwLock<HashMap<String, Talent>>,
}

#[derive(Debug, Clone)]
pub struct TalentProfile {
    pub role: String,
    pub name: String,
    pub description: String,
    pub required_capabilities: Vec<String>,
    pub recommended_skills: Vec<String>,
    pub min_quality_threshold: f64,
}

impl TalentMarket {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();

        // Pre-register specialist profiles
        profiles.insert("observational".into(), TalentProfile {
            role: "observational".into(),
            name: "Field Access Observer".into(),
            description: "Watches users in legacy apps and absorbs workflows".into(),
            required_capabilities: vec!["browser_automation".into(), "field_tracking".into()],
            recommended_skills: vec!["session_replay".into(), "workflow_mining".into()],
            min_quality_threshold: 0.7,
        });

        profiles.insert("schema_grounding".into(), TalentProfile {
            role: "schema_grounding".into(),
            name: "Schema Grounding Agent".into(),
            description: "Auto-discovers database schemas and builds semantic maps".into(),
            required_capabilities: vec!["schema_discovery".into(), "semantic_mapping".into()],
            recommended_skills: vec!["text2sql".into(), "nl_interface".into()],
            min_quality_threshold: 0.75,
        });

        profiles.insert("knowledge".into(), TalentProfile {
            role: "knowledge".into(),
            name: "Knowledge Agent".into(),
            description: "Natural language query interface for all data".into(),
            required_capabilities: vec!["nl_query".into(), "cross_system_join".into()],
            recommended_skills: vec!["nl2sql".into(), "data_visualisation".into()],
            min_quality_threshold: 0.7,
        });

        profiles.insert("engineer".into(), TalentProfile {
            role: "engineer".into(),
            name: "Engineer Agent (PMAx)".into(),
            description: "Analyses event-log metadata and generates local scripts for exact computation".into(),
            required_capabilities: vec!["process_mining".into(), "script_generation".into()],
            recommended_skills: vec!["pm_algorithms".into(), "data_privacy".into()],
            min_quality_threshold: 0.8,
        });

        profiles.insert("observer".into(), TalentProfile {
            role: "observer".into(),
            name: "Observer Agent".into(),
            description: "Monitors field-level user interactions and records decision traces".into(),
            required_capabilities: vec!["field_tracking".into(), "behavioral_tokenization".into()],
            recommended_skills: vec!["a11y_inspection".into(), "ocr".into(), "terminal_emulation".into()],
            min_quality_threshold: 0.75,
        });

        profiles.insert("analyst".into(), TalentProfile {
            role: "analyst".into(),
            name: "Analyst Agent (PMAx)".into(),
            description: "Interprets process mining results and identifies workflow patterns".into(),
            required_capabilities: vec!["pattern_mining".into(), "sequence_analysis".into()],
            recommended_skills: vec!["probabilistic_modeling".into(), "report_generation".into()],
            min_quality_threshold: 0.75,
        });

        profiles.insert("pii_redaction".into(), TalentProfile {
            role: "pii_redaction".into(),
            name: "PII Redaction Agent".into(),
            description: "Auto-detects and redacts PII using GoldenGate AI Microservice".into(),
            required_capabilities: vec!["pii_detection".into(), "data_masking".into()],
            recommended_skills: vec!["ner".into(), "gdpr_compliance".into()],
            min_quality_threshold: 0.85,
        });

        Self {
            profiles: RwLock::new(profiles),
            active_market_talents: RwLock::new(HashMap::new()),
        }
    }

    /// Recruit a talent from the market.
    pub async fn recruit(
        &self,
        role: &str,
        _required_skills: &[String],
    ) -> Result<Talent, CouncilError> {
        let profiles = self.profiles.read().await;
        let profile = profiles.get(role).ok_or_else(|| {
            CouncilError::RecruitmentFailed(format!("No profile for role '{}'", role))
        })?;

        let mut talent = Talent::new(role, &profile.name, &profile.description);
        for cap in &profile.required_capabilities {
            talent.add_capability(cap);
        }
        for skill in &profile.recommended_skills {
            talent.acquire_skill(skill);
        }

        self.active_market_talents.write().await.insert(role.to_string(), talent.clone());
        tracing::info!(role, "Talent recruited from market");
        Ok(talent)
    }

    /// Register a new talent profile in the market.
    pub async fn register_profile(&self, role: &str, profile: TalentProfile) {
        self.profiles.write().await.insert(role.to_string(), profile);
    }

    /// List available profiles.
    pub async fn list_profiles(&self) -> Vec<String> {
        self.profiles.read().await.keys().cloned().collect()
    }
}
