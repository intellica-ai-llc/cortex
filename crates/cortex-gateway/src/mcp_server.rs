use axum::{Router, routing::{get, post}, Json, http::StatusCode, response::IntoResponse};
use crate::SemanticGateway;
use cortex_provenance::ProvenanceEngine;
use cortex_security::semantic_firewall::SemanticFirewall;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

static KILL_SWITCH: AtomicBool = AtomicBool::new(false);

pub fn router(
    gateway: Arc<SemanticGateway>,
    provenance: Arc<ProvenanceEngine>,
    firewall: Arc<SemanticFirewall>,
) -> Router {
    Router::new()
        .route("/mcp", post(handle_mcp))
        .route("/health", get(health_check))
        .route("/admin/kill", post(kill))
        .route("/admin/revive", post(revive))
        .with_state(AppState { gateway, provenance, firewall })
}

#[derive(Clone)]
struct AppState {
    gateway: Arc<SemanticGateway>,
    provenance: Arc<ProvenanceEngine>,
    firewall: Arc<SemanticFirewall>,
}

async fn health_check() -> &'static str { "ok" }

async fn kill() -> impl IntoResponse {
    KILL_SWITCH.store(true, Ordering::SeqCst);
    (StatusCode::OK, Json(serde_json::json!({ "status": "killed" })))
}

async fn revive() -> impl IntoResponse {
    KILL_SWITCH.store(false, Ordering::SeqCst);
    (StatusCode::OK, Json(serde_json::json!({ "status": "revived" })))
}

async fn handle_mcp(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    if KILL_SWITCH.load(Ordering::SeqCst) {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({ "error": "kill switch active" }))).into_response();
    }

    let intent = payload.get("intent").and_then(|v| v.as_str()).unwrap_or("");

    if let Err(_) = state.firewall.evaluate(intent, "", &serde_json::json!({})) {
        return (StatusCode::FORBIDDEN, Json(serde_json::json!({ "error": "Request blocked by semantic firewall" }))).into_response();
    }

    match state.gateway.route_intent(intent).await {
                Ok(plan) => {
            let mut acc = state.provenance.accumulator.write().await;
            let capsule = acc.attach(
                uuid::Uuid::new_v4(),
                cortex_provenance::tracecaps::ActionKind::ToolCall,
                &[],
            );
            let mut ledger = state.provenance.ledger.write().await;
            ledger.append(serde_json::to_string(&capsule).unwrap_or_default());
            (StatusCode::OK, Json(serde_json::json!({ "plan": plan, "capsule_id": capsule.id.to_string() }))).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}