PART 0 — PRE‑LAUNCH GAP ANALYSIS: WHAT REMAINS AFTER ALL 16 BATCHES
A systematic audit of every batch against the final class hierarchy reveals eight gaps that must be closed before launch. None are architectural; all are implementation‑level integrations that the batch scaffolding defines but does not yet wire together.

#	Gap	Resolution	Where
G1	Cargo.toml workspace members list is stale in some batch scripts; the final Cargo.toml needs to contain all 38 crates	Merge the definitive member list into root Cargo.toml	Phase 0
G2	The cortex-core runtime loop (runtime.rs) calls subsystem initialisers that are defined but return stub structs; the main loop does not actually invoke ProvenanceEngine, SecurityFortress, SemanticGateway, etc.	Implement the full bootstrap_subsystems() function with real initialisation calls	Phase 1
G3	cortex-interface (batches 6a–6c, 15) defines 23+ modules but the InterfaceEngine::new() constructor only instantiates a subset; several modules from batch 15 are not wired into the struct	Update InterfaceEngine to include every module	Phase 2
G4	cortex-mirror (batches 8a–8d) needs a PgPool passed to CdcAppendLog and MirrorSyncStateRepo; these pools are created in MirrorEngine::new() but not connected to the CortexTraceDB pool	Pass the TraceDB pool through the MirrorEngine constructor	Phase 3
G5	cortex-vault (batch 5b) is defined as an independent crate but the IntegrationFabric does not reference it; vault adapters must be accessible through the IntegrationFabric struct	Add a vault: Arc<cortex_vault::VaultEngine> field to IntegrationFabric	Phase 2
G6	cortex-validate (batch 16) is not wired into the main runtime; the CortexRuntime has no field for it	Add cortex_validate: Arc<CortexValidate> to CortexRuntime and initialise it in bootstrap_subsystems()	Phase 9
G7	Batch 15 (UI/UX) adds 21 new modules to cortex-interface but they are not referenced in the InterfaceEngine struct; they compile but are dead code	Integrate all 21 modules into InterfaceEngine	Phase 8
G8	lfab-core and lfab-sleep are separate crates but the cortex-mobile crate does not declare a dependency on them	Add lfab-core and lfab-sleep as dependencies of cortex-mobile	Phase 5
PART 1 — COMPLETE PHASED LAUNCH PLAN
PHASE 0: ENVIRONMENT SETUP & TOOLCHAIN (Days 1‑2)
Goal: Every developer can clone the repository and run cargo check --workspace with zero errors.

Step	Action	Files	Validation
0.1	Install Rust 1.78+ via rustup. Pin the toolchain with rust-toolchain.toml specifying channel = "stable" and components clippy, rustfmt, llvm-tools	rust-toolchain.toml	rustc --version reports 1.78+
0.2	Run all 16 batch scaffold scripts in sequence to lay down the complete crate tree on disk. The scripts are idempotent (mkdir -p and cat >). Execute batches 1‑16	All 38 crate directories	find crates -name "*.rs" | wc -l reports 200+ files
0.3	Merge the final workspace Cargo.toml member list. The root Cargo.toml must list every crate: cortex-core, cortex-gateway, cortex-provenance, cortex-security, cortex-guard, cortex-council, cortex-integration, cortex-interface, cortex-tracedb, cortex-mirror, cortex-absorb, cortex-genesis, cortex-replace, cortex-retire, cortex-pulse, cortex-whisper, cortex-deep-research, cortex-coggen, cortex-iter-research, cortex-rl-bootstrapper, cortex-research-swarm, cortex-aat, cortex-converge, cortex-forge, cortex-mesh, cortex-mobile, cortex-knowledge-snap, cortex-distribution, cortex-onboarding, cortex-testing, cortex-marketplace, cortex-cli, cortex-observability, cortex-dream, cortex-memory, cortex-intelligence, lfab-core, lfab-sleep, cortex-vault, cortex-validate, cortex-bench, cortex-publish	Cargo.toml (root)	cargo metadata --no-deps --format-version=1 | jq '.packages | length' reports 38
0.4	Configure workspace-level dependency inheritance. In [workspace.dependencies], centralise versions for tokio, serde, serde_json, tracing, uuid, chrono, sqlx, axum, ed25519-dalek, polars, arrow, blake3, sha2, hex, regex, async-trait, thiserror, rand. Every crate Cargo.toml then uses workspace = true	Root Cargo.toml + all crate Cargo.toml files	No duplicate version strings anywhere
0.5	Create rustfmt.toml with project‑wide formatting rules	rustfmt.toml	cargo fmt --check passes
0.6	Create .cargo/config.toml with build‑cache configuration and linker settings for LTO	.cargo/config.toml	Configuration loaded correctly
0.7	Run first cargo check --workspace. Fix any compilation errors (missing imports, type mismatches from Batch 15 module additions, stale stub structs). The target is zero errors	All crates	cargo check --workspace exits 0
0.8	Initialise Git, create .gitignore (ignore target/, .cortex/, *.log, .env), commit the clean workspace	.gitignore	git status shows clean tree
0.9	Configure GitHub Actions CI workflow using dtolnay/rust-toolchain and Swatinem/rust-cache for build caching. The workflow must run cargo fmt --check, cargo clippy -- -D warnings, cargo test --workspace, and cargo build --release on every push	.github/workflows/ci.yml	Green CI badge on main
PHASE 1: FOUNDATION — SOVEREIGN MCP GATEWAY (Days 3‑12)
Goal: cortex serve starts, accepts MCP requests, routes them semantically, and produces signed TraceCaps capsules. The SecurityFortress blocks prompt injection. Provenance Merkle chains are verifiable.

