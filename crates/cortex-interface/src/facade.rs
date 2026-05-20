use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Strangler Fig Façade — invisible UI replacement.
///
/// Intercepts all user requests to legacy applications and routes
/// reads/writes to either the real legacy interface (via MCP) or
/// Cortex‑generated panels (reading from TraceDB). The user sees
/// the same screens and workflows; the legacy vendor sees normal
/// activity patterns.
///
/// Based on Azure Architecture Center: “Customers can continue
/// using the same interface, unaware that this migration is
/// taking place. A façade intercepts requests and routes them.”
pub struct StranglerFigFacade {
    /// Routing table: source application → field coverage map.
    routes: RwLock<HashMap<String, ApplicationRoute>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationRoute {
    pub source_application: String,
    pub absorbed_fields: HashMap<String, bool>, // field → absorbed yes/no
    pub route_reads_to_cortex: bool,            // true when ≥80% fields absorbed
    pub route_writes_dual: bool,                // true during Replace phase
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutedRequest {
    pub user_id: String,
    pub application: String,
    pub screen: String,
    pub fields_requested: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutedResponse {
    pub source: ResponseSource,
    pub data: serde_json::Value,
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseSource {
    LegacyApplication,
    CortexTraceDB,
    Hybrid { legacy_pct: f64 },
}

impl StranglerFigFacade {
    pub fn new() -> Self {
        Self { routes: RwLock::new(HashMap::new()) }
    }

    /// Register an application for progressive interception.
    pub async fn register_application(&self, app: &str) {
        self.routes.write().await.insert(app.to_string(), ApplicationRoute {
            source_application: app.to_string(),
            absorbed_fields: HashMap::new(),
            route_reads_to_cortex: false,
            route_writes_dual: false,
        });
    }

    /// Mark a field as absorbed; automatically update routing thresholds.
    pub async fn field_absorbed(&self, app: &str, field: &str) {
        let mut routes = self.routes.write().await;
        if let Some(route) = routes.get_mut(app) {
            route.absorbed_fields.insert(field.to_string(), true);
            // When 80% of known fields are absorbed, switch reads to Cortex.
            let total = route.absorbed_fields.len();
            let absorbed = route.absorbed_fields.values().filter(|v| **v).count();
            route.route_reads_to_cortex = total > 0 && (absorbed as f64 / total as f64) >= 0.8;
        }
    }

    /// Route a user request.
    pub async fn route(&self, req: &RoutedRequest) -> RoutedResponse {
        let routes = self.routes.read().await;
        let route = routes.get(&req.application);

        // If reads are routed to Cortex, serve from TraceDB.
        if route.map(|r| r.route_reads_to_cortex).unwrap_or(false) {
            return RoutedResponse {
                source: ResponseSource::CortexTraceDB,
                data: serde_json::json!({"served_from": "cortex"}),
                latency_ms: 5,
            };
        }

        // Otherwise, proxy to the legacy application.
        RoutedResponse {
            source: ResponseSource::LegacyApplication,
            data: serde_json::json!({"served_from": "legacy"}),
            latency_ms: 200,
        }
    }
}
