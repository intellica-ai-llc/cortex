//! Maps extracted Oracle EBS and IBM Maximo role‑to‑function maps to
//! Knowledge Snap role templates and A2UI dashboard specifications.

use serde::{Deserialize, Serialize};

pub struct RoleToDashboardMapper;

/// A mapped role ready for dashboard generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappedRole {
    pub role_name: String,
    pub source_system: String,          // "Oracle EBS", "IBM Maximo"
    pub responsibility_or_group: String,
    pub applications_accessed: Vec<String>,
    pub suggested_panels: Vec<SuggestedPanel>,
    pub suggested_metrics: Vec<String>,
    pub mapped_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPanel {
    pub title: String,
    pub panel_type: String,     // "WorkOrderList", "AssetDashboard", "DataTable"
    pub source_fields: Vec<String>,
}

impl RoleToDashboardMapper {
    pub fn new() -> Self { Self }

    /// Map an Oracle EBS responsibility to Cortex dashboard panels.
    /// Based on Oracle's Enterprise Command Centers taxonomy of 145+ role‑based
    /// dashboards across 6 pillars (Financial, Order Mgmt, Asset Lifecycle,
    /// Procurement, Manufacturing, HCM).
    pub fn map_ebs_responsibility(
        responsibility: &super::oracle_ebs_role_extractor::EBSResponsibility,
    ) -> MappedRole {
        let (panels, metrics) = match responsibility.responsibility_key.to_uppercase().as_str() {
            k if k.contains("GL") || k.contains("GENERAL_LEDGER") => (
                vec![
                    SuggestedPanel { title: "General Ledger Overview".into(), panel_type: "KpiCard".into(), source_fields: vec!["period_name","actual_amount","budget_amount".into()] },
                    SuggestedPanel { title: "Journal Entries".into(), panel_type: "DataTable".into(), source_fields: vec!["je_batch_name","status","posted_date".into()] },
                ],
                vec!["Period Close Status", "JE Approval Backlog"],
            ),
            k if k.contains("AP") || k.contains("PAYABLES") => (
                vec![
                    SuggestedPanel { title: "Invoice Processing".into(), panel_type: "DataTable".into(), source_fields: vec!["invoice_num","vendor_name","amount".into()] },
                ],
                vec!["Invoices Awaiting Approval", "Payment Run Status"],
            ),
            k if k.contains("AR") || k.contains("RECEIVABLES") => (
                vec![
                    SuggestedPanel { title: "Collections Overview".into(), panel_type: "KpiCard".into(), source_fields: vec!["customer_name","balance_due","days_overdue".into()] },
                ],
                vec!["DSO", "Collections Efficiency"],
            ),
            k if k.contains("ASSET") || k.contains("EAM") => (
                vec![
                    SuggestedPanel { title: "Asset Work Orders".into(), panel_type: "DataTable".into(), source_fields: vec!["asset_number","work_order","status".into()] },
                ],
                vec!["PM Compliance", "MTTR", "Asset Utilisation"],
            ),
            _ => (
                vec![SuggestedPanel { title: format!("{} Overview", responsibility.responsibility_name), panel_type: "DataTable".into(), source_fields: vec![] }],
                vec![],
            ),
        };

        MappedRole {
            role_name: responsibility.responsibility_name.clone(),
            source_system: "Oracle EBS".into(),
            responsibility_or_group: responsibility.responsibility_key.clone(),
            applications_accessed: vec![responsibility.application_name.clone()],
            suggested_panels: panels,
            suggested_metrics: metrics,
            mapped_at: chrono::Utc::now(),
        }
    }

    /// Map an IBM Maximo security group to Cortex dashboard panels.
    /// Based on IBM's Maximo module structure: Assets, Work Management,
    /// Inventory, Purchasing, Contracts, Service Desk, Planning, Safety.
    pub fn map_maximo_group(
        group: &super::maximo_security_group_extractor::MaximoSecurityGroup,
    ) -> MappedRole {
        let app_names: Vec<&str> = group.applications.iter()
            .map(|a| a.application_name.as_str())
            .collect();

        let mut panels = Vec::new();
        let mut metrics = Vec::new();

        if app_names.iter().any(|a| a.contains("WOTRACK") || a.contains("Work Order")) {
            panels.push(SuggestedPanel { title: "Work Orders".into(), panel_type: "WorkOrderList".into(), source_fields: vec!["wonum","assetnum","status".into()] });
            metrics.push("Open Work Orders".into());
        }
        if app_names.iter().any(|a| a.contains("ASSET")) {
            panels.push(SuggestedPanel { title: "Asset Dashboard".into(), panel_type: "AssetDashboard".into(), source_fields: vec!["assetnum","location","status".into()] });
            metrics.push("Asset Count by Status".into());
        }
        if app_names.iter().any(|a| a.contains("INVENTOR")) {
            panels.push(SuggestedPanel { title: "Inventory Levels".into(), panel_type: "DataTable".into(), source_fields: vec!["itemnum","storeroom","curbal".into()] });
            metrics.push("Stockout Incidents".into());
        }
        if app_names.iter().any(|a| a.contains("PURCH")) {
            panels.push(SuggestedPanel { title: "Purchase Orders".into(), panel_type: "DataTable".into(), source_fields: vec!["ponum","vendor","status".into()] });
            metrics.push("PO Approval Backlog".into());
        }
        if app_names.iter().any(|a| a.contains("SAFETY")) {
            panels.push(SuggestedPanel { title: "Safety Plans".into(), panel_type: "DataTable".into(), source_fields: vec!["safetyplannum","assetnum".into()] });
            metrics.push("Safety Compliance Rate".into());
        }
        if panels.is_empty() {
            panels.push(SuggestedPanel { title: format!("{} Overview", group.group_name), panel_type: "DataTable".into(), source_fields: vec![] });
        }

        MappedRole {
            role_name: group.group_name.clone(),
            source_system: "IBM Maximo".into(),
            responsibility_or_group: group.group_name.clone(),
            applications_accessed: group.applications.iter().map(|a| a.application_name.clone()).collect(),
            suggested_panels: panels,
            suggested_metrics: metrics,
            mapped_at: chrono::Utc::now(),
        }
    }
}
