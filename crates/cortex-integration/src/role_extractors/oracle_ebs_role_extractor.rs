//! Oracle EBS Role Extractor – queries FND_USER_RESP_GROUPS_DIRECT,
//! FND_RESPONSIBILITY_VL, and FND_MENU_ENTRIES to build role‑to‑function‑to‑
//! field maps directly from the Oracle EBS database.
//!
//! Based on the documented Oracle EBS security model: Users → Responsibilities
//! → Menus → Functions → Forms. The extractor queries these tables via the
//! Cortex MCP connector to Oracle. No Oracle client libraries required – all
//! queries are standard SQL executed through the PostgreSQL MCP bridge.

use serde::{Deserialize, Serialize};

pub struct OracleEBSRoleExtractor;

/// An Oracle EBS responsibility assigned to a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBSResponsibility {
    pub responsibility_name: String,
    pub responsibility_key: String,
    pub application_name: String,
    pub menu_name: String,
    pub user_count: u32,
}

/// A function accessible through a responsibility's menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBSFunction {
    pub function_name: String,
    pub user_function_name: String,
    pub form_name: Option<String>,
    pub entry_sequence: u32,
}

/// Complete role‑to‑function mapping extracted from EBS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EBSRoleMap {
    pub responsibilities: Vec<EBSResponsibility>,
    pub functions: Vec<EBSFunction>,
    pub extracted_at: chrono::DateTime<chrono::Utc>,
    pub ebs_version: Option<String>,
}

impl OracleEBSRoleExtractor {
    pub fn new() -> Self { Self }

    /// Extract all active responsibilities with their user counts.
    /// Based on the standard FND_USER_RESP_GROUPS_DIRECT join used by
    /// Oracle EBS DBAs for audit and access review.
    pub fn responsibilities_query() -> &'static str {
        r#"
        SELECT
            frv.responsibility_name,
            frv.responsibility_key,
            fav.application_name,
            fmv.user_menu_name AS menu_name,
            COUNT(DISTINCT furg.user_id) AS user_count
        FROM fnd_user_resp_groups_direct furg
        JOIN fnd_responsibility_vl frv ON furg.responsibility_id = frv.responsibility_id
        JOIN fnd_application_vl fav ON frv.application_id = fav.application_id
        LEFT JOIN fnd_menus_vl fmv ON frv.menu_id = fmv.menu_id
        WHERE SYSDATE BETWEEN furg.start_date AND NVL(furg.end_date, SYSDATE + 1)
          AND SYSDATE BETWEEN frv.start_date AND NVL(frv.end_date, SYSDATE + 1)
        GROUP BY frv.responsibility_name, frv.responsibility_key,
                 fav.application_name, fmv.user_menu_name
        ORDER BY user_count DESC
        "#
    }

    /// Extract functions accessible through a specific menu.
    /// Based on the FND_MENU_ENTRIES explosion used for security audits.
    pub fn menu_functions_query() -> &'static str {
        r#"
        SELECT
            fffv.function_name,
            fffv.user_function_name,
            fme.entry_sequence,
            ffv.form_name
        FROM fnd_menu_entries fme
        JOIN fnd_form_functions_vl fffv ON fme.function_id = fffv.function_id
        LEFT JOIN fnd_form_vl ffv ON fffv.form_id = ffv.form_id
        WHERE fme.menu_id = (SELECT menu_id FROM fnd_menus WHERE menu_name = :menu_name)
        ORDER BY fme.entry_sequence
        "#
    }
}