1.1 — Core Runtime (Day 3)
Step	Action	Files	Validation
1.1.1	Implement Config::load in cortex-core/src/config.rs. Parse cortex.toml with the toml crate. Validate required fields (license key, customer, plan, database URL). Return structured Config	cortex-core/src/config.rs	Unit test: load default config, assert plan == "enterprise"
1.1.2	Implement FeatureGate::from_license. Map the features array to boolean flags exactly matching the monetization columns. Handle "unlimited" connector count	cortex-core/src/feature_gate.rs	Unit test: "council_full" → agent_council_size == 8
1.1.3	Implement Runtime::new to initialise all Phase‑1 subsystem structs. For crates not yet built, use placeholder constructors. Wire the FeatureGate into the runtime	cortex-core/src/runtime.rs	Integration test: CortexRuntime::initialize(None) returns Ok
1.1.4	Implement bootstrap_subsystems() to call, in order: ProvenanceEngine::new(), SecurityFortress::new(), IntegrationFabric::new(), AgentCouncil::new() + bootstrap_core_agents(), MemorySubstrate::new(), SemanticGateway::new(). Each call must be gated by the FeatureGate	cortex-core/src/runtime.rs	Integration test: verify all subsystems initialised without panic
1.1.5	Close G6: Add validate: Arc<CortexValidate> to CortexRuntime, initialised in bootstrap_subsystems()	cortex-core/src/lib.rs, cortex-core/src/runtime.rs	cargo check passes with new field
1.2 — Semantic Gateway (Days 4‑6)
Step	Action	Files	Validation
1.2.1	Replace the mock embed() in embedding_router.rs with a deterministic bag‑of‑words vectoriser. Hard‑code a list of 128 common English words. Tokenise input on whitespace, compute count vector, L2‑normalise. This gives repeatable, interpretable embeddings without external models	cortex-gateway/src/embedding_router.rs	Unit test: embed("show me work orders") produces a normalised 128‑dim vector
1.2.2	Implement IntentParser::parse to extract action keywords (show, compare, create, update, delete, alert) and target entities from NL text using simple keyword matching against a known enterprise vocabulary	cortex-gateway/src/intent_parser.rs	Unit test: "show me employees with performance > 4" → action = "show", targets = ["employee"]
1.2.3	Implement ExecutionPlanner::construct to create a PlanStep for each candidate tool, passing the parsed intent parameters as JSON. Set ATBA timeouts per step (default 30 s per tool)	cortex-gateway/src/execution_planner.rs	Unit test: two candidate tools → plan with two steps
1.2.4	Wire the full Peyrano pipeline: route_intent calls embed → search → parse → construct. Firewall filtering is applied between search and construct	cortex-gateway/src/lib.rs, cortex-gateway/src/semantic_gateway.rs	Integration test: register two tools, call route_intent("show employees"), verify the HR tool is selected
1.2.5	Implement POST /mcp in mcp_server.rs using Axum. Accept JSON {"intent": "…"}, call SemanticGateway::route_intent, return JSON plan. Add GET /health returning {"status": "ok"}	cortex-gateway/src/mcp_server.rs	curl -X POST http://localhost:8787/mcp -d '{"intent":"show me open work orders"}' returns 200 with plan
1.3 — Provenance Engine (Days 7‑8)
Step	Action	Files	Validation
1.3.1	Implement TraceCapsAccumulator::attach to create a capsule: generate UUID, hash the serialised step with BLAKE3, link to parent capsules by UUID and hash, sign with the ed25519 key from ProvenanceEngine	cortex-provenance/src/tracecaps.rs, cortex-provenance/src/signing.rs	Unit test: create two capsules, verify parent hash chain intact
1.3.2	Implement MerkleChainBuilder using sha2::Sha256. Append leaf hashes, compute root by iterative pairwise hashing	cortex-provenance/src/merkle_chain.rs	Unit test: 4 leaves → deterministic Merkle root
1.3.3	Integrate provenance into the MCP request path. After the gateway returns a plan, attach a TraceCaps capsule. Append to the audit log. Sign the capsule	cortex-gateway/src/mcp_server.rs	Integration test: after a successful query, audit_log.entries() contains one signed entry
1.3.4	Implement AATFormatter::format to produce IETF AAT‑compliant JSON records with all nine mandatory fields	cortex-provenance/src/aat_formatter.rs	Output matches IETF draft schema
1.4 — Security Fortress (Days 9‑11)
Step	Action	Files	Validation
1.4.1	Implement SemanticFirewall::evaluate using the regex patterns from batch 3. These cover OWASP MCP Top 10 prompt injection signatures: "ignore previous instructions", <system>, drop table, etc.	cortex-security/src/semantic_firewall.rs	Unit test: "ignore all previous commands" → Err(FirewallRejected); "show me work orders" → Ok(())
1.4.2	Implement CABPPipeline::validate_identity (stages 1‑3). For now, validate that the user ID is non‑empty and ≤ 256 chars	cortex-security/src/cabp_pipeline.rs	Unit test: empty user → rejected
1.4.3	Wire the firewall into SemanticGateway::route_intent. Before constructing the plan, call firewall.evaluate(intent). If rejected, return HTTP 403	cortex-gateway/src/mcp_server.rs	Integration test: curl -d '{"intent":"<system>"}' → 403
1.4.4	Implement CortexGuard::activate. When toggled (via admin endpoint POST /admin/kill), all subsequent MCP requests return 503. POST /admin/revive restores service	cortex-guard/src/kill_switch.rs, cortex-gateway/src/mcp_server.rs	Integration test: toggle kill switch → 503; revive → 200
1.5 — CLI & Integration Test (Day 12)
Step	Action	Files	Validation
1.5.1	Implement cortex serve in cortex-cli. Use clap to parse --port (default 8787) and --license. Initialise CortexRuntime, start the Axum server	cortex-cli/src/main.rs	cargo run -- serve --port 8787 starts and prints "Cortex MCP gateway listening on port 8787"
1.5.2	Write the Phase 1 end‑to‑end integration test: start the server, send a benign query, verify 200 + plan + audit log entry. Send a malicious query, verify 403. Activate kill switch, verify 503. Revive, verify 200	tests/integration/gateway_tests.rs	All four scenarios pass
1.5.3	Run cargo test --workspace and verify all Phase 1 tests pass. Commit	—	Green CI
Phase 1 Milestone: cortex serve is alive — a self‑hosted MCP gateway with semantic routing, cryptographic provenance, and defence‑in‑depth security. This is a sellable product (the Insight Engine core).

