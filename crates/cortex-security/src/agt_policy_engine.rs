#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Microsoft Agent Governance Toolkit (AGT) policy engine bridge.
pub struct AGTPolicyEngine {
    policies: HashMap<String, Policy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub description: String,
    pub tool_patterns: Vec<String>,
    pub param_checks: Vec<ParamCheck>,
    pub max_risk_threshold: f64,
    pub enforcement: EnforcementMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamCheck {
    pub field: String,
    pub check_type: ParamCheckType,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ParamCheckType {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    AllowedValues(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementMode {
    Enforce,
    Warn,
}

impl AGTPolicyEngine {
    pub fn new() -> Self {
        let mut policies = HashMap::new();

        policies.insert("default-block-system-tags".into(), Policy {
            name: "default-block-system-tags".into(),
            description: "Blocks tool descriptors containing system manipulation tags".into(),
            tool_patterns: vec!["*".into()],
            param_checks: vec![],
            max_risk_threshold: 0.5,
            enforcement: EnforcementMode::Enforce,
        });

        policies.insert("require-approval-destructive".into(), Policy {
            name: "require-approval-destructive".into(),
            description: "Requires HITL approval for destructive operations".into(),
            tool_patterns: vec![
                "delete_*".into(), "drop_*".into(), "truncate_*".into(),
                "exec_*".into(), "sudo_*".into(),
            ],
            param_checks: vec![],
            max_risk_threshold: 0.3,
            enforcement: EnforcementMode::Enforce,
        });

        Self { policies }
    }

    pub fn evaluate(
        &self,
        tool_name: &str,
        params: &serde_json::Value,
        risk_score: f64,
    ) -> Result<PolicyVerdict, super::SecurityError> {
        for policy in self.policies.values() {
            if policy.tool_patterns.iter().any(|pat| glob_match(pat, tool_name)) {
                for check in &policy.param_checks {
                    let param_value = params.get(&check.field);
                    if !check_passes(check, param_value) {
                        let msg = format!(
                            "Policy '{}' parameter check failed for field '{}'",
                            policy.name, check.field
                        );
                        if policy.enforcement == EnforcementMode::Enforce {
                            return Ok(PolicyVerdict::Denied { policy: policy.name.clone(), reason: msg });
                        } else {
                            tracing::warn!("{}", msg);
                        }
                    }
                }

                if risk_score > policy.max_risk_threshold {
                    return Ok(PolicyVerdict::RequiresApproval {
                        policy: policy.name.clone(),
                        risk_score,
                    });
                }
            }
        }

        Ok(PolicyVerdict::Allowed)
    }
}

#[derive(Debug)]
pub enum PolicyVerdict {
    Allowed,
    Denied { policy: String, reason: String },
    RequiresApproval { policy: String, risk_score: f64 },
}

fn glob_match(pattern: &str, name: &str) -> bool {
    if pattern == "*" { return true; }
    if pattern.starts_with('*') && pattern.ends_with('*') {
        return name.contains(&pattern[1..pattern.len()-1]);
    }
    if pattern.ends_with('*') {
        return name.starts_with(&pattern[..pattern.len()-1]);
    }
    if pattern.starts_with('*') {
        return name.ends_with(&pattern[1..]);
    }
    pattern == name
}

fn check_passes(check: &ParamCheck, value: Option<&serde_json::Value>) -> bool {
    match &check.check_type {
        ParamCheckType::Required => value.is_some(),
        ParamCheckType::AllowedValues(ref allowed) => {
            value.map(|v| allowed.iter().any(|a| {
                serde_json::to_string(a).ok().map(|s| v.to_string().contains(&s.trim_matches('"'))).unwrap_or(false)
            })).unwrap_or(false)
        }
        _ => true,
    }
}