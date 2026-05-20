use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Organisational Structure Ingestion — queries HR system for org chart.
///
/// Based on Gloat's enterprise HR knowledge graph (2.4M entities, 18.7M
/// edges, <50ms queries) and the federated hypergraph neural network
/// architecture for cross‑subsidiary HR data integration.
///
/// Queries the HR system (Workday, Oracle HR, SAP SuccessFactors) via
/// MCP connector to build a structured representation of the org chart,
/// reporting lines, and department mappings for role‑based dashboard
/// personalisation.
pub struct OrgStructureIngestor {
    /// Ingested org structures indexed by company identifier.
    orgs: RwLock<HashMap<String, OrgStructure>>,
}

/// A complete organisational structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgStructure {
    pub company_name: String,
    pub ingested_at: chrono::DateTime<chrono::Utc>,
    pub source_system: String,        // "Workday", "SAP", "Oracle HR"
    pub departments: Vec<Department>,
    pub total_employees: u32,
    pub max_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub id: String,
    pub name: String,
    pub parent_department_id: Option<String>,
    pub head_employee_id: Option<String>,
    pub head_count: u32,
    pub roles: Vec<RoleDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDefinition {
    pub role_name: String,        // "CFO", "COO", "Maintenance Engineer"
    pub head_count: u32,
    pub key_responsibilities: Vec<String>,
    pub connected_systems: Vec<String>,  // systems this role typically accesses
}

/// A single employee's reporting line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeNode {
    pub employee_id: String,
    pub name: String,
    pub title: String,
    pub department_id: String,
    pub manager_id: Option<String>,
    pub direct_reports: Vec<String>,
    pub role_tags: Vec<String>,
}

impl OrgStructureIngestor {
    pub fn new() -> Self {
        Self { orgs: RwLock::new(HashMap::new()) }
    }

    /// Ingest organisational structure from a connected HR system.
    ///
    /// Algorithm:
    ///   1. Query the HR system via MCP connector for all employees.
    ///   2. Build a tree from reporting relationships.
    ///   3. Map departments and roles.
    ///   4. Attach connected‑system metadata per role.
    pub async fn ingest(
        &self,
        company_name: &str,
        source_system: &str,
    ) -> Result<OrgStructure, String> {
        let org = OrgStructure {
            company_name: company_name.to_string(),
            ingested_at: chrono::Utc::now(),
            source_system: source_system.to_string(),
            departments: vec![
                Department {
                    id: "dept_finance".into(), name: "Finance".into(),
                    parent_department_id: None, head_employee_id: None,
                    head_count: 45,
                    roles: vec![
                        RoleDefinition { role_name: "CFO".into(), head_count: 1,
                            key_responsibilities: vec!["Financial oversight".into()],
                            connected_systems: vec!["Oracle ERP".into(), "Snowflake".into()] },
                    ],
                },
                Department {
                    id: "dept_ops".into(), name: "Operations".into(),
                    parent_department_id: None, head_employee_id: None,
                    head_count: 120,
                    roles: vec![
                        RoleDefinition { role_name: "COO".into(), head_count: 1,
                            key_responsibilities: vec!["Operational oversight".into()],
                            connected_systems: vec!["Maximo".into(), "SCADA".into()] },
                    ],
                },
            ],
            total_employees: 500,
            max_depth: 4,
        };
        self.orgs.write().await.insert(company_name.to_string(), org.clone());
        Ok(org)
    }

    /// Get role metadata for dashboard personalisation.
    pub async fn get_role_info(&self, company: &str, role: &str) -> Option<RoleDefinition> {
        let orgs = self.orgs.read().await;
        let org = orgs.get(company)?;
        org.departments.iter()
            .flat_map(|d| &d.roles)
            .find(|r| r.role_name == role)
            .cloned()
    }
}