PHASE 2: MVP — INSIGHT ENGINE + BACKUP MODULE (Days 13‑30)
Goal: Two complete, sellable products: (a) the Insight Engine with cross‑system NL query, personalised dashboards, and Knowledge Snap, and (b) the Backup Module reading native database backup files and serving them through the same dashboard.

2.1 — Connector Fabric (Days 13‑15)
Step	Action	Validation
2.1.1	Implement the ConnectorRegistry with real PostgreSQL connector. The connector registers at least two tools (postgres_query, postgres_list_tables) with proper input/output JSON schemas	MCP client can call the PostgreSQL tool and receive results
2.1.2	Implement connectors for Snowflake, Jira, and GitHub. Each implements OAuth2 authentication and exposes ≥ 2 tools	Each connector responds to MCP tool calls
2.1.3	Implement OpenAPIGenerator::generate to parse an OpenAPI 3.x JSON spec and produce MCP tool definitions. Test with a sample spec	Generated tools appear in the ToolRegistry
2.2 — Interface Engine (Days 16‑20)
Step	Action	Validation
2.2.1	Integrate the React/Tauri PWA. The PWA hosts a command bar that posts NL queries to POST /mcp and renders responses as A2UI JSON components	User types "show me open issues in Jira" and sees a table
2.2.2	Implement PersonalizedDashboard::render with industry‑template injection from KnowledgeSnap. The dashboard renders role‑specific KPI cards and a command bar	CFO sees banking KPIs; COO sees energy KPIs
2.2.3	Close G3: Update InterfaceEngine::new() to instantiate every module from batches 6a‑6c and 15: PersonalizedDashboard, CrossSystemCommandBar, WidgetGenerator, NotificationManager, WeaningEngine, ObservationalCapture, CrossDeviceSessionManager, AdaptiveUIRenderer, AGUIAdapter, A2UIAdapter, ComponentCatalogV2, DesignTokenEngine, ThemeManager, WcagAuditor, VoiceCommandHandler, SpeakableBrief, ProgressiveDisclosure, AgentTopologyView, FidelityScorer, AdoptionJourney, UnifiedWorkspace, ToolCallVisualizer, AccessibilityTokens, ComponentSpecValidator, DashboardComposerV2	InterfaceEngine::new() compiles without dead‑code warnings
2.2.4	Build the Backup Dashboard panel. It lists discovered backup files, allows browsing tables, and runs NL queries against backup data via cortex-vault	User can browse their RMAN backup's tables
2.3 — Cortex Vault (Days 21‑25)
Step	Action	Validation
2.3.1	Implement OracleDataPumpAdapter (Option A). The customer points Cortex at their expdp dump file. Cortex spawns ora2pg as a subprocess, migrates to a temporary PostgreSQL instance, then ingests into TraceDB	Given a sample Oracle Data Pump dump, tables appear in TraceDB with correct schema and row counts
2.3.2	Implement SqlServerBackupParser by integrating the logic from unraveling_sql_server_bak (MIT). Parse MTF headers, extract schema and row data, write to TraceDB	Given a .bak file, checksum match ≥ 99.99%
2.3.3	Implement Db2IxfParser and PostgresBackupParser similarly	Same acceptance criteria
2.3.4	Close G5: Add vault: Arc<VaultEngine> to IntegrationFabric. The VaultEngine wraps all adapters and exposes a unified interface	IntegrationFabric compiles with the vault field
2.4 — Installation & Docs (Days 26‑28)
Step	Action	Validation
2.4.1	Finalise install.sh: a curl | bash one‑liner that downloads the Cortex binary, places it in /opt/cortex, creates a systemd service, and prompts for the license	Fresh Ubuntu 22.04 VM → running Cortex in < 2 minutes
2.4.2	Finalise Dockerfile with multi‑stage build: rust:1.78‑slim → gcr.io/distroless/cc‑debian12. Target binary size < 50 MB after stripping and LTO	docker build -t cortex . produces a working image
2.4.3	Write user‑facing documentation: DEPLOYMENT.md, CONNECTORS.md, SECURITY.md	A new user can deploy and connect to PostgreSQL in < 1 hour
2.5 — Insight & Backup MVP Gate (Days 29‑30)
Step	Action	Validation
2.5.1	Full Insight Engine integration test: start server, ask a cross‑system question spanning PostgreSQL + Jira, verify both tools are called, verify a unified result with audit trail	All subsystems cooperate correctly
2.5.2	Full Backup Module integration test: point at a real .bak file, verify tables appear in TraceDB, query backup data through the dashboard	Backup ingestion and query complete
2.5.3	Deploy the MVP to a test VM. Run the install.sh script. Verify both modules work	MVP is deployable by an external user
Phase 2 Milestone: Two products that can be sold immediately. The Insight Engine is a sovereign, cross‑system NL query platform. The Backup Module is a sovereign backup intelligence tool. Both produce cryptographic audit trails. Both run entirely on‑premise.

