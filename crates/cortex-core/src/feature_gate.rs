use serde::{Deserialize, Serialize};

/// Feature gate derived from the licence at boot time.
/// Controls which subsystems and capacities are active.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGate {
    // Capacity limits
    pub max_seats: u32,
    pub max_connectors: u32,

    // Council sizing
    pub agent_council_size: usize,      // 2, 8, or full

    // Provenance & compliance
    pub provenance_level: ProvenanceLevel,
    pub vap_compliance: bool,
    pub nerc_cip: bool,
    pub ietf_aat: bool,

    // Core intelligence features
    pub schema_grounding: bool,
    pub knowledge_snap: bool,
    pub observational_capture: bool,
    pub weaning_engine: bool,
    pub cross_device_sync: bool,

    // Advanced features
    pub deep_research: bool,
    pub convergent_reasoning: bool,
    pub forge_skills: bool,
    pub mesh_federation: bool,
    pub wellness_pulse: bool,
    pub mobile_brain: bool,

    // Absorption pipeline phase gates
    pub phase_observe: bool,
    pub phase_mirror: bool,
    pub phase_absorb: bool,
    pub phase_genesis: bool,
    pub phase_replace: bool,
    pub phase_retire: bool,
}

/// VAP conformance level per IETF framework.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProvenanceLevel {
    Bronze,
    Silver,
    Gold,
}

impl FeatureGate {
    /// Build a feature gate from a licence configuration.
    pub fn from_license(license: &super::config::LicenseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let features = &license.features;
        let is_unlimited = license.connectors == "unlimited";
        let is_enterprise = license.plan == "enterprise" || license.plan == "unlimited";
        let is_pro = is_enterprise || license.plan == "professional";

        Ok(Self {
            max_seats: license.seats,
            max_connectors: if is_unlimited { u32::MAX } else { license.connectors.parse()? },
            agent_council_size: if features.contains(&"council_full".into()) { 8 } else { 2 },

            provenance_level: if features.contains(&"provenance_gold".into()) {
                ProvenanceLevel::Gold
            } else {
                ProvenanceLevel::Silver
            },
            vap_compliance: features.contains(&"vap_compliance".into()),
            nerc_cip: features.contains(&"nerc_cip".into()),
            ietf_aat: is_enterprise,

            schema_grounding: is_pro,
            knowledge_snap: is_pro,
            observational_capture: is_pro,
            weaning_engine: is_enterprise,
            cross_device_sync: is_pro,

            deep_research: is_enterprise,
            convergent_reasoning: is_enterprise,
            forge_skills: is_pro,
            mesh_federation: is_enterprise,
            wellness_pulse: false,   // opt-in separately
            mobile_brain: is_enterprise,

            phase_observe: true,
            phase_mirror: is_pro,
            phase_absorb: is_pro,
            phase_genesis: is_pro,
            phase_replace: is_enterprise,
            phase_retire: is_enterprise,
        })
    }
}
