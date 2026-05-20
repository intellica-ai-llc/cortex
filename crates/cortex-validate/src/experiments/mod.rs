pub mod x1_mcp_security;
pub mod x2_semantic_routing;
pub mod x3_provenance_integrity;
pub mod x4_agent_council;
pub mod x5_absorption_equivalence;
pub mod x6_backup_extraction;
pub mod x7_cdc_latency;
pub mod x8_deep_research;
pub mod x9_convergent_reasoning;
pub mod x10_wellness_correlation;
pub mod x11_genui_compliance;
pub mod x12_mobile_parity;

use crate::benchmark_registry::BenchmarkRegistry;

/// Register all 12 built-in experiments.
pub fn register_all(registry: &mut BenchmarkRegistry) {
    registry.register(Box::new(x1_mcp_security::MCPAttackSurfaceExperiment));
    registry.register(Box::new(x2_semantic_routing::SemanticGatewayFuzzingExperiment));
    registry.register(Box::new(x3_provenance_integrity::ProvenanceChainIntegrityExperiment));
    registry.register(Box::new(x4_agent_council::AgentCouncilPerformanceExperiment));
    registry.register(Box::new(x5_absorption_equivalence::AbsorptionPipelineEquivalenceExperiment));
    registry.register(Box::new(x6_backup_extraction::BackupExtractionAccuracyExperiment));
    registry.register(Box::new(x7_cdc_latency::CDCMirrorLatencyExperiment));
    registry.register(Box::new(x8_deep_research::DeepResearchAccuracyExperiment));
    registry.register(Box::new(x9_convergent_reasoning::ConvergentReasoningFactualityExperiment));
    registry.register(Box::new(x10_wellness_correlation::WellnessMultimodalCorrelationExperiment));
    registry.register(Box::new(x11_genui_compliance::GenUIComplianceHallucinationExperiment));
    registry.register(Box::new(x12_mobile_parity::MobileAIPerformanceParityExperiment));
}