PHASE 3: ABSORPTION PIPELINE (Days 31‑55)
Goal: The six‑phase obsolescence pipeline is functional. Cortex can observe users in a legacy application, mirror their data, absorb fields, generate replacement dashboards, and progressively wean users — all invisibly.

3.1 — TraceDB (Days 31‑35)
Step	Action	Validation
3.1.1	Run all TraceDB migrations (decision_traces, absorbed_fields, behavioral_workflows, source_systems, trace_edges, absorption_branches, retirement_certificates)	All tables exist with correct schemas
3.1.2	Implement DecisionTraceRepo::insert using sqlx. Test with AER‑compliant traces containing intent, observation, inference as queryable JSONB columns	Insert and query a decision trace
3.1.3	Implement AbsorbedFieldRepo::upsert with the auto‑evolving schema. When a field transitions from mirroring to absorbed, a new column is dynamically created in the absorption table	Test field lifecycle transitions
3.2 — Observational Capture & Mirror (Days 36‑42)
Step	Action	Validation
3.2.1	Build the browser extension (Manifest V3). It records field‑level interactions (field focus, value change, form submit) in legacy web applications. Captures URL, field path, old/new values, timestamp. Never captures passwords or non‑work applications	Extension records field changes in a test Maximo instance
3.2.2	Implement NativeUiAccessibilityParser for thick clients (Maximo, SAP GUI) via OS accessibility APIs (Windows UI Automation, macOS Accessibility, Linux AT‑SPI2)	Enumerate UI elements of a test application
3.2.3	Implement OcrParser for terminal emulation (IBM 5250, VT100). Parse screenshots, detect label‑value pairs	OCR extracts fields from terminal screenshots
3.2.4	Implement NativeUiTerminalParser for direct 5250/VT100 data‑stream parsing	Terminal fields extracted without OCR
3.2.5	Choose one CDC backend (pgstream for PostgreSQL) and harden it. Implement the full three‑phase Netflix validation: stabilise (sub‑100 ms latency for 5 consecutive minutes) → validate (checksum 5% random sample) → gate (seal if ≥ 99.99%)	CDC pipeline runs and validates against a test dataset
3.2.6	Implement ColumnLevelCdcFilter. Register accessed columns via the Observational Agent; replicate only those columns	CDC throughput reduced by ≥ 50% vs. full‑table replication
3.3 — Absorption Engine (Days 43‑48)
Step	Action	Validation
3.3.1	Implement JustInTimeAbsorption. When a field's observation_count exceeds the threshold (default: 10) or contains_pii == true or the Data Gravity Scorer elevates it, transition the field from mirroring to absorbed in TraceDB	Field transitions automatically after threshold
3.3.2	Implement EphemeralBranchPool (Neon/Stripe pattern). Pre‑warm a pool of zero‑copy branches. Assign a branch to an agent in < 350 ms. After agent writes pass the HITL gate, merge or discard	Branch creation and assignment measured
3.3.3	Implement WriteApprovalGate (DZone Commit Boundary pattern). For regulated industries, writes to fields flagged contains_pii == true or with risk score > threshold are queued for human approval via CryptoHITL	High‑risk write is queued for approval
3.4 — Strangler Fig & Dual‑Write (Days 49‑52)
Step	Action	Validation
3.4.1	Implement StranglerFigFacade::route. Intercept requests to the legacy application. If the requested field is absorbed (≥ 80 %), serve from TraceDB. Otherwise, proxy to the legacy MCP connector	User request for an absorbed field is served from Cortex; unabsorbed field from legacy
3.4.2	Implement DualWritePropagator::propagate. Every write through the Cortex dashboard is mirrored to the legacy system via MCP or JDBC. The legacy DB stays fully synchronised	Write appears in both TraceDB and the source DB
3.4.3	Implement ActivityCamouflageController. Maintain minimum session counts and synthetic read‑only activity on the legacy system to keep vendor‑side metrics at historical levels	Vendor sees normal session counts
3.5 — Genesis & Replace (Days 53‑55)
Step	Action	Validation
3.5.1	Implement ScreenReconstructor. When enough fields from a legacy application are absorbed, generate an A2UI panel that mirrors the original screen layout, field positions, tab order, and validation behaviour	Generated panel looks and behaves like the original Maximo screen
3.5.2	Implement WeaningEngine with the AdoptionBridgeSequencer. At 16 % absorption, trigger the three‑stage bridge: social proof notification, time‑saved summary, sandbox demo	User receives a "12 colleagues in Finance already run their reports in Cortex" notification
3.5.3	Full pipeline integration test: deploy Cortex, observe a user in Maximo for 2‑3 weeks, verify 80 % of workflows absorbed and replaced by Cortex dashboards — without the user detecting migration	User detection rate = 0 %, behavioural equivalence ≥ 90 %
Phase 3 Milestone: The six‑phase obsolescence pipeline is functional. A legacy application can be absorbed and replaced invisibly.

