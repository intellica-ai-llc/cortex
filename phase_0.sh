#!/bin/bash
# ============================================================
# PHASE 0 — ZERO‑ERROR WORKSPACE (single execution)
# ============================================================
set -e

echo "=== Phase 0: System deps + workspace normalisation ==="

# ----- 1. Install system libraries (one‑time) -----
sudo apt-get update -qq
sudo apt-get install -y -qq pkg-config libssl-dev libpq-dev

# ----- 2. Fix root Cargo.toml -----
cat > Cargo.toml << 'ROOTEOF'
[workspace]
resolver = "2"
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
    "crates/cortex-document-intelligence",
    "crates/lfab-core",
    "crates/lfab-sleep",
    "crates/cortex-vault",
    "crates/cortex-validate",
    "crates/cortex-self-validate",
    "crates/cortex-due-diligence",
    "crates/cortex-bench",
    "crates/cortex-publish",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "Proprietary (Core) / Apache-2.0 (Connectors)"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
async-trait = "0.1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
ed25519-dalek = { version = "2", features = ["rand_core"] }
sha2 = "0.10"
blake3 = "1"
hex = "0.4"
rand = "0.8"
rand_distr = "0.4"
regex = "1"
axum = { version = "0.7", features = ["macros"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres", "uuid", "chrono", "json"] }
polars = { version = "0.42", features = ["lazy", "describe", "ndarray"] }
arrow = { version = "52", features = ["ipc"] }
statrs = "0.17"
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
thiserror = "2"
flate2 = "0.2"
byteorder = "1"
crc = "3"
opentelemetry = { version = "0.23", features = ["metrics", "trace"] }
opentelemetry-otlp = { version = "0.16", features = ["grpc-tonic"] }
tracing-opentelemetry = "0.24"
toml = "0.8"
nalgebra = "0.33"
ROOTEOF

# ----- 3. Normalise every crate Cargo.toml -----
for crate_dir in crates/*/ ; do
    crate_name=$(basename "$crate_dir")
    cat > "${crate_dir}Cargo.toml" << CRATEOF
[package]
name = "${crate_name}"
version.workspace = true
edition.workspace = true

[dependencies]
CRATEOF
done

# Special: lfab-* crates are not under crates/
for lfab in lfab-core lfab-sleep; do
    mkdir -p "${lfab}/src"
    [ -f "${lfab}/src/lib.rs" ] || echo "pub struct Stub;" > "${lfab}/src/lib.rs"
    cat > "${lfab}/Cargo.toml" << LFABEOF
[package]
name = "${lfab}"
version.workspace = true
edition.workspace = true

[dependencies]
LFABEOF
done

# ----- 4. Add essential inter‑crate path deps + external deps -----
declare -A CRATE_DEPS
CRATE_DEPS["cortex-core"]="tokio = { workspace = true }|tracing = { workspace = true }|tracing-subscriber = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|toml = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-gateway"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|axum = { workspace = true }|rand = { workspace = true }|thiserror = { workspace = true }"
CRATE_DEPS["cortex-provenance"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|ed25519-dalek = { workspace = true }|sha2 = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }|rand = { workspace = true }"
CRATE_DEPS["cortex-security"]="cortex-core = { path = \"../cortex-core\" }|cortex-gateway = { path = \"../cortex-gateway\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|ed25519-dalek = { workspace = true }|sha2 = { workspace = true }|hex = { workspace = true }|rand = { workspace = true }|blake3 = { workspace = true }|regex = { workspace = true }"
CRATE_DEPS["cortex-guard"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|ed25519-dalek = { workspace = true }"
CRATE_DEPS["cortex-council"]="cortex-core = { path = \"../cortex-core\" }|cortex-gateway = { path = \"../cortex-gateway\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|rand = { workspace = true }|ed25519-dalek = { workspace = true }|blake3 = { workspace = true }"
CRATE_DEPS["cortex-integration"]="cortex-core = { path = \"../cortex-core\" }|cortex-gateway = { path = \"../cortex-gateway\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|reqwest = { workspace = true }|blake3 = { workspace = true }"
CRATE_DEPS["cortex-interface"]="cortex-core = { path = \"../cortex-core\" }|cortex-gateway = { path = \"../cortex-gateway\" }|cortex-council = { path = \"../cortex-council\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|reqwest = { workspace = true }"
CRATE_DEPS["cortex-tracedb"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|sqlx = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-mirror"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|serde_yaml = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }|sqlx = { workspace = true }|reqwest = { workspace = true }"
CRATE_DEPS["cortex-absorb"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-mirror = { path = \"../cortex-mirror\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|sqlx = { workspace = true }"
CRATE_DEPS["cortex-genesis"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-interface = { path = \"../cortex-interface\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-replace"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-interface = { path = \"../cortex-interface\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|chrono = { workspace = true }|uuid = { workspace = true }"
CRATE_DEPS["cortex-retire"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|chrono = { workspace = true }|uuid = { workspace = true }|ed25519-dalek = { workspace = true }|sha2 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-pulse"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|chrono = { workspace = true }|nalgebra = { workspace = true }"
CRATE_DEPS["cortex-whisper"]="cortex-core = { path = \"../cortex-core\" }|cortex-pulse = { path = \"../cortex-pulse\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-deep-research"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|rand = { workspace = true }"
CRATE_DEPS["cortex-coggen"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-iter-research"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-rl-bootstrapper"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|rand = { workspace = true }"
CRATE_DEPS["cortex-research-swarm"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|rand = { workspace = true }"
CRATE_DEPS["cortex-aat"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|ed25519-dalek = { workspace = true }|sha2 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-converge"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-forge"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-mesh"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-mobile"]="cortex-core = { path = \"../cortex-core\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|lfab-core = { path = \"../lfab-core\" }|lfab-sleep = { path = \"../lfab-sleep\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }"
CRATE_DEPS["cortex-knowledge-snap"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-distribution"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|ed25519-dalek = { workspace = true }|sha2 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-onboarding"]="cortex-core = { path = \"../cortex-core\" }|cortex-knowledge-snap = { path = \"../cortex-knowledge-snap\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-testing"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|rand = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-marketplace"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-cli"]="cortex-core = { path = \"../cortex-core\" }|cortex-gateway = { path = \"../cortex-gateway\" }|cortex-provenance = { path = \"../cortex-provenance\" }|cortex-security = { path = \"../cortex-security\" }|tokio = { workspace = true }|clap = { workspace = true }|tracing = { workspace = true }|tracing-subscriber = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }"
CRATE_DEPS["cortex-observability"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|tracing-opentelemetry = { workspace = true }|opentelemetry = { workspace = true }|opentelemetry-otlp = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-dream"]="cortex-core = { path = \"../cortex-core\" }|cortex-memory = { path = \"../cortex-memory\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|ed25519-dalek = { workspace = true }|sha2 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-memory"]="cortex-core = { path = \"../cortex-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|sha2 = { workspace = true }|hex = { workspace = true }|blake3 = { workspace = true }"
CRATE_DEPS["cortex-intelligence"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|reqwest = { workspace = true }"
CRATE_DEPS["cortex-document-intelligence"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-vault"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }|flate2 = { workspace = true }|sha2 = { workspace = true }|byteorder = { workspace = true }|crc = { workspace = true }"
CRATE_DEPS["cortex-validate"]="cortex-core = { path = \"../cortex-core\" }|cortex-provenance = { path = \"../cortex-provenance\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|cortex-gateway = { path = \"../cortex-gateway\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|serde_yaml = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|polars = { workspace = true }|arrow = { workspace = true }|statrs = { workspace = true }|rand = { workspace = true }|rand_distr = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-self-validate"]="cortex-validate = { path = \"../cortex-validate\" }|cortex-provenance = { path = \"../cortex-provenance\" }|cortex-tracedb = { path = \"../cortex-tracedb\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-due-diligence"]="cortex-self-validate = { path = \"../cortex-self-validate\" }|cortex-provenance = { path = \"../cortex-provenance\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|serde_yaml = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["cortex-bench"]="cortex-validate = { path = \"../cortex-validate\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|async-trait = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }"
CRATE_DEPS["cortex-publish"]="cortex-validate = { path = \"../cortex-validate\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|uuid = { workspace = true }|chrono = { workspace = true }|blake3 = { workspace = true }|hex = { workspace = true }"
CRATE_DEPS["lfab-core"]="tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|chrono = { workspace = true }|uuid = { workspace = true }"
CRATE_DEPS["lfab-sleep"]="lfab-core = { path = \"../lfab-core\" }|tokio = { workspace = true }|serde = { workspace = true }|serde_json = { workspace = true }|tracing = { workspace = true }|chrono = { workspace = true }|uuid = { workspace = true }"

# ----- 5. Write deps into each crate Cargo.toml -----
for crate_dir in crates/*/; do
    crate_name=$(basename "$crate_dir")
    deps="${CRATE_DEPS[$crate_name]}"
    if [ -n "$deps" ]; then
        IFS='|' read -ra DEP_ARRAY <<< "$deps"
        for dep in "${DEP_ARRAY[@]}"; do
            echo "$dep" >> "${crate_dir}Cargo.toml"
        done
    fi
done

# lfab crates
for lfab in lfab-core lfab-sleep; do
    deps="${CRATE_DEPS[$lfab]}"
    if [ -n "$deps" ]; then
        IFS='|' read -ra DEP_ARRAY <<< "$deps"
        for dep in "${DEP_ARRAY[@]}"; do
            echo "$dep" >> "${lfab}/Cargo.toml"
        done
    fi
done

# ----- 6. Ensure every crate has a minimal lib.rs -----
for dir in crates/*/src/; do
    [ -f "${dir}lib.rs" ] || echo "pub struct Stub;" > "${dir}lib.rs"
done
for dir in lfab-core/src lfab-sleep/src; do
    mkdir -p "$dir"
    [ -f "${dir}/lib.rs" ] || echo "pub struct Stub;" > "${dir}/lib.rs"
done

# ----- 7. Add agents/mod.rs if missing -----
[ -f crates/cortex-council/src/agents/mod.rs ] || {
    mkdir -p crates/cortex-council/src/agents
    echo "// auto-generated stub" > crates/cortex-council/src/agents/mod.rs
}

# ----- 8. Add role_extractors/mod.rs if missing -----
[ -f crates/cortex-integration/src/role_extractors/mod.rs ] || {
    mkdir -p crates/cortex-integration/src/role_extractors
    echo "// auto-generated stub" > crates/cortex-integration/src/role_extractors/mod.rs
}

# ----- 9. First compile -----
echo ""
echo "=== Running: cargo check --workspace ==="
cargo check --workspace 2>&1 | tee check.log

ERROR_COUNT=$(grep -c "^error" check.log || true)
if [ "$ERROR_COUNT" -eq 0 ]; then
    echo ""
    echo "✅ Phase 0 complete — 0 errors, workspace compiles cleanly."
    echo "   Ready for Phase 1: git add -A && git commit"
else
    echo ""
    echo "⚠️  ${ERROR_COUNT} errors found.  Review check.log and run:"
    echo "   cargo check --workspace 2>&1 | grep 'error\['"
    echo "   (most remaining errors are missing module files or type mismatches)"
fi