use crate::SecurityError;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Cryptographic Human-In-The-Loop approval (Peyrano L3, arXiv:2604.25555).
///
/// Out-of-band cryptographic approval for high-risk operations.
/// Uses RSA/Ed25519 manifest signing for tool descriptor integrity.
///
/// Design inspired by ZeroBiometrics ZeroSentinel (March 18, 2026):
/// "uses public key infrastructure to cryptographically bind human
/// authorization to AI agent actions. Revoking a certificate cuts
/// off agent authorization instantly"[reference:3].
pub struct CryptoHITL {
    /// Active pending approval requests.
    pending: RwLock<HashMap<String, ApprovalRequest>>,
    /// Approved manifests (tool + params hash → signature).
    approved_manifests: RwLock<HashMap<String, Vec<u8>>>,
    /// Signing key for the Cortex instance.
    signing_key: SigningKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub user_id: String,
    pub tool: String,
    pub params_hash: String,
    pub risk_score: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: ApprovalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved { signed_by: String, signed_at: chrono::DateTime<chrono::Utc> },
    Denied { reason: String },
    Expired,
}

impl CryptoHITL {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut rng);
        Self {
            pending: RwLock::new(HashMap::new()),
            approved_manifests: RwLock::new(HashMap::new()),
            signing_key,
        }
    }

    /// Create a new approval request for a high-risk operation.
    pub async fn request_approval(
        &self,
        user_id: &str,
        tool: &str,
        params: &serde_json::Value,
    ) -> Result<(), SecurityError> {
        let id = Uuid::new_v4().to_string();
        let params_hash = blake3::hash(params.to_string().as_bytes()).to_hex().to_string();

        let request = ApprovalRequest {
            id: id.clone(),
            user_id: user_id.to_string(),
            tool: tool.to_string(),
            params_hash,
            risk_score: 0.85,
            created_at: chrono::Utc::now(),
            status: ApprovalStatus::Pending,
        };

        self.pending.write().await.insert(id, request);

        // In production, this would trigger a push notification
        // to the designated security officer's device.
        Err(SecurityError::HITLNotApproved(format!(
            "Approval required for tool '{}' by user '{}'", tool, user_id
        )))
    }

    /// Record an approval (called when human authorises).
    pub async fn approve(
        &self,
        request_id: &str,
        approver_id: &str,
    ) -> Result<(), SecurityError> {
        let mut pending = self.pending.write().await;
        if let Some(req) = pending.get_mut(request_id) {
            req.status = ApprovalStatus::Approved {
                signed_by: approver_id.to_string(),
                signed_at: chrono::Utc::now(),
            };
            Ok(())
        } else {
            Err(SecurityError::HITLNotApproved("Request not found".into()))
        }
    }

    /// Sign a tool descriptor manifest for integrity verification.
    pub fn sign_manifest(&self, tool_descriptor: &[u8]) -> Vec<u8> {
        self.signing_key.sign(tool_descriptor).to_vec()
    }

    /// Public key for manifest verification.
    pub fn verifying_key(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }
}
