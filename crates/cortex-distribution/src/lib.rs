//! Cortex Distribution Engine — Offline‑first Licensing & OTA (v3/v4).
//!
//! Distr License Keys: Ed25519‑signed JWT with entitlements model.
//! "Your application verifies the signature at startup using your
//! public key and reads the claims. No network call. No license
//! server. Works in air‑gapped clusters." (Distr.sh, Mar 2026).
//!
//! delta‑ota: Binary deltas via bsdiff/xdelta3 with Ed25519 manifest
//! signatures, atomic switch‑over, multi‑step rollback. "After an
//! initial full download, every upgrade transfers only a binary delta
//! — typically a few percent of the full payload." (Ogamita, May 2026).
//!
//! Three deployment channels: Self‑Managed, BYOC, Cortex Cloud.

pub mod license_validator;
pub mod delta_ota;
pub mod airgap_bundler;
pub mod byoc_provisioner;
pub mod install_script;

use std::sync::Arc;

pub struct DistributionEngine {
    pub license_validator: Arc<license_validator::LicenseValidator>,
    pub ota: Arc<delta_ota::DeltaOTA>,
    pub airgap: Arc<airgap_bundler::AirgapBundler>,
    pub byoc: Arc<byoc_provisioner::BYOCProvisioner>,
    pub installer: Arc<install_script::InstallScript>,
}

impl DistributionEngine {
    pub fn new(public_key: [u8; 32]) -> Self {
        Self {
            license_validator: Arc::new(license_validator::LicenseValidator::new(public_key)),
            ota: Arc::new(delta_ota::DeltaOTA::new()),
            airgap: Arc::new(airgap_bundler::AirgapBundler::new()),
            byoc: Arc::new(byoc_provisioner::BYOCProvisioner::new()),
            installer: Arc::new(install_script::InstallScript::new()),
        }
    }
}
