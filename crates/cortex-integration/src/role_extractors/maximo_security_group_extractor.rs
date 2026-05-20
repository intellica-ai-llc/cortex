//! IBM Maximo Security Group Extractor – queries APPLICATIONAUTH, GROUPUSER,
//! and MAXAPPS to build security‑group‑to‑application‑to‑field maps directly
//! from the Maximo database.
//!
//! Based on IBM's documented security model: Users → Security Groups →
//! Applications → Options (READ, INSERT, SAVE, DELETE, etc.). The extractor
//! queries these tables via the Cortex MCP connector to the Maximo database.
//! No IBM client libraries required.

use serde::{Deserialize, Serialize};

pub struct MaximoSecurityGroupExtractor;

/// A Maximo security group with its application authorisations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximoSecurityGroup {
    pub group_name: String,
    pub user_count: u32,
    pub applications: Vec<MaximoApplicationAuth>,
}

/// Authorisation for a single application within a security group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximoApplicationAuth {
    pub application_name: String,
    pub description: String,
    pub options: Vec<String>,       // "READ", "INSERT", "SAVE", "DELETE"
}

/// Complete security‑group‑to‑application map extracted from Maximo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaximoRoleMap {
    pub groups: Vec<MaximoSecurityGroup>,
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    pub maximo_version: Option<String>,
}

impl MaximoSecurityGroupExtractor {
    pub fn new() -> Self { Self }

    /// Extract all security groups with user counts.
    /// Based on the standard GROUPUSER + APPLICATIONAUTH queries used by
    /// Maximo administrators for security audits (IBM Support, MoreMaximo).
    pub fn security_groups_query() -> &'static str {
        r#"
        SELECT
            aa.groupname,
            COUNT(DISTINCT gu.userid) AS user_count
        FROM applicationauth aa
        LEFT JOIN groupuser gu ON aa.groupname = gu.groupname
        GROUP BY aa.groupname
        ORDER BY user_count DESC
        "#
    }

    /// Extract application authorisations for a specific security group.
    pub fn application_auth_query() -> &'static str {
        r#"
        SELECT
            aa.groupname,
            aa.app AS application_name,
            ma.description AS app_description,
            aa.optionname
        FROM applicationauth aa
        JOIN maxapps ma ON aa.app = ma.app
        WHERE aa.groupname = :groupname
        ORDER BY ma.description, aa.optionname
        "#
    }
}
