use crate::SecurityError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Context-Aware Broker Protocol (CABP) 6-stage identity pipeline.
///
/// Based on Srinivasan (arXiv:2603.13417, March 2026):
/// "CABP injects identity claims from JWT tokens into individual
/// JSON-RPC request contexts at the broker layer, maintaining
/// stateless request processing"[reference:4].
///
/// Six stages:
///   1. Token validation       — JWT signature, expiry, issuer
///   2. Scope verification     — token scopes vs. required scopes
///   3. User resolution        — token → user identity mapping
///   4. Plan entitlement       — user can execute this tool chain
///   5. Per-tool rate limiting — token bucket per tool per user
///   6. Structured audit log   — write to provenance ledger
pub struct CABPPipeline {
    /// Active JWT verification keys indexed by issuer.
    jwt_keys: RwLock<HashMap<String, Vec<u8>>>,
    /// Rate limiters: (user_id, tool) → token bucket state.
    rate_limiters: RwLock<HashMap<(String, String), TokenBucket>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CABPContext {
    pub jwt_token: Option<String>,
    pub user_id: Option<String>,
    pub session_id: String,
    pub required_scopes: Vec<String>,
}

#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    last_refill: chrono::DateTime<chrono::Utc>,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
}

impl CABPPipeline {
    pub fn new() -> Self {
        Self {
            jwt_keys: RwLock::new(HashMap::new()),
            rate_limiters: RwLock::new(HashMap::new()),
        }
    }

    /// Stage 1-3: Validate identity from JWT token.
    pub async fn validate_identity(&self, user_id: &str) -> Result<CABPContext, SecurityError> {
        // In production: verify JWT signature, check expiry, resolve user.
        // For now: validate that user_id is not empty and looks reasonable.
        if user_id.is_empty() {
            return Err(SecurityError::IdentityFailed("Empty user ID".into()));
        }
        if user_id.len() > 256 {
            return Err(SecurityError::IdentityFailed("User ID too long".into()));
        }

        Ok(CABPContext {
            jwt_token: None,
            user_id: Some(user_id.to_string()),
            session_id: Uuid::new_v4().to_string(),
            required_scopes: vec![],
        })
    }

    /// Stage 4: Check plan entitlement.
    pub fn check_entitlement(
        &self,
        context: &CABPContext,
        required_plan: &str,
    ) -> Result<(), SecurityError> {
        // In production: query the FeatureGate for the user's plan tier.
        // For now: always allow.
        Ok(())
    }

    /// Stage 5: Per-tool rate limiting (token bucket algorithm).
    pub async fn check_rate_limit(
        &self,
        user_id: &str,
        tool: &str,
        max_rpm: u32,
    ) -> Result<(), SecurityError> {
        let key = (user_id.to_string(), tool.to_string());
        let mut limiters = self.rate_limiters.write().await;

        let bucket = limiters.entry(key.clone()).or_insert_with(|| TokenBucket {
            tokens: max_rpm as f64,
            last_refill: chrono::Utc::now(),
            max_tokens: max_rpm as f64,
            refill_rate: max_rpm as f64 / 60.0,
        });

        // Refill tokens based on elapsed time
        let now = chrono::Utc::now();
        let elapsed = (now - bucket.last_refill).num_milliseconds() as f64 / 1000.0;
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.max_tokens);
        bucket.last_refill = now;

        if bucket.tokens < 1.0 {
            return Err(SecurityError::IdentityFailed(format!(
                "Rate limit exceeded for tool '{}' (max {} RPM)", tool, max_rpm
            )));
        }

        bucket.tokens -= 1.0;
        Ok(())
    }

    /// Stage 6: Write structured audit record.
    pub async fn write_audit_record(
        &self,
        context: &CABPContext,
        tool: &str,
        outcome: &str,
    ) {
        tracing::info!(
            user_id = ?context.user_id,
            session_id = %context.session_id,
            tool = tool,
            outcome = outcome,
            "CABP audit record"
        );
    }
}
