//! Cortex TraceDB — the world’s first six‑phase agentic database.
//!
//! Part 2: source_systems, trace_edges, absorption_branches,
//! retirement_certificates, reactive_mesh.

pub mod schema;
pub mod decision_traces;
pub mod absorbed_fields;
pub mod behavioral_workflows;
// new modules
pub mod source_systems;
pub mod trace_edges;
pub mod absorption_branches;
pub mod retirement_certificates;
pub mod reactive_mesh;

use std::sync::Arc;
use sqlx::PgPool;

pub struct CortexTraceDB {
    pub pool: PgPool,
    pub decision_traces: decision_traces::DecisionTraceRepo,
    pub absorbed_fields: absorbed_fields::AbsorbedFieldRepo,
    pub behavioral_workflows: behavioral_workflows::BehavioralWorkflowRepo,
    pub source_systems: source_systems::SourceSystemRepo,
    pub trace_edges: trace_edges::TraceEdgeRepo,
    pub absorption_branches: absorption_branches::AbsorptionBranchRepo,
    pub retirement_certs: retirement_certificates::RetirementCertificateRepo,
    pub reactive_mesh: reactive_mesh::ReactiveMesh,
}

impl CortexTraceDB {
    pub async fn initialize(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = PgPool::connect(database_url).await?;
        schema::run_migrations(&pool).await?;

        Ok(Self {
            pool: pool.clone(),
            decision_traces: decision_traces::DecisionTraceRepo::new(pool.clone()),
            absorbed_fields: absorbed_fields::AbsorbedFieldRepo::new(pool.clone()),
            behavioral_workflows: behavioral_workflows::BehavioralWorkflowRepo::new(pool.clone()),
            source_systems: source_systems::SourceSystemRepo::new(pool.clone()),
            trace_edges: trace_edges::TraceEdgeRepo::new(pool.clone()),
            absorption_branches: absorption_branches::AbsorptionBranchRepo::new(pool.clone()),
            retirement_certs: retirement_certificates::RetirementCertificateRepo::new(pool.clone()),
            reactive_mesh: reactive_mesh::ReactiveMesh::new(),
        })
    }
}
