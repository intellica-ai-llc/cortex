use serde::{Deserialize, Serialize};
use ed25519_dalek::Signer;

/// Cryptographically signs a retirement certificate and anchors it
/// via SCITT (Supply Chain Integrity, Transparency, and Trust).
pub struct RetirementCertificateSigner {
    signing_key: ed25519_dalek::SigningKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedRetirementCertificate {
    pub certificate: RetirementCertificatePayload,
    pub signature: Vec<u8>,
    pub scitt_receipt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetirementCertificatePayload {
    pub source: String,
    pub fields_absorbed: u64,
    pub workflows_migrated: u64,
    pub data_integrity_hash: String,       // Merkle root of all absorbed data
    pub compliance_frameworks: Vec<String>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub signed_by: String,
}

impl RetirementCertificateSigner {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let key = ed25519_dalek::SigningKey::generate(&mut rng);
        Self { signing_key: key }
    }

    /// Sign a retirement certificate.
    pub fn sign(&self, payload: &RetirementCertificatePayload) -> SignedRetirementCertificate {
        let serialized = serde_json::to_vec(payload).unwrap();
        let signature = self.signing_key.sign(&serialized).to_vec();
        SignedRetirementCertificate {
            certificate: payload.clone(),
            signature,
            scitt_receipt: None, // anchor later
        }
    }
}
