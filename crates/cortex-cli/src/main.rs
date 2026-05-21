#![allow(unused)]

use clap::{Parser, Subcommand};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "cortex")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(short, long, default_value = "8787")]
        port: u16,
    },
    Deploy { license: Option<String>, offline: bool },
    Connect { system: String, host: String, port: Option<u16> },
    Audit { agent_id: Option<String>, since: Option<String> },
    Configure { key: String, value: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port } => {
            let runtime = cortex_core::CortexRuntime::initialize(None).await?;
            tracing::info!(?runtime.feature_gate, "Cortex initialised");

            let provenance = Arc::new(cortex_provenance::ProvenanceEngine::new([0u8; 32]));
            let firewall = Arc::new(cortex_security::semantic_firewall::SemanticFirewall::new());
            let mut gateway = cortex_gateway::SemanticGateway::new();

            gateway.registry.write().await.register(cortex_gateway::tool_registry::Tool {
                id: "demo".into(),
                name: "demo_tool".into(),
                description: "show me work order and asset".into(),
                description_embedding: gateway.router.embed("show me work order and asset"),
                input_schema: serde_json::json!({}),
                output_schema: None,
                connector_id: None,
                plan_required: cortex_gateway::tool_registry::PlanTier::Free,
                rate_limit_rpm: 60,
                is_active: true,
                tool_hash: "demo_hash".into(),
                created_at: chrono::Utc::now(),
            });

            let app = cortex_gateway::mcp_server::router(
                Arc::new(gateway),
                provenance,
                firewall,
            );
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
            tracing::info!("Cortex MCP gateway listening on port {}", port);
            axum::serve(listener, app).await?;
        }
        Commands::Deploy { license, offline } => {
            println!("Deploying Cortex (offline: {})", offline);
            if let Some(l) = license { println!("License: {}", l); }
        }
        Commands::Connect { system, host, port } => {
            println!("Connecting to {} at {}:{}", system, host, port.unwrap_or(443));
        }
        Commands::Audit { agent_id, since } => {
            println!("Audit query: agent={:?}, since={:?}", agent_id, since);
        }
        Commands::Configure { key, value } => {
            println!("Config: {} = {}", key, value);
        }
    }
    Ok(())
}
