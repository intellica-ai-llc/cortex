use ed25519_dalek::{VerifyingKey, Verifier};
use serde::{Deserialize, Serialize};

/// Offline‑first license key validation using Distr.sh pattern.
///
/// "You define a JSON payload with whatever your application needs to
/// enforce. Distr issues a signed JWT. Your application verifies the
/// signature at startup using your public key and reads the claims.
/// No network call. No license server. Works fully offline."
/// — Distr.sh, Mar 2026
///
/// Honua's Ed25519 pattern (Mar 2026): "signed offline‑capable license
/// file format, Ed25519 verification, startup validation with clear
/// license status resolution, community mode default when no license."
pub struct LicenseValidator {
    verifying_key: VerifyingKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    pub license_id: String,
    pub customer: String,
    pub plan: String,            // "starter" | "professional" | "enterprise" | "unlimited"
    pub seats: u32,
    pub connectors: String,      // "5" | "15" | "unlimited"
    pub features: Vec<String>,
    pub expires: chrono::NaiveDate,
    pub issued_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedLicense {
    pub payload: LicensePayload,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseStatus {
    Valid,
    Missing,
    Expired { expired_on: chrono::NaiveDate },
    InvalidSignature,
    SeatLimitExceeded { current: u32, max: u32 },
}

impl LicenseValidator {
    pub fn new(public_key: [u8; 32]) -> Self {
        let vk = VerifyingKey::from_bytes(&public_key).expect("valid Ed25519 public key");
        Self { verifying_key: vk }
    }

    /// Validate a license at startup (air‑gapped safe).
    ///
    /// Algorithm:
    ///   1. Load license file from configured path.
    ///   2. Verify Ed25519 signature against embedded public key.
    ///   3. Check expiry date.
    ///   4. Extract entitlements for feature gating.
    ///   5. Return status — all without any network call.
    pub fn validate(&self, license_path: &str) -> LicenseStatus {
        let content = match std::fs::read_to_string(license_path) {
            Ok(c) => c,
            Err(_) => return LicenseStatus::Missing,
        };

        let signed: SignedLicense = match serde_json::from_str(&content) {
            Ok(s) => s,
            Err(_) => return LicenseStatus::InvalidSignature,
        };

        // Serialise payload canonically and verify signature.
        let payload_bytes = serde_json::to_vec(&signed.payload).unwrap_or_default();
        if self.verifying_key.verify(&payload_bytes, &ed25519_dalek::Signature::from_slice(&signed.signature).unwrap()).is_err() {
            return LicenseStatus::InvalidSignature;
        }

        // Check expiry.
        let today = chrono::Utc::now().date_naive();
        if today > signed.payload.expires {
            return LicenseStatus::Expired { expired_on: signed.payload.expires };
        }

        LicenseStatus::Valid
    }

    /// Parse a license to extract feature flags.
    pub fn parse_features(&self, license_path: &str) -> Vec<String> {
        let content = std::fs::read_to_string(license_path).unwrap_or_default();
        let signed: SignedLicense = serde_json::from_str(&content).unwrap_or_else(|_| SignedLicense {
            payload: LicensePayload {
                license_id: "none".into(), customer: "none".into(), plan: "starter".into(),
                seats: 5, connectors: "5".into(), features: vec![],
                expires: chrono::NaiveDate::from_ymd_opt(2027, 1, 1).unwrap(),
                issued_at: chrono::Utc::now(),
            },
            signature: vec![],
        });
        signed.payload.features
    }
}
