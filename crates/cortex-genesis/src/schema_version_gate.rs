use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Schema Version Gate — invalidates UI components on DDL change.
///
/// Based on ThemisDB (Feb 2026) runtime schema version tracking:
/// "Dynamische Rekonfiguration des Datenbankschemas und der
/// Betriebsparameter zur Laufzeit — mit Unterstützung für Zero‑
/// Downtime und automatisierte selbst‑adaptive Anpassungen."
///
/// When a source column type changes during the Genesis phase,
/// any dashboard component built from the old version must be
/// invalidated and regenerated. The Gate tracks schema versions
/// per field and flags stale components.
pub struct SchemaVersionGate {
    /// field_id → current schema_version
    field_versions: RwLock<HashMap<uuid::Uuid, i32>>,
    /// panel_id → set of (field_id, version_at_generation)
    panel_dependencies: RwLock<HashMap<String, HashMap<uuid::Uuid, i32>>>,
}

/// Result of a version gate check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCheckResult {
    pub panel_id: String,
    pub needs_regeneration: bool,
    pub stale_fields: Vec<StaleField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaleField {
    pub field_id: uuid::Uuid,
    pub version_at_generation: i32,
    pub current_version: i32,
    pub field_name: String,
}

impl SchemaVersionGate {
    pub fn new() -> Self {
        Self {
            field_versions: RwLock::new(HashMap::new()),
            panel_dependencies: RwLock::new(HashMap::new()),
        }
    }

    /// Register a field's current schema version.
    pub async fn register_field(&self, field_id: uuid::Uuid, version: i32) {
        self.field_versions.write().await.insert(field_id, version);
    }

    /// Record that a panel was generated using a specific field version.
    pub async fn record_panel_dependency(
        &self,
        panel_id: &str,
        field_id: uuid::Uuid,
        version_at_generation: i32,
    ) {
        let mut deps = self.panel_dependencies.write().await;
        deps.entry(panel_id.to_string())
            .or_default()
            .insert(field_id, version_at_generation);
    }

    /// Check whether a panel needs regeneration.
    ///
    /// Compares each field's version at panel generation time
    /// against the current version. If any field has been
    /// incremented, the panel is stale.
    pub async fn check_panel(&self, panel_id: &str) -> VersionCheckResult {
        let deps = self.panel_dependencies.read().await;
        let versions = self.field_versions.read().await;

        let mut stale_fields = Vec::new();
        let dep_map = deps.get(panel_id);

        if let Some(field_deps) = dep_map {
            for (field_id, version_at_gen) in field_deps {
                let current = versions.get(field_id).copied().unwrap_or(*version_at_gen);
                if current > *version_at_gen {
                    stale_fields.push(StaleField {
                        field_id: *field_id,
                        version_at_generation: *version_at_gen,
                        current_version: current,
                        field_name: field_id.to_string(),
                    });
                }
            }
        }

        VersionCheckResult {
            panel_id: panel_id.to_string(),
            needs_regeneration: !stale_fields.is_empty(),
            stale_fields,
        }
    }

    /// Invalidate all panels that depend on a given field.
    pub async fn invalidate_field(&self, field_id: uuid::Uuid) -> Vec<String> {
        let deps = self.panel_dependencies.read().await;
        deps.iter()
            .filter(|(_, fields)| fields.contains_key(&field_id))
            .map(|(panel_id, _)| panel_id.clone())
            .collect()
    }
}
