//! Cortex CLI — command-line interface for deploy, connect, audit, configure.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cortex", about = "Intellecta Cortex CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Deploy Cortex
    Deploy { license: Option<String>, offline: bool },
    /// Connect to an enterprise system
    Connect { system: String, host: String, port: Option<u16> },
    /// Audit agent actions
    Audit { agent_id: Option<String>, since: Option<String> },
    /// Configure Cortex
    Configure { key: String, value: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
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
}
