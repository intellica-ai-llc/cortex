use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Pre‑loaded industry intelligence templates.
///
/// Based on Credo AI's Harmonized Controls Framework and RegPass's
/// regulatory knowledge graphs: machine‑readable representations of
/// industry entities, compliance obligations, and governance controls.
pub struct IndustryTemplateRegistry {
    templates: RwLock<HashMap<String, IndustryTemplate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryTemplate {
    pub industry_name: String,
    pub chart_of_accounts: Option<Vec<String>>,   // banking
    pub regulatory_frameworks: Vec<RegulatoryFramework>,
    pub asset_taxonomy: Option<Vec<String>>,       // energy
    pub event_classifications: Option<Vec<String>>, // SCADA events
    pub compliance_checklists: Vec<ComplianceChecklist>,
    pub preloaded_kpis: Vec<IndustryKpi>,
    pub peer_benchmarks: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    pub name: String,            // "NERC CIP-015-1", "EU AI Act", "HIPAA"
    pub jurisdiction: String,    // "US", "EU", "Global"
    pub effective_date: String,
    pub key_articles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceChecklist {
    pub framework: String,
    pub items: Vec<ChecklistItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub requirement: String,
    pub evidence_type: String,   // "audit_log", "crypto_proof", "policy_doc"
    pub priority: String,        // "critical", "high", "medium"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryKpi {
    pub name: String,
    pub formula: String,
    pub benchmark_value: Option<f64>,
    pub unit: String,
}

impl IndustryTemplateRegistry {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Banking
        templates.insert("banking".into(), IndustryTemplate {
            industry_name: "Banking".into(),
            chart_of_accounts: Some(vec!["1000-Assets".into(), "2000-Liabilities".into(), "3000-Equity".into()]),
            regulatory_frameworks: vec![
                RegulatoryFramework { name: "EU AI Act".into(), jurisdiction: "EU".into(), effective_date: "2026-08-01".into(), key_articles: vec!["Art. 12".into(), "Art. 13".into()] },
                RegulatoryFramework { name: "SOX".into(), jurisdiction: "US".into(), effective_date: "2002".into(), key_articles: vec!["Sec. 404".into()] },
            ],
            asset_taxonomy: None,
            event_classifications: None,
            compliance_checklists: vec![],
            preloaded_kpis: vec![
                IndustryKpi { name: "Capital Adequacy Ratio".into(), formula: "CET1 / RWA".into(), benchmark_value: Some(10.5), unit: "%".into() },
                IndustryKpi { name: "Liquidity Coverage Ratio".into(), formula: "HQLA / Net Cash Outflows".into(), benchmark_value: Some(100.0), unit: "%".into() },
            ],
            peer_benchmarks: None,
        });

        // Energy & Utilities
        templates.insert("energy_utilities".into(), IndustryTemplate {
            industry_name: "Energy & Utilities".into(),
            chart_of_accounts: None,
            regulatory_frameworks: vec![
                RegulatoryFramework { name: "NERC CIP-015-1".into(), jurisdiction: "US/CA".into(), effective_date: "2028-10-01".into(), key_articles: vec!["Real‑time computational traces".into()] },
                RegulatoryFramework { name: "EPA Clean Air Act".into(), jurisdiction: "US".into(), effective_date: "ongoing".into(), key_articles: vec!["Title V".into()] },
            ],
            asset_taxonomy: Some(vec!["Generation".into(), "Transmission".into(), "Distribution".into(), "Substation".into()]),
            event_classifications: Some(vec!["SCADA_Fault".into(), "Forced_Outage".into(), "Planned_Maintenance".into()]),
            compliance_checklists: vec![],
            preloaded_kpis: vec![
                IndustryKpi { name: "Generation Availability".into(), formula: "Available Hours / Period Hours".into(), benchmark_value: Some(95.0), unit: "%".into() },
                IndustryKpi { name: "Forced Outage Rate".into(), formula: "Forced Outage Hours / Total Hours".into(), benchmark_value: Some(1.0), unit: "%".into() },
            ],
            peer_benchmarks: None,
        });

        // Healthcare
        templates.insert("healthcare".into(), IndustryTemplate {
            industry_name: "Healthcare".into(),
            chart_of_accounts: None,
            regulatory_frameworks: vec![
                RegulatoryFramework { name: "HIPAA".into(), jurisdiction: "US".into(), effective_date: "ongoing".into(), key_articles: vec!["Privacy Rule".into(), "Security Rule".into()] },
            ],
            asset_taxonomy: None,
            event_classifications: None,
            compliance_checklists: vec![],
            preloaded_kpis: vec![
                IndustryKpi { name: "PHI Access Audit Score".into(), formula: "Audited Accesses / Total Accesses".into(), benchmark_value: Some(100.0), unit: "%".into() },
            ],
            peer_benchmarks: None,
        });

        Self { templates: RwLock::new(templates) }
    }

    /// Get the preloaded template for an industry.
    pub async fn get(&self, industry: &str) -> Option<IndustryTemplate> {
        self.templates.read().await.get(industry).cloned()
    }

    /// List available industries.
    pub async fn list_industries(&self) -> Vec<String> {
        self.templates.read().await.keys().cloned().collect()
    }
}