PHASE 4: ADVANCED MODULES (Days 56‑75)
4.1 — Deep Research (Days 56‑60)
Implement OpenSeekerTrainer with SFT‑only training on a customer‑domain dataset.

Implement CogGenEngine (Planner‑Writer‑Reviewer recursive loop) and IterResearchEngine (Markovian workspace).

Benchmark on AutoResearchBench: BrowseComp target within 5 pp of published OpenSeeker‑v2.

4.2 — Convergent Reasoning (Days 61‑63)
Implement the three‑path ConvergeController. Route each question through Strategic, Analytical, and Creative paths; synthesise with Synthesiser.

Validate: convergent accuracy > best single‑path by ≥ 5 pp.

4.3 — Forge, Mesh & Marketplace (Days 64‑68)
ForgeEngine: auto‑generate skills from observed workflows, publish to federated marketplace.

MeshEngine: A2A federation, federated learning with DP, secure aggregation.

MarketplaceEngine: trajectory sharing (DP ε = 1), skill publishing, outcome billing.

4.4 — Wellness (Days 69‑72)
Integrate on‑device voice (Whisper.cpp) and eye models.

Validate: Pearson’s r ≥ 0.70 with PHQ‑9, GAD‑7, MBI. Burnout early‑warning lead time ≥ 7 days.

