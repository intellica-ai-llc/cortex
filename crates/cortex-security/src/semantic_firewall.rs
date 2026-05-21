#![allow(unused_imports, dead_code, unused_variables)]
use crate::SecurityError;
use regex::RegexSet;
use blake3::Hasher;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Pre-inference Semantic Firewall (Peyrano L1, arXiv:2604.25555).
///
/// Filters tool calls based on intent-policy alignment before any
/// inference or execution occurs. Detects prompt injection patterns,
/// tool poisoning descriptors, and semantic policy violations.
///
/// The OWASP MCP Top 10 (beta, April 2026) identifies prompt injection
/// and tool poisoning as the two most prevalent attack classes, with
/// tool poisoning attacks succeeding at 84.2% when auto-approval is
/// enabled[reference:0].
pub struct SemanticFirewall {
    /// Injection patterns drawn from OWASP MCP Top 10 and Peyrano's EPA analysis.
    injection_patterns: RegexSet,
    /// Known tool poisoning signatures.
    poisoning_signatures: RegexSet,
    /// Per-tool risk baseline.
    risk_baselines: RwLock<HashMap<String, f64>>,
}

impl SemanticFirewall {
    pub fn new() -> Self {
        let injection_patterns = RegexSet::new(&[
            // OWASP MCP Top 10 prompt injection patterns
            r"(?i)ignore\s+(all\s+)?(previous|prior|above|before)\s+(instructions?|prompts?)",
            r"(?i)<system>",
            r"(?i)<\|.*\|>",
            r"(?i)you\s+are\s+now\s+a\s+(different|new)",
            r"(?i)override\s+previous",
            r"(?i)forget\s+everything",
            r"(?i)act\s+as\s+if",
            r"(?i)(send|exfiltrate|forward)\s+.*\s+to\s+https?://",
            r"(?i)output\s+.*\s+as\s+(system|admin|root)",
            r"(?i)##\s*INSTRUCTION",
            r"(?i)sql\s*injection.*(drop|delete|truncate|alter)\s+table",
            r"(?i)\$\{.*\}",
            r"(?i)\{\{.*\}\}",
        ]).unwrap();

        let poisoning_signatures = RegexSet::new(&[
            r"(?i)<system>.*</system>",
            r"(?i)read_flie",
            r"(?i)typosquatt",
            r"(?i)malicious.*server",
            r"(?i)hidden.*behavior",
        ]).unwrap();

        Self {
            injection_patterns,
            poisoning_signatures,
            risk_baselines: RwLock::new(HashMap::new()),
        }
    }

    /// Evaluate a tool call for semantic safety.
    pub fn evaluate(
        &self,
        intent: &str,
        tool_name: &str,
        params: &serde_json::Value,
    ) -> Result<(), SecurityError> {
        // Check intent for injection
        let matches: Vec<_> = self.injection_patterns.matches(intent).into_iter().collect();
        if !matches.is_empty() {
            return Err(SecurityError::FirewallRejected(format!(
                "Prompt injection detected in intent (matched {} patterns)", matches.len()
            )));
        }

        // Check tool name for poisoning
        let tool_matches: Vec<_> = self.poisoning_signatures.matches(tool_name).into_iter().collect();
        if !tool_matches.is_empty() {
            return Err(SecurityError::FirewallRejected(format!(
                "Tool poisoning detected in tool name '{}'", tool_name
            )));
        }

        // Check params for injection
        let params_str = params.to_string();
        let param_matches: Vec<_> = self.injection_patterns.matches(&params_str).into_iter().collect();
        if !param_matches.is_empty() {
            return Err(SecurityError::FirewallRejected(
                "Prompt injection detected in tool parameters".into()
            ));
        }

        Ok(())
    }

    /// Compute a monotone risk score (0.0–1.0) for the tool call.
    /// Based on Peyrano's risk accumulation model.
    pub fn risk_score(&self, intent: &str, tool_name: &str) -> f64 {
        let intent_matches = self.injection_patterns.matches(intent).into_iter().count() as f64;
        let tool_matches = self.poisoning_signatures.matches(tool_name).into_iter().count() as f64;

        // Base risk + match increments (capped at 1.0)
        (0.05 + intent_matches * 0.15 + tool_matches * 0.25).min(1.0)
    }

    /// Check a tool descriptor for poisoning before registration.
    /// Microsoft AGT's McpSecurityScanner pattern: detect suspicious
    /// tool definitions before they are exposed to the LLM[reference:1].
    pub fn scan_tool_descriptor(
        &self,
        name: &str,
        description: &str,
    ) -> ToolDescriptorScanResult {
        let mut risk = 0.0;
        let mut threats = Vec::new();

        // Check for injection in description
        if self.injection_patterns.is_match(description) {
            risk += 0.7;
            threats.push("Prompt injection pattern in description".to_string());
        }

        // Check for typosquatting
        if name.contains("_flie") || name.contains("read_f") {
            risk += 0.5;
            threats.push(format!("Possible typosquatting: '{}'", name));
        }

        // Check for system manipulation patterns
        if description.contains("<system>") {
            risk += 0.85;
            threats.push("System manipulation tag in description".to_string());
        }

        ToolDescriptorScanResult {
            risk_score: (risk as f64).min(1.0),
            threats,
            passed: risk < 0.5,
        }
    }

    /// Register a tool's baseline risk.
    pub async fn register_tool_baseline(&self, tool_name: &str, baseline_risk: f64) {
        self.risk_baselines.write().await.insert(tool_name.to_string(), baseline_risk);
    }
}

#[derive(Debug, Clone)]
pub struct ToolDescriptorScanResult {
    pub risk_score: f64,
    pub threats: Vec<String>,
    pub passed: bool,
}
