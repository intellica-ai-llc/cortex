use super::tracecaps::VAPLevel;

/// Verifiable Action Provenance compliance layer (IETF VAP framework).
pub struct VAPComplianceLayer;

impl VAPComplianceLayer {
    pub fn new() -> Self { Self }

    pub fn assess_level(&self, risk: f64) -> VAPLevel {
        if risk < 0.3 { VAPLevel::Gold }
        else if risk < 0.7 { VAPLevel::Silver }
        else { VAPLevel::Bronze }
    }
}