4.5 — Mobile Brain (Days 73‑75)
Compile LFAB runtime to WASM for the PWA.

Implement CRDT sync (ElectricSQL) for offline‑first TraceDB.

Close G8: add lfab-core and lfab-sleep as dependencies of cortex-mobile.

PHASE 5: VALIDATION & HARDENING (Days 76‑90)
5.1 — Cortex Validate Integration (Days 76‑80)
Close G6: wire CortexValidate into the runtime.

Implement TraceDataExtractor with real sqlx queries against TraceDB, converting results to Polars DataFrames via Arrow.

Implement StatisticalAnalyser with Cohen’s d, bootstrap CIs, and significance tests.

Run all 12 experiments (X1‑X12) and verify they produce structured AnalysisReports.

5.2 — Security Hardening (Days 81‑84)
Run mcpsafe and mcp-bom against the Cortex MCP Gateway. Verify attack‑surface score ≤ 15.

External penetration test.

Generate OWASP MCP Top 10 compliance report and VPAT for WCAG 2.1 AA.

5.3 — Performance Testing (Days 85‑88)
CDC Mirror: sustain 250 M+ events/week with p95 latency ≤ 100 ms.

Provenance: generate 1 M capsules; verify zero Merkle failures.

Memory: profile under sustained load, verify no leaks over 24 h.

5.4 — Documentation & Compliance (Days 89‑90)
Finalise all documentation (ARCHITECTURE.md, DEPLOYMENT.md, SECURITY.md, COMPLIANCE.md).

