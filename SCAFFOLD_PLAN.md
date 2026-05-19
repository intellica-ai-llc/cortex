1. Environment & Workspace Initialisation
Install Rust 1.78+ (if not already)

bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
Create project root and initialise Git

bash
mkdir intellecta-cortex && cd intellecta-cortex
git init
Create the workspace Cargo.toml
Use the one from Batch 1, but add all 35 crate members.

toml
[workspace]
members = [
    "crates/cortex-core",
    "crates/cortex-gateway",
    "crates/cortex-provenance",
    "crates/cortex-security",
    "crates/cortex-guard",
    "crates/cortex-council",
    "crates/cortex-integration",
    "crates/cortex-interface",
    "crates/cortex-tracedb",
    "crates/cortex-mirror",
    "crates/cortex-absorb",
    "crates/cortex-genesis",
    "crates/cortex-replace",
    "crates/cortex-retire",
    "crates/cortex-pulse",
    "crates/cortex-whisper",
    "crates/cortex-deep-research",
    "crates/cortex-coggen",
    "crates/cortex-iter-research",
    "crates/cortex-rl-bootstrapper",
    "crates/cortex-research-swarm",
    "crates/cortex-aat",
    "crates/cortex-converge",
    "crates/cortex-forge",
    "crates/cortex-mesh",
    "crates/cortex-mobile",
    "crates/cortex-knowledge-snap",
    "crates/cortex-distribution",
    "crates/cortex-onboarding",
    "crates/cortex-testing",
    "crates/cortex-marketplace",
    "crates/cortex-cli",
    "crates/cortex-observability",
    "crates/cortex-dream",
    "crates/cortex-memory",
    "crates/cortex-intelligence",
    "crates/lfab-core",
    "crates/lfab-sleep",
    "crates/cortex-vault",
]
Copy every scaffolded file into the right directory
For each batch (1‑14), create the corresponding file structure and paste the content. If you saved the batch scripts, run them in order. If not, create the directories manually and copy each file from our conversation.

bash
# Example for batch 1 files
mkdir -p crates/cortex-core/src
# then paste lib.rs, config.rs, feature_gate.rs, runtime.rs …
Add the root cortex.toml config, Makefile, Dockerfile, .github/, etc.

First cargo check

bash
cargo check --workspace
Expect many errors—missing imports, wrong feature gates, etc. This is normal. Fix them file by file until cargo check passes with zero errors.

Commit

bash
git add -A && git commit -m "Initial scaffold: full workspace with 35 crates"
2. Core Runtime & Feature Gate (cortex-core)
Implement Config::load – read cortex.toml with toml crate.

Implement FeatureGate::from_license – parse the license features into boolean flags.

Implement Runtime::new – initialise all subsystem structs (even if they are empty).

Implement the main loop skeleton – a loop { tokio::time::sleep(…).await } with a heartbeat counter.

Write a unit test that loads the config and verifies FeatureGate is populated.

cargo check and commit.

3. Semantic Gateway – Embedding Router & Tool Registry (cortex-gateway)
Replace the mock embed() with a deterministic implementation. Use a simple word‑frequency vector (bag of words) so that tests are repeatable and do not require an external model.

Implement ToolRegistry::search properly – loop over tools, compute cosine similarity, sort, return top‑K.

Implement IntentParser for at least 3 actions – “show”, “compare”, “create”.

Write integration test:

Register two tools with different descriptions.

Call route_intent("show me employees") → expect the HR tool to be selected.

Commit.

4. MCP Server (cortex-gateway)
Choose a web framework – axum is idiomatic for Rust.

Implement mcp_server.rs to serve POST /mcp and GET /health.

POST /mcp should accept a JSON body {"intent": "…"} and return a JSON plan with the selected tools.

Run the server locally:

bash
cargo run --bin cortex -- serve
curl -X POST http://localhost:8787/mcp -H 'Content-Type: application/json' -d '{"intent":"show me open work orders"}'
Verify you get a 200 with a JSON plan.

Commit.

5. Provenance – TraceCaps & Merkle Chain (cortex-provenance)
Implement TraceCapsAccumulator using the merkle crate (or a simple hand‑rolled Merkle tree for now).

Implement TraceCaps::attach – create a capsule, compute risk score, hash it, and link to parent capsules.

Implement Signer::new – generate an Ed25519 keypair.

Write a test: create three capsules linked together, verify the Merkle root, sign one, verify the signature.

Commit.

6. Security – Semantic Firewall & CABP (cortex-security)
Implement SemanticFirewall::evaluate with the injection regexes from Batch 3.

Write a test: pass a prompt injection string, expect FirewallRejected. Pass a benign string, expect Ok(()).

Implement CABPPipeline::validate_identity – for now, just check that the user ID is not empty.

Commit.

7. End‑to‑End Gateway Test with Security
Wire SecurityFortress::validate_tool_call into SemanticGateway::route_intent (call it after the plan is built).

Integration test:

Send a malicious intent → expect 403.

Send a benign intent → expect a valid plan with a signed TraceCaps capsule.

Commit.

8. CLI – cortex serve (cortex-cli)
Implement cortex serve to parse arguments (--port, --db), initialise CortexRuntime, and start the MCP server.

Add a --license flag that passes the license file to Config::load and FeatureGate.

cargo run --bin cortex -- serve --port 8787 should start the server and print a startup message.

Commit.

9. CI Pipeline
Create .github/workflows/ci.yml (already scaffolded in Batch 14).

Ensure it runs cargo check, cargo test, cargo clippy, and cargo fmt --check.

Push to GitHub, verify the CI passes.

10. Phase 1 Milestone
✅ Single binary cortex serve starts and accepts MCP requests.

✅ Semantic router correctly matches intents to tools.

✅ Every interaction produces a signed, Merkle‑proofed TraceCaps capsule.

✅ Prompt injection is blocked.

✅ All tests pass, CI is green.

