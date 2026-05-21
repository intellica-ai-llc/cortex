use crate::config::LicenseConfig;

#[derive(Debug, Clone)]
pub struct FeatureGate {
    pub max_seats: u32,
    pub max_connectors: u32,
    pub agent_council_size: usize,
    pub provenance_level: ProvenanceLevel,
    pub vap_compliance: bool,
    pub nerc_cip: bool,
    pub schema_grounding: bool,
    pub observational_capture: bool,
    pub weaning_engine: bool,
    pub cross_device_sync: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProvenanceLevel { Bronze, Silver, Gold }

impl FeatureGate {
    pub fn from_license(license: &LicenseConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let features = &license.features;
        let is_unlimited = license.connectors == "unlimited";
        Ok(Self {
            max_seats: license.seats,
            max_connectors: if is_unlimited { u32::MAX } else { license.connectors.parse()? },
            agent_council_size: if features.contains(&"council_full".into()) { 8 } else { 2 },
            provenance_level: if features.contains(&"provenance_gold".into()) { ProvenanceLevel::Gold } else { ProvenanceLevel::Silver },
            vap_compliance: features.contains(&"vap_compliance".into()),
            nerc_cip: features.contains(&"nerc_cip".into()),
            schema_grounding: features.contains(&"schema_grounding".into()),
            observational_capture: features.contains(&"observational_capture".into()),
            weaning_engine: features.contains(&"weaning_engine".into()),
            cross_device_sync: features.contains(&"cross_device_sync".into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LicenseConfig;

    fn test_license() -> LicenseConfig {
        LicenseConfig {
            key: "test-key".into(), customer: "test-corp".into(), plan: "enterprise".into(),
            seats: 500, connectors: "unlimited".into(),
            features: vec!["council_full".into(),"provenance_gold".into(),"vap_compliance".into(),"nerc_cip".into(),"schema_grounding".into(),"observational_capture".into(),"weaning_engine".into(),"cross_device_sync".into()],
            expires: "2027-05-07".into(), signature: "ed25519:test".into(),
        }
    }

    #[test]
    fn test_from_license_full() {
        let gate = FeatureGate::from_license(&test_license()).unwrap();
        assert_eq!(gate.agent_council_size, 8);
        assert_eq!(gate.max_connectors, u32::MAX);
        assert_eq!(gate.provenance_level, ProvenanceLevel::Gold);
        assert!(gate.vap_compliance);
    }

    #[test]
    fn test_from_license_limited() {
        let mut license = test_license();
        license.features = vec![];
        license.connectors = "5".into();
        let gate = FeatureGate::from_license(&license).unwrap();
        assert_eq!(gate.agent_council_size, 2);
        assert_eq!(gate.max_connectors, 5);
        assert_eq!(gate.provenance_level, ProvenanceLevel::Silver);
        assert!(!gate.vap_compliance);
    }
}