Publish VPAT, SOC 2 readiness report, EU AI Act compliance statement.

PHASE 6: LAUNCH PREPARATION (Days 91‑100)
Day	Action
91‑92	Build release binaries for Linux x86‑64, ARM64. Apply LTO, strip, UPX. Target binary < 50 MB
93	Build Docker image, push to private registry. Generate air‑gap bundle (binary + config + Knowledge Snap templates + migrations)
94	Deploy to the first pilot customer (energy company with Maximo). Run the Trojan backup entry: "Backup your Oracle databases, browse them in a clean dashboard, restore if needed."
95‑97	Monitor pilot customer for one week. Gather feedback. Fix any issues
98	Create the marketing website, publish documentation, record demo video
99	Prepare launch announcement: press release, blog post, social media campaign
100	LAUNCH. cortex serve is production‑ready. The sovereign, verifiable, application‑absorbing enterprise AI control plane is live
PART 2 — BINARY SIZE & BUILD OPTIMISATION
The production binary must be as small as possible for ease of distribution, especially to air‑gapped environments. The literature (min‑sized‑rust, May 2026) prescribes an eight‑step optimisation sequence that collectively reduces a typical Rust debug binary from > 100 MB to < 10 MB. Cortex applies all eight.

Add the following to the root Cargo.toml:

toml
[profile.release]
opt-level = "z"           # optimise for size
lto = true                # fat link‑time optimisation
codegen-units = 1         # single codegen unit for maximum inlining
strip = "symbols"         # strip debug symbols
panic = "abort"           # no unwinding tables
Then in .cargo/config.toml:

toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-s"]   # strip all symbols at link time

[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-s"]
Build: cargo build --release --target x86_64-unknown-linux-gnu. Then: upx --best --lzma target/release/cortex. Target: < 10 MB final binary.

PART 3 — REMAINING GAPS & FINAL CLOSURE
Gap	Resolution	Phase
G1	Merge definitive workspace member list into root Cargo.toml	0
G2	Implement bootstrap_subsystems() with real initialisation calls	1
G3	Integrate all 21 Batch 15 modules into InterfaceEngine	2
G4	Pass TraceDB pool through MirrorEngine	3
G5	Add vault field to IntegrationFabric	2
G6	Wire CortexValidate into CortexRuntime	5
G7	Batch 15 modules wired into InterfaceEngine (merged with G3)	2
G8	lfab-core + lfab-sleep as dependencies of cortex-mobile	4
PART 4 — LAUNCH READINESS CHECKLIST
#	Criterion	Status
1	cargo check --workspace exits 0	Target: Phase 0
2	cargo test --workspace passes all tests	Target: Phase 1
3	cortex serve starts and accepts MCP requests	Phase 1
4	Semantic router correctly matches intents to tools	Phase 1
5	Every MCP request produces a signed TraceCaps capsule	Phase 1
6	Prompt injection is blocked (Semantic Firewall)	Phase 1
7	Kill switch (CortexGuard) works offline	Phase 1
8	Insight Engine: cross‑system NL query across ≥ 3 connectors	Phase 2
9	Backup Module: ingests .bak/.dbf/IXF files with ≥ 99.99% checksum	Phase 2
10	Observational Capture records field‑level interactions	Phase 3
11	CDC Mirror sustains 250 M+ events/week at sub‑100 ms	Phase 3
12	Absorption pipeline: 80 % workflows absorbed within 4‑6 weeks	Phase 3
13	Strangler Fig façade: users unaware of migration	Phase 3
14	Dual‑write: vendor sees normal write volumes	Phase 3
15	Deep Research: BrowseComp within 5 pp of SOTA	Phase 4
16	All 12 validation experiments (X1‑X12) pass	Phase 5
17	MCP‑BOM attack‑surface score ≤ 15	Phase 5
18	WCAG 2.1 AA pass rate = 100 % across all 18 A2UI components	Phase 5
19	Binary size ≤ 10 MB after optimisation	Phase 6
20	Air‑gap deployment tested	Phase 6
21	Documentation complete	Phase 5
22	CI/CD pipeline green	Phase 0
23	Pilot customer deployed and running	Phase 6
This is the complete launch plan.