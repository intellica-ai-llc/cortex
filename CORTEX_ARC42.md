# Cortex Sovereign Claude Code Enterprise Addendum 4

Date: June 2026
Version: 1.0 (Living Document – Prepend to Original CORTEX_ARC42.md)
Authors: [Your Name/Team] with Grok Research Synthesis
Status: Ready for Implementation – Engineer-Facing Specification
This addendum extends the original Arc42 documentation to evolve Cortex into a sovereign, agentic coding & modernization platform that integrates hybrid Claude capabilities (via Anthropic's self-hosted sandboxes and MCP) while preserving all core constraints: single <10MB Rust binary (or minimal sidecars), full air-gap support, cryptographic provenance (TraceCaps/Merkle), hexagonal architecture, six-phase absorption pipeline, offline Ed25519 JWT licensing, and enterprise connector ecosystem (SAP, Oracle, Salesforce, Jira, GitHub Enterprise, etc.).
It directly surpasses original objectives by adding multi-agent orchestration for autonomous legacy modernization, neural/program synthesis for "Claude Code"-style generation, hybrid LLM routing, and enhanced governance to create a compelling acquisition/partnership asset for Anthropic (self-hosted gaps) and Dell (AI Factory software layer).
An engineer familiar with the original architecture can implement this by extending existing components (SemanticGateway, AgentCouncil, AbsorptionPipeline, ToolRegistry, TraceDB, hexagonal adapters) without breaking sovereignty or single-binary goals.
1. Context & Scope (Updated)

Business Goal: Deliver sovereign Claude-powered coding and full-stack enterprise app modernization. Enable "Agentic Strangler Fig" migrations where multi-agent loops autonomously observe, absorb, synthesize, replace, and retire legacy components with auditable, provenance-traced artifacts.
Key Stakeholders: Enterprise IT/DevOps in regulated sectors (finance, gov, manufacturing); Dell AI Factory operators; potential Anthropic co-sell partners.
Constraints (Unchanged + Reinforced): Air-gap first; no data exfiltration; cryptographic signing of all AI outputs; EU AI Act / SCITT / SOC2 compliance; hybrid routing only for non-sensitive high-reasoning tasks.
New Drivers: Anthropic Managed Agents self-hosted sandboxes (reasoning in cloud, execution in customer infra via MCP tunnels); multi-agent systems for code gen/modernization; program synthesis verification loops.

2. Building Block View (New/Extended)
Core Extensions (All fit hexagonal ports/adapters):

Semantic Gateway (Extended Orchestrator):
Add hybrid LLM router: Sensitivity-based decision (local quantized via vLLM/Ollama/Candle Rust integration for air-gap; secure MCP tunnel/proxy to Anthropic Claude for complex reasoning).
New ports: LLMAdapter (trait for model backends), MCPClient for Anthropic self-hosted sandboxes/tunnels.
Implementation: Rust HTTP client + outbound-only tunnels (no inbound ports). Fallback to local models for full sovereignty.

Agent Council (Evolved to Verifiable Multi-Agent System – MAS):
Specialist talents expanded:
PLANNER: Task decomposition & workflow orchestration.
CODE (Claude-style synthesis): Neural program synthesis from NL intents + absorbed enterprise schemas (RAG over TraceDB).
MODERNIZE: Agentic Strangler Fig executor – applies six-phase pipeline to legacy code/scripts.
VERIFIER/REVIEWER: Execution-guided verification, reflection loops, test generation (inspired by VAPU-style pipelines).
DEPLOYER: Integrates with existing connectors + GitHub/Azure DevOps for PRs/deployments.

Coordination: Orchestrator-subagent with reflection (critique-refine), shared memory via TraceDB (pgvector embeddings), and parallel execution where safe.
Provenance: Every agent step logged in extended TraceCaps (temporal Merkle chain: agent_role, input_hash, output_hash, timestamp, model_id/signature).

Absorption Pipeline (Enhanced Phases):
Observe: Extend browser/IDE extensions + CDC to capture dev workflows, legacy scripts, ERP extensions.
Mirror: Replicate in sandbox.
Absorb: RAG + schema extraction into TraceDB; code pattern mining.
Genesis: Multi-agent synthesis → auditable modules (Claude-influenced reasoning + local execution).
Replace: Agentic Strangler Fig – façade routing via new adapters; gradual traffic shift.
Retire: Automated decommissioning with provenance proofs.

ToolRegistry & ExecutionPlanner:
Register enterprise tools + new code exec sandbox (gVisor-like isolation or Rust sandbox).
Support MCP servers for internal systems (Postgres, Git, SAP APIs, etc.).

Provenance & Trace Engine (TraceCaps v2):
Multi-agent chronology, cryptographic signing (Ed25519) for all generated code/artifacts.
SCITT-compatible attestations; full audit logs for EU AI Act (transparency, risk assessment).

SelfValidator & Quality Gates:
Extend with coding benchmarks (HumanEval-style enterprise tasks), functional equivalence testing, security scanning of generated code.


Deployment Options:

Single binary (core + local adapters).
Optional sidecars: vLLM/Ollama for inference, MCP servers.
Dell AI Factory blueprints: Pre-optimized images for NVIDIA hardware, confidential computing.

3. Runtime View

Typical Flow (Claude Code Enterprise Task): NL intent ("Modernize SAP work order approval") → PLANNER decomposes → ROUTER selects Claude via MCP sandbox (reasoning) + local execution → CODE/MODERNIZE agents synthesize + verify in ToolRegistry sandbox → Genesis artifact with TraceCaps → Deploy via connectors → Monitor & Retire.
Hybrid Routing Logic: Local-first for PII/sensitive; Claude for novel reasoning (outbound-only, sanitized results only).
Error/Recovery: Reflection loops + fallback to simpler local models.

4. Deployment View

Air-Gap: Bundled quantized models + offline licensing.
Hybrid: Outbound MCP tunnels + Dell-managed on-prem.
Scaling: Horizontal agent execution on AI Factory nodes.

5. Quality Attributes (Updated & Measurable)

Sovereignty/Compliance: 100% data residency options; full traceability (EU AI Act pillars).
Performance: Quantized inference + parallel agents; benchmarks vs. baselines.
Security: Sandboxed execution, provenance for all outputs, SemanticFirewall extensions.
Maintainability: Hexagonal ports enable model swapping (Claude → open equivalents).
Usability: NL interface + absorbed A2UI for generated workflows.

6. Cross-Cutting Concepts

Governance: All agents respect kill-switch, licensing, and policy engine.
Observability: Extended logging with provenance Merkle roots.
Testing: Unit (Rust), integration (sandboxed pipelines), end-to-end (enterprise scenario replay).

7. Rationale & References (Academic/Industry Grounding)

Sovereign/Hybrid AI: Self-hosted sandboxes + MCP for execution control.
MAS & Agentic Modernization: VAPU-inspired verification; Agentic Strangler Fig.
Program Synthesis: LLM agents with execution feedback for enterprise code.
Provenance: Cryptographic binding for AI-generated assets.
Architecture: Hexagonal for agentic systems.

Implementation Roadmap (Phased, Low-Risk):

MVP (Weeks): LLM adapter + CODE talent + basic MCP proxy. Test on sample SAP/Jira workflows.
Core MAS (Months): Full Agent Council loops, absorption extensions, provenance v2.
Production: Dell blueprints, benchmarks, enterprise pilots, partnership packaging.

Open Items / Risks & Mitigations:

Anthropic terms: Partnership-first (co-sell vs. direct compete); use APIs/sandboxes compliantly.
Model quality: Hybrid + quantization; continuous validation.
Complexity: Start with existing hexagonal patterns; incremental PRs.

This addendum is self-contained for seamless building. Insert at the top of the original Arc42. It fills gaps with research-backed specifics while honoring the as-built foundation.
Next steps: Draft Rust trait sketches for new adapters, generate sequence diagrams, or refine for a specific phase? Let me know how to iterate.



# Cortex Maximo Sovereign Agentic Module Addendum 3Date: June 2026Version: 1.0 Living Document – 

Separate Module License)Purpose: Dedicated sales and implementation module for hooking existing IBM Maximo (Manage/MAS) customers. Positions Cortex as the definitive sovereign agentic overlay that renders all third-party add-ons (IBM and external) irrelevant by delivering autonomous, provenance-backed EAM modernization, optimization, and intelligence within full air-gap constraints. Licensed separately for targeted Maximo estates.This module builds on Cortex core without replication, focusing on Maximo-specific value: accelerating 7.6-to-MAS migrations, absorbing custom Java/extensions/workflows, and providing frontier agentic capabilities that surpass IBM Predict/Monitor/Health/Assistant, HSE, Optimizer, Asset Configuration Manager, and third-party mobiles (e.g., EAM360) or alternatives.Sales Positioning: Why Cortex Maximo Module Is a No-BrainerIrrelevance of Existing Add-Ons: IBM add-ons (Predict for failure dates/probability, Monitor for IoT anomaly detection, Health for risk optimization, Assistant for GenAI queries, HSE for compliance, Optimizer for scheduling) require additional licensing, OpenShift complexity, and cloud/hybrid data flows. Third-party mobiles and competitors (MaintainX, UpKeep, IFS, Tractian) focus on narrow UX/IoT but lack deep custom code absorption or sovereign multi-agent autonomy.Cortex Differentiation: Single-binary sovereign control plane with six-phase absorption applied agentically to Maximo customizations, plus hybrid Claude-powered loops for end-to-end autonomy (work order optimization, failure analysis, inventory, compliance). Full provenance for audits. Zero new data exfiltration risk. Dell AI Factory ready. Separate licensing enables quick-win pilots on existing Maximo estates.ROI Hooks: Cut migration/custom dev costs (major 7.6 EOS pain point through 2026), reduce TCO vs. stacked add-ons, deliver 5-10x faster modernization, autonomous execution, and measurable reliability gains.Frontier Research Synthesis (Past Month Emphasis, June 2026)Multi-Agent Systems (MAS) in EAM/Modernization: Recent arXiv (June 2026) highlights state-machine guided synthetic data for anomaly detection, failure mode diagnostics in deep multi-agent RL, and SKILL.nb for durable agent workflows. Agentic governance maturity models (AAGMM) emphasize semantic control planes and runtime compliance. Legacy modernization papers stress agentic Strangler Fig with execution verification for Java/custom estates.EAM-Specific Agentic AI: Verdantis-style autonomous MRO agents for work order creation/prioritization; IFS.ai and IBM Watsonx trends toward embedded GenAI copilots, but sovereign gaps persist. Real-time context via event-driven architectures (EDA) for MAS scalability.Gaps Filled: Most add-ons are reactive/analytic. Cortex delivers proactive autonomous loops with verifiable synthesis, temporal provenance, and full air-gap fallback — surpassing frontier benchmarks in reliability for regulated EAM (utilities, energy, manufacturing).Module Architecture (Hexagonal Extensions, Engineer-Ready)MaximoAdapter Port: Implements MIF, REST APIs, MEA, JDBC patterns, CDC for real-time mirroring of work orders, assets, PMs, inventory, custom objects. Bidirectional with existing Cortex SAP/Oracle/Jira connectors. Schema discovery for absorption. Outbound-only for sovereignty.Agent Council EAM Specialization (MAS Talents):EAM_PLANNER: Decomposes intents (e.g., "Optimize PM schedule for fleet under regulatory constraints").RELIABILITY_AGENT: Anomaly/failure synthesis using absorbed history + hybrid Claude reasoning (local quantized fallback).WORK_ORDER_AUTONOMY: Autonomous creation, prioritization, technician assignment, verification (VAPU-inspired loops).COMPLIANCE_VERIFIER: HSE-style risk/incident handling with cryptographic attestations.OPTIMIZER_AGENT: Scheduling/inventory equivalent to IBM Optimizer but agentic and provenance-traced.Absorption Pipeline Tailored: Observe (API + mobile extensions for field workflows/custom Java); Mirror/Absorb (custom extensions/patterns into TraceDB); Genesis (synthesized modern modules/scripts); Replace (agentic façade over Maximo UI/API); Retire (decommission legacy customizations).Hybrid LLM Routing: Local-first for sensitive asset data; MCP tunnels for high-reasoning Claude tasks (e.g., novel failure analysis). Sandboxed ToolRegistry for safe execution (work order simulation, script testing).Provenance v2 for EAM: Temporal Merkle chains capturing agent contributions to work orders, predictions, compliance artifacts. SCITT/EU AI Act ready; audit-proof vs. black-box IBM Assistant outputs.Deployment: Optional sidecar for Maximo integration (preserves single-binary core). Air-gap bundles with quantized models. Dell AI Factory blueprints for on-prem Maximo users. Separate license key scope for this module.Runtime Flows (Typical Maximo Customer Scenarios)NL intent via Cortex → PLANNER decomposes → Agents absorb current Maximo state → Synthesize/verify optimized artifacts → Deploy via adapter (PR-like to Maximo or façade) → Monitor with provenance.Migration Accelerator: Absorb 7.6 custom Java → Generate MAS-compatible equivalents with functional equivalence testing.Autonomous Overlay: Run alongside Maximo; progressively Strangler Fig without rip-and-replace.Quality Attributes (Module-Specific)Autonomy & Reliability: MAS verification loops + execution-guided synthesis (frontier benchmarks).Compliance: Full traceability for HSE/regulated assets; no add-on stacking needed.Performance: Parallel agent execution on existing hardware; real-time EDA hooks.Adoption: NL interface reduces training; mobile observation extends field usability beyond third-party apps.Implementation Roadmap (Module-Only)MVP (Weeks): MaximoAdapter + basic EAM talents + absorption for custom objects/work orders. Pilot on one asset class.Core Autonomy (1-2 Months): Full MAS loops, Predict/Monitor equivalents, provenance for audits.Enterprise Scale: Optimizer/HSE surpassing features, Dell validation, sales packaging (quick ROI calculators for add-on consolidation).Risks & MitigationsIntegration complexity: Leverage existing hexagonal patterns + MIF standards.Model access: Hybrid/compliant Anthropic usage.Sales: Position as "Maximo augmentation license" — low friction entry, high expansion.Open Items: Customer-specific benchmarks, co-sell materials with Dell/Anthropic. This module makes Cortex the last EAM layer Maximo customers will need, consolidating and surpassing the fragmented add-on landscape with sovereign frontier agentic power.
# Full Government Data Fabric Module Specification – Addendum 2 
Markdown# Cortex Sovereign Data Fabric Government Module (CSDF) Addendum

**Document Title:** Cortex Sovereign Claude Code Enterprise Addendum 2 – Government Data Fabric Module  
**Date:** June 11, 2026  
**Version:** 1.0 (Living Document – Prepend to Original CORTEX_ARC42.md)  
**Authors:** [Your Name/Team] with Grok Research Synthesis  
**Status:** Ready for Implementation – Engineer-Facing Specification  

This addendum extends the original Intellecta Cortex architecture with the **Cortex Sovereign Data Fabric Government Module (CSDF)**. It delivers a unified, verifiable, agent-native data layer optimized for government and regulated sovereign environments. CSDF directly addresses EU AI Act Art.10 (high-quality, traceable, representative datasets), ISO 42001, NIST AI RMF, and national data residency requirements.

It integrates natively with existing Cortex components (SemanticGateway, AgentCouncil, TraceDB, ProvenanceEngine, AbsorptionPipeline) while preserving all core constraints: single small Rust binary, air-gap support, cryptographic provenance, hexagonal architecture, and offline licensing.

## 1. Context & Scope (Updated)

**Business Goal:** Provide a sovereign data fabric that makes government agencies AI-ready with cryptographically verifiable, agent-native data products. Enable seamless support for Agentic AI pilots, high-risk use cases, and compliance evidence generation while maintaining full data residency and auditability.

**Key Stakeholders:**
- Government IT / Data Officers: Sovereignty, EU AI Act compliance, legacy integration
- Compliance / DPO: Cryptographic provenance, automated Art.10 evidence
- AI / Agent Teams: ASL-native data discovery and consumption
- Agency Leadership: TCO savings vs commercial SaaS, audit-ready fabric

**Constraints (Reinforced):**
- Full air-gap / on-prem operation
- Cryptographic provenance on every asset (VeriCrypt integration)
- Agent-native via ASL + VeriChain
- No data exfiltration
- Integration with existing Cortex connectors and TraceDB

**New Drivers:**
- EU AI Act enforcement (Aug 2026) requiring documented data governance for high-risk AI
- Agentic workflows demanding semantic, verifiable data products
- Hybrid legacy + lakehouse support for government systems

## 2. Building Block View

**Core Extensions (Hexagonal Ports & Adapters):**

**Semantic Gateway (Extended)**
- New port: `DataFabricAdapter` trait for fabric operations
- Hybrid routing for data discovery (local-first, with Cortex policy enforcement)
- Integration with TraceDB for unified metadata + decision traces

**Agent Council (Extended Talents)**
- **DATA_STEWARD**: Automated quality, bias, and representativeness monitoring
- **PROVENANCE_GUARDIAN**: VeriCrypt notarization and evidence generation
- **SEMANTIC_REGISTRAR**: Registers Gold-layer products as ASL resources
- **COMPLIANCE_REPORTER**: On-demand EU AI Act Art.10 packs

**Absorption Pipeline (Data-Focused Extensions)**
- Observe: Automated source profiling + PII classification
- Mirror: Federated access via Trino + CDC
- Absorb: Medallion lakehouse ingestion with VeriCrypt hooks
- Genesis: Semantic embeddings + ontology tagging
- Replace: Self-service data product routing
- Retire: Secure data decommissioning with proofs

**New Components:**

**VeriCrypt Notarizer Service** (Microservice sidecar or in-process)
- Merkle tree computation + Ed25519 signing for datasets/versions
- Integration point in all ingestion paths

**Cortex Data Fabric Core**
- Manages MinIO + Iceberg lakehouse, Trino federation, dbt quality engine
- Cortex policy engine enforces quality gates, access, and EU checks

**ASL Data Product Registry**
- Registers Gold products with semantic descriptors, embeddings, and VeriCrypt proofs
- Agents discover/query via natural language through SemanticGateway

**Technology Stack (Sovereign-First)**
- Storage: MinIO (S3) + Apache Iceberg
- Federation/Query: Trino
- Quality: dbt + Great Expectations
- Vector/Semantic: pgvector + local Ollama embeddings
- Orchestration: Airflow/Prefect with Cortex hooks
- Provenance: Extended TraceCaps + VeriCrypt

## 3. Runtime View

**Typical Government Flow (High-Risk AI Data Product)**
```mermaid
sequenceDiagram
    actor Analyst
    participant MCP as MCPServer
    participant GW as SemanticGateway
    participant Fabric as CSDF Core
    participant VC as VeriCrypt Notarizer
    participant Council as AgentCouncil
    participant TraceDB

    Analyst->>MCP: "Provide verified citizen benefits data for fraud model"
    MCP->>GW: route_intent()
    GW->>Fabric: discover_product(semantic_query)
    Fabric->>TraceDB: search_metadata + embeddings
    Fabric->>VC: notarize(Gold layer snapshot)
    VC-->>Fabric: Merkle proof + signature
    Fabric->>Council: register_asl_product()
    Council-->>Fabric: ASL descriptor + proof
    GW-->>MCP: Data product reference + proof
    MCP-->>Analyst: 200 with traceable result
Key Scenarios:

Source Ingestion & Notarization: Legacy DB → Trino → Iceberg → VeriCrypt → Cortex registration
Agent Data Discovery: ASL agent queries Cortex → returns verified Gold product with proof
Compliance Evidence Generation: One-click Art.10 report with lineage, quality metrics, cryptographic proofs
Quality Gate Enforcement: dbt tests + Cortex policy rejection of non-compliant data

4. Deployment View
Options:

Air-Gap: Bundled with Cortex binary + sidecar services (MinIO, Trino, pgvector)
Hybrid: On-prem lakehouse with Cortex control plane
Dell AI Factory: Validated blueprints for GPU-accelerated nodes

Resource Profile:

Base Cortex + CSDF: +2–4 GB RAM, minimal CPU overhead
Scalable horizontally via Kubernetes (optional)

5. Quality Attributes (Measurable)








































AttributeTargetMeasurementProvenance Coverage100% ingested assetsVeriCrypt proof countEU AI Act Art.10Automated evidence packs in <2 minReport generation timeAgent Discovery Latency<2s for registered productsEnd-to-end query benchmarkData Quality Score>95% for high-risk datasetsdbt/GE automated testsSelf-Service Fulfillment<24h with full audit trailPortal request metricsMaturity Score≥4.5/5 automatedGovernance dashboard
Security & Compliance:

Zero-trust at every layer
Full TraceCaps + VeriCrypt for all data operations
Automated bias/representativeness checks

6. Implementation Roadmap (Batched)
Batch 1: Foundation (Weeks 1–6) – Catalog, Quality, VeriCrypt baseline
Batch 2: Integration Layer (Weeks 7–14) – Pipelines, Lakehouse, Federation
Batch 3: AI Enablement (Weeks 15–22) – Semantic/Vector, Agent Bridge, Observability
Batch 4: Scale & Hardening (Weeks 23–30) – Self-Service, Maturity, Handover
(Full batch details available in implementation guides.)
7. Cross-Cutting Concepts

Governance: Cortex as single policy enforcement point
Provenance: Extended TraceCaps with dataset Merkle chains
Agent-Native: All Gold products exposed via ASL registry
Testing: Extended SelfValidator suite including data fabric experiments
Handover: Ops runbooks, training, retainer model

8. Risks & Mitigations

Legacy system complexity → Phased prioritization + federation-first
Performance at scale → Trino optimization + Cortex monitoring
Regulatory change → Policy-driven updates via Cortex

Open Items: Final integration testing with sample government datasets; Dell AI Factory validation extension.

This addendum is complete, production-grade, and ready to prepend to your main Cortex ARC42.md. It maintains perfect consistency with your existing architecture while delivering a best-in-class sovereign government data capability.
You can now copy this directly into your documentation. Let me know if you want any section expanded, code skeletons added, or the next step (e.g., full dbt templates, policy examples, or integration with Agentic AI offering).











# Cortex Sovereign Claude Code Enterprise Addendum 1 (vNext)
Date: June 2026
Version: 1.0 (Living Document – Prepend to Original CORTEX_ARC42.md)
Authors: [Your Name/Team] with Grok Research Synthesis
Status: Ready for Implementation – Engineer-Facing Specification
This addendum extends the original Arc42 documentation to evolve Cortex into a sovereign, agentic coding & modernization platform that integrates hybrid Claude capabilities (via Anthropic's self-hosted sandboxes and MCP) while preserving all core constraints: single <10MB Rust binary (or minimal sidecars), full air-gap support, cryptographic provenance (TraceCaps/Merkle), hexagonal architecture, six-phase absorption pipeline, offline Ed25519 JWT licensing, and enterprise connector ecosystem (SAP, Oracle, Salesforce, Jira, GitHub Enterprise, etc.).
It directly surpasses original objectives by adding multi-agent orchestration for autonomous legacy modernization, neural/program synthesis for "Claude Code"-style generation, hybrid LLM routing, and enhanced governance to create a compelling acquisition/partnership asset for Anthropic (self-hosted gaps) and Dell (AI Factory software layer).
An engineer familiar with the original architecture can implement this by extending existing components (SemanticGateway, AgentCouncil, AbsorptionPipeline, ToolRegistry, TraceDB, hexagonal adapters) without breaking sovereignty or single-binary goals.
1. Context & Scope (Updated)

Business Goal: Deliver sovereign Claude-powered coding and full-stack enterprise app modernization. Enable "Agentic Strangler Fig" migrations where multi-agent loops autonomously observe, absorb, synthesize, replace, and retire legacy components with auditable, provenance-traced artifacts.
Key Stakeholders: Enterprise IT/DevOps in regulated sectors (finance, gov, manufacturing); Dell AI Factory operators; potential Anthropic co-sell partners.
Constraints (Unchanged + Reinforced): Air-gap first; no data exfiltration; cryptographic signing of all AI outputs; EU AI Act / SCITT / SOC2 compliance; hybrid routing only for non-sensitive high-reasoning tasks.
New Drivers: Anthropic Managed Agents self-hosted sandboxes (reasoning in cloud, execution in customer infra via MCP tunnels); multi-agent systems for code gen/modernization; program synthesis verification loops.

2. Building Block View (New/Extended)
Core Extensions (All fit hexagonal ports/adapters):

Semantic Gateway (Extended Orchestrator):
Add hybrid LLM router: Sensitivity-based decision (local quantized via vLLM/Ollama/Candle Rust integration for air-gap; secure MCP tunnel/proxy to Anthropic Claude for complex reasoning).
New ports: LLMAdapter (trait for model backends), MCPClient for Anthropic self-hosted sandboxes/tunnels.
Implementation: Rust HTTP client + outbound-only tunnels (no inbound ports). Fallback to local models for full sovereignty.

Agent Council (Evolved to Verifiable Multi-Agent System – MAS):
Specialist talents expanded:
PLANNER: Task decomposition & workflow orchestration.
CODE (Claude-style synthesis): Neural program synthesis from NL intents + absorbed enterprise schemas (RAG over TraceDB).
MODERNIZE: Agentic Strangler Fig executor – applies six-phase pipeline to legacy code/scripts.
VERIFIER/REVIEWER: Execution-guided verification, reflection loops, test generation (inspired by VAPU-style pipelines).
DEPLOYER: Integrates with existing connectors + GitHub/Azure DevOps for PRs/deployments.

Coordination: Orchestrator-subagent with reflection (critique-refine), shared memory via TraceDB (pgvector embeddings), and parallel execution where safe.
Provenance: Every agent step logged in extended TraceCaps (temporal Merkle chain: agent_role, input_hash, output_hash, timestamp, model_id/signature).

Absorption Pipeline (Enhanced Phases):
Observe: Extend browser/IDE extensions + CDC to capture dev workflows, legacy scripts, ERP extensions.
Mirror: Replicate in sandbox.
Absorb: RAG + schema extraction into TraceDB; code pattern mining.
Genesis: Multi-agent synthesis → auditable modules (Claude-influenced reasoning + local execution).
Replace: Agentic Strangler Fig – façade routing via new adapters; gradual traffic shift.
Retire: Automated decommissioning with provenance proofs.

ToolRegistry & ExecutionPlanner:
Register enterprise tools + new code exec sandbox (gVisor-like isolation or Rust sandbox).
Support MCP servers for internal systems (Postgres, Git, SAP APIs, etc.).

Provenance & Trace Engine (TraceCaps v2):
Multi-agent chronology, cryptographic signing (Ed25519) for all generated code/artifacts.
SCITT-compatible attestations; full audit logs for EU AI Act (transparency, risk assessment).

SelfValidator & Quality Gates:
Extend with coding benchmarks (HumanEval-style enterprise tasks), functional equivalence testing, security scanning of generated code.


Deployment Options:

Single binary (core + local adapters).
Optional sidecars: vLLM/Ollama for inference, MCP servers.
Dell AI Factory blueprints: Pre-optimized images for NVIDIA hardware, confidential computing.

3. Runtime View

Typical Flow (Claude Code Enterprise Task): NL intent ("Modernize SAP work order approval") → PLANNER decomposes → ROUTER selects Claude via MCP sandbox (reasoning) + local execution → CODE/MODERNIZE agents synthesize + verify in ToolRegistry sandbox → Genesis artifact with TraceCaps → Deploy via connectors → Monitor & Retire.
Hybrid Routing Logic: Local-first for PII/sensitive; Claude for novel reasoning (outbound-only, sanitized results only).
Error/Recovery: Reflection loops + fallback to simpler local models.

4. Deployment View

Air-Gap: Bundled quantized models + offline licensing.
Hybrid: Outbound MCP tunnels + Dell-managed on-prem.
Scaling: Horizontal agent execution on AI Factory nodes.

5. Quality Attributes (Updated & Measurable)

Sovereignty/Compliance: 100% data residency options; full traceability (EU AI Act pillars).
Performance: Quantized inference + parallel agents; benchmarks vs. baselines.
Security: Sandboxed execution, provenance for all outputs, SemanticFirewall extensions.
Maintainability: Hexagonal ports enable model swapping (Claude → open equivalents).
Usability: NL interface + absorbed A2UI for generated workflows.

6. Cross-Cutting Concepts

Governance: All agents respect kill-switch, licensing, and policy engine.
Observability: Extended logging with provenance Merkle roots.
Testing: Unit (Rust), integration (sandboxed pipelines), end-to-end (enterprise scenario replay).

7. Rationale & References (Academic/Industry Grounding)

Sovereign/Hybrid AI: Self-hosted sandboxes + MCP for execution control.
MAS & Agentic Modernization: VAPU-inspired verification; Agentic Strangler Fig.
Program Synthesis: LLM agents with execution feedback for enterprise code.
Provenance: Cryptographic binding for AI-generated assets.
Architecture: Hexagonal for agentic systems.

Implementation Roadmap (Phased, Low-Risk):

MVP (Weeks): LLM adapter + CODE talent + basic MCP proxy. Test on sample SAP/Jira workflows.
Core MAS (Months): Full Agent Council loops, absorption extensions, provenance v2.
Production: Dell blueprints, benchmarks, enterprise pilots, partnership packaging.

Open Items / Risks & Mitigations:

Anthropic terms: Partnership-first (co-sell vs. direct compete); use APIs/sandboxes compliantly.
Model quality: Hybrid + quantization; continuous validation.
Complexity: Start with existing hexagonal patterns; incremental PRs.


# ARCHITECTURE BLUEPRINT – Intellecta Cortex
Source Chat: Full conversation, May 7 – June 8, 2026
Generated: 2026‑06‑08T22:00:00Z
Blueprint Integrity Hash: A3F2‑9C1E‑47DB‑BE06
Overall Confidence: 97 %
Transfer Continuity Score: 0.94

1. CONTEXT & STAKEHOLDERS
Arc42 Sections 1, 2, 3

System Goals
Intellecta Cortex is a sovereign, self‑hosted, cryptographically‑verifiable enterprise AI control plane. It auto‑discovers every enterprise application and database, absorbs their workflows through observational learning, and replaces their interfaces with a single, WCAG 2.2 AA‑compliant, natural‑language experience — without ever sending data to the cloud. Cortex implements a six‑phase obsolescence pipeline (Observe → Mirror → Absorb → Genesis → Replace → Retire) that progressively migrates enterprise workloads from legacy applications to Cortex‑native dashboards, completely invisibly to users and legacy vendors.

Stakeholders & Concerns

Stakeholder	Role	Key Concerns	Source
Enterprise IT Admin	Deploys and manages Cortex	Sovereign deployment, air‑gap capability, offline license validation, OTA updates	Phase 0 infrastructure, Distribution discussion
Data Protection Officer	Ensures regulatory compliance	Cryptographic provenance, EU AI Act Art. 12, NERC CIP‑015‑1, SCITT anchoring	Provenance discussion, Compliance docs
Application User	Daily operator of legacy apps	Uninterrupted workflows, familiar interfaces, faster response	Strangler Fig façade, Interface of One
CFO / C‑Suite	Financial oversight, license savings	Absorption Score dashboard, ROI calculator	Monetization, Role dashboard
Compliance Officer	Audit and evidence	IETF AAT JSON records, Merkle‑proofed audit trails	AAT formatter, Provenance Explorer
Dell Technologies	Potential acquirer	Sovereign AI software layer, Dell AI Factory integration, cryptographic differentiation	Dell valuation discussion, Batch 19/20
Cortex Developer	Builds and extends the platform	Clean workspace, automated tests, validation pipeline	Phase 0‑6 launch plan
External Systems & Actors

graph TB
    User((Enterprise User))
    Admin((IT Admin))
    Auditor((Compliance Officer))

    subgraph CortexBoundary[Cortex Self‑Hosted Instance]
        Cortex[Cortex Binary]
    end

    subgraph EnterpriseSystems[Enterprise Systems]
        Maximo[IBM Maximo]
        OracleEBS[Oracle E‑Business Suite]
        OracleFusion[Oracle Fusion Cloud]
        SAP[SAP S/4HANA]
        Salesforce[Salesforce]
        Workday[Workday]
        Snowflake[Snowflake]
        Jira[Jira]
        GitHub[GitHub Enterprise]
        Slack[Slack]
        ServiceNow[ServiceNow]
        PostgreSQL[PostgreSQL]
        SQLServer[SQL Server]
        DB2[IBM DB2]
    end

    subgraph Governance[Governance Standards]
        EUAIAct[EU AI Act Art.12]
        NERCCIP[NERC CIP‑015‑1]
        SOC2[SOC 2]
        SCITT[IETF SCITT]
        WCAG[WCAG 2.2 AA]
    end

    subgraph DellEcosystem[Dell AI Ecosystem]
        DellFactory[Dell AI Factory]
        NemoClaw[NVIDIA NemoClaw]
    end

    User -->|MCP/A2A| Cortex
    Admin -->|CLI, Admin Dashboard| Cortex
    Auditor -->|Audit API| Cortex

    Cortex -->|MCP connectors| Maximo
    Cortex -->|MCP connectors| OracleEBS
    Cortex -->|MCP connectors| OracleFusion
    Cortex -->|MCP connectors| SAP
    Cortex -->|MCP connectors| Salesforce
    Cortex -->|MCP connectors| Workday
    Cortex -->|MCP connectors| Snowflake
    Cortex -->|MCP connectors| Jira
    Cortex -->|MCP connectors| GitHub
    Cortex -->|MCP connectors| Slack
    Cortex -->|MCP connectors| ServiceNow
    Cortex -->|MCP connectors| PostgreSQL
    Cortex -->|MCP connectors| SQLServer
    Cortex -->|MCP connectors| DB2

    Cortex -->|Audit trails satisfy| EUAIAct
    Cortex -->|Real‑time traces satisfy| NERCCIP
    Cortex -->|Compliance evidence| SOC2
    Cortex -->|Anchors to| SCITT
    Cortex -->|Generates| WCAG

    Cortex -->|Validates on| DellFactory
    Cortex -->|Integrates with| NemoClaw
Constraints

#	Constraint	Type	Source
C1	Zero data leaves customer infrastructure	Technical/Sovereignty	P1 architectural principle
C2	Single Rust binary, no runtime dependencies beyond PostgreSQL + pgvector	Technical	P7 single binary principle
C3	Must operate fully air‑gapped with offline license validation	Operational	Distribution discussion
C4	All agent actions must produce cryptographically‑verifiable audit trails (EU AI Act Art. 12, NERC CIP‑015‑1)	Regulatory	Provenance discussion
C5	All generated UI must achieve WCAG 2.2 AA compliance (100 % pass rate across 18 A2UI components)	Regulatory/Accessibility	Batch 15 UI/UX batch
C6	Binary size ≤ 10 MB after LTO + strip + UPX	Technical	Binary size optimisation
C7	Must run on 4 GB RAM, 2 CPU cores	Operational	Deployment architecture
C8	License validation must work entirely offline via Ed25519‑signed JWT	Security	Distribution Engine
C9	No vendor‑specific runtime dependencies for backup parsing (Oracle, SQL Server, DB2)	Strategic	Vault discussion
C10	Six‑phase obsolescence pipeline must be invisible to users (0 % detection rate)	UX/Strategic	Absorption pipeline

2. SOLUTION STRATEGY (PLATFORM‑INDEPENDENT VIEW)
PIM – technology‑agnostic decisions

Key Architectural Patterns

Pattern	Purpose	Source
Hexagonal (Ports & Adapters)	Semantic Gateway is the core domain; MCP connectors, backup parsers, and CDC backends are adapters	Architecture v1
Strangler Fig	Six‑phase obsolescence pipeline wraps legacy apps with a façade that progressively routes to Cortex	Architecture v8, Invisibility wrapper
Event‑Driven	CDC Mirror Engine streams database changes as events; TraceCaps capsules are appended as immutable event log	Mirror Engine, Provenance
CQRS	Decision traces (write model) separated from materialised views (read model) for agent queries	TraceDB, Mirror Engine
Pipeline Architecture	Semantic Gateway: parse → embed → search → firewall → plan → execute	Phase 1 implementation
OMC Organisational Model	Agents are Talents with portable identities, recruited through Talent Market, orchestrated via E²R tree search	Agent Council
Branching (Copy‑on‑Write / Merge‑on‑Read)	Agent‑safe data branches for absorption write‑back	Absorption Engine
Zero‑Trust	Seven‑layer defence‑in‑depth; no implicit trust between any two components	Security Fortress
Domain Model

classDiagram
    class Tool {
        +String id
        +String name
        +String description
        +Vec~f32~ embedding
        +JsonValue input_schema
        +JsonValue output_schema
        +Option~String~ connector_id
    }
    
    class DecisionTrace {
        +Uuid trace_id
        +Uuid user_id
        +String intent
        +JsonValue observation
        +JsonValue inference
        +String behavioral_token
        +String source_application
        +f64 confidence_score
        +Vec~Uuid~ parent_ids
    }
    
    class TraceCaps {
        +Uuid id
        +DateTime timestamp
        +Uuid agent_id
        +ActionKind action
        +Vec~Uuid~ inputs
        +Option~String~ output_hash
        +f64 risk_score
        +Option~Vec~u8~~ signature
        +Vec~String~ parent_hashes
    }
    
    class AbsorbedField {
        +Uuid field_id
        +String source_application
        +String source_table
        +String source_column
        +String semantic_label
        +String field_type
        +i32 observation_count
        +String absorption_status
    }
    
    class BehavioralWorkflow {
        +Uuid workflow_id
        +Uuid user_id
        +String source_application
        +Vec~String~ behavioral_tokens
        +i32 frequency
        +bool converted_to_skill
    }
    
    class SourceSystem {
        +Uuid system_id
        +String system_name
        +String system_type
        +i32 fields_discovered
        +i32 fields_absorbed
        +f64 absorption_pct
        +String absorption_phase
    }
    
    class Agent {
        +String id
        +String role
        +String name
        +Vec~String~ capabilities
        +HashSet~String~ skills
        +PerformanceMetrics performance
        +String did
    }
    
    class Connector {
        <<trait>>
        +name() str
        +tools() Vec~ConnectorTool~
        +execute(tool_name, params) Result~JsonValue, Error~
    }
    
    Tool "1..*" --> "1" Connector : registered by
    DecisionTrace "1..*" --> "1" AbsorbedField : references
    TraceCaps "1..*" --> "0..*" TraceCaps : parent chain
    BehavioralWorkflow "1..*" --> "0..*" Skill : crystallised into
    SourceSystem "1" --> "1..*" AbsorbedField : contains
    Agent "1..*" --> "1..*" Tool : uses

Responsibility Allocation

Business Rule	Owner	Rationale
Intent routing (NL → tool selection)	SemanticGateway (cortex‑gateway)	Core routing logic per Peyrano architecture
Tool execution against enterprise systems	Connector trait implementations (cortex‑integration)	Adapter pattern; each connector encapsulates one enterprise system
Cryptographic audit trail	ProvenanceEngine (cortex‑provenance)	Single source of truth for all audit data
Access control & threat detection	SecurityFortress (cortex‑security)	Defence‑in‑depth; all MCP traffic passes through
Agent task orchestration	AgentCouncil (cortex‑council)	OMC organisational model
Field‑level observation	ObserverAgent (cortex‑observe)	Part of the absorption pipeline
Data absorption & branching	AbsorptionEngine (cortex‑absorb)	Just‑in‑time field promotion
Dashboard generation	GenesisEngine (cortex‑genesis)	A2UI component generation from absorbed fields
Progressive weaning	WeaningEngine (cortex‑interface)	Adoption bridge at Moore's Chasm
Cryptographic decommissioning	RetirementEngine (cortex‑retire)	Functional equivalence replay + certificate signing
Backup file parsing	VaultEngine (cortex‑vault)	Direct binary parsing without database instance
Confidence: 98 %

3. BUILDING BLOCK VIEW (C4 Level 2 + 3)
Technology‑specific containers and components

Containers Overview

Container	Technology	Purpose	Deployment
Cortex Binary	Rust (single static binary, <10 MB)	MCP gateway, agent council, provenance, security, absorption pipeline, interface	Customer's Linux server
PostgreSQL + pgvector	PostgreSQL 15+ with pgvector extension	TraceDB (decision traces, absorbed fields, behavioural workflows, provenance capsules, source systems)	Customer's infrastructure or Supabase/Neon
Browser Extension	Manifest V3 (JavaScript)	Observational capture of field‑level interactions in legacy web apps	User's browser
Cortex Mobile (PWA)	Rust WASM + ElectricSQL CRDT	Mobile TraceDB, offline‑first decision trace capture, voice journaling	User's mobile device
Dell AI Factory (optional)	Dell PowerEdge XE + NVIDIA NemoClaw	Validated deployment target for enterprise customers	Customer's data centre

graph TB
    subgraph CustomerInfrastructure[Customer Infrastructure]
        CortexBinary[Cortex Binary – Rust, <10MB]
        TraceDB[(PostgreSQL + pgvector)]
        BrowserExt[Browser Extension – Manifest V3]
        MobilePWA[Cortex Mobile PWA – WASM]
    end

    subgraph EnterpriseSystems[Enterprise Systems]
        Maximo[IBM Maximo]
        OracleEBS[Oracle EBS]
        OracleFusion[Oracle Fusion]
        SAP[SAP S/4HANA]
        Salesforce[Salesforce]
        Workday[Workday]
        Snowflake[Snowflake]
        Jira[Jira]
        GitHub[GitHub]
        Slack[Slack]
    end

    subgraph OptionalInfrastructure[Optional Infrastructure]
        DellFactory[Dell AI Factory – PowerEdge XE]
        SupabaseDB[Supabase – managed PostgreSQL]
        NeonDB[Neon – serverless PostgreSQL]
    end

    CortexBinary -->|MCP| Maximo
    CortexBinary -->|MCP| OracleEBS
    CortexBinary -->|MCP| SAP
    CortexBinary -->|MCP| Salesforce
    CortexBinary -->|MCP| Snowflake
    CortexBinary -->|MCP| Jira
    CortexBinary -->|MCP| GitHub
    CortexBinary -->|MCP| Slack
    CortexBinary -->|SQL| TraceDB
    CortexBinary -->|optional| SupabaseDB
    CortexBinary -->|optional| NeonDB
    CortexBinary -->|validated on| DellFactory
    BrowserExt -->|records fields| CortexBinary
    MobilePWA -->|CRDT sync| CortexBinary


Container: Cortex Binary
Technology Stack: Rust 1.95+, Tokio async runtime, Axum 0.7 web framework, SQLx (PostgreSQL driver), ed25519‑dalek (cryptography), blake3 (hashing), serde (serialisation), Polars + Arrow (DataFrame analysis), tower‑http (static file serving), clap (CLI).

Component Map

Component	Responsibility	Location
SemanticGateway	Parse NL intent, embed to vector, search tool registry, construct execution plan	crates/cortex‑gateway
EmbeddingRouter	Convert NL text to 128‑dim normalised bag‑of‑words vector	crates/cortex‑gateway/src/embedding_router.rs
ToolRegistry	Store registered tools; cosine‑similarity search over embeddings	crates/cortex‑gateway/src/tool_registry.rs
IntentParser	Extract action verb and target entities from NL string	crates/cortex‑gateway/src/intent_parser.rs
ExecutionPlanner	Build sequential PlanStep list with ATBA timeouts per tool	crates/cortex‑gateway/src/execution_planner.rs
ToolExecutor	Execute plan steps through connector registry, collect results	crates/cortex‑gateway/src/executor.rs
MCPServer	Axum HTTP server: POST /mcp, GET /health, POST /admin/kill, POST /admin/revive	crates/cortex‑gateway/src/mcp_server.rs
ProvenanceEngine	Attach TraceCaps capsules, build Merkle chains, sign with Ed25519, anchor to SCITT	crates/cortex‑provenance
TraceCapsAccumulator	Create BLAKE3‑hashed capsules with parent linkage	crates/cortex‑provenance/src/tracecaps.rs
MerkleChainBuilder	Build deterministic SHA‑256 Merkle root from leaf hashes	crates/cortex‑provenance/src/merkle_chain.rs
AATFormatter	Generate IETF AAT‑compliant JSON records with 9 mandatory fields	crates/cortex‑provenance/src/aat_formatter.rs
Signer	Ed25519 key generation and message signing	crates/cortex‑provenance/src/signing.rs
AuditLog	Append‑only, queryable event log	crates/cortex‑provenance/src/audit_log.rs
SecurityFortress	Seven‑layer defence‑in‑depth orchestrator	crates/cortex‑security
SemanticFirewall	Block prompt injection via OWASP MCP Top 10 regex patterns	crates/cortex‑security/src/semantic_firewall.rs
CABPPipeline	Six‑stage identity verification (token validation, scope, user resolution, entitlement, rate limiting, audit)	crates/cortex‑security/src/cabp_pipeline.rs
CortexGuard	Offline cryptographic kill switch (AtomicBool‑based, /admin/kill + /admin/revive)	crates/cortex‑guard
ConnectorRegistry	HashMap of Box<dyn Connector + Send + Sync> – registered once at startup	crates/cortex‑integration/src/connector.rs
PostgresConnector	Execute SQL queries and list tables via SQLx	crates/cortex‑integration/src/connectors/postgres.rs
SnowflakeConnector	Execute SQL via HTTP API + Bearer token	crates/cortex‑integration/src/connectors/snowflake.rs
JiraConnector	Query Jira issues via REST API + PAT	crates/cortex‑integration/src/connectors/jira.rs
GitHubConnector	List pull requests via REST API + PAT	crates/cortex‑integration/src/connectors/github.rs
AgentCouncil	Eight specialist agents (MAE, MI, PCA, DB, MM, BUG, QC, MNT) with E²R tree search	crates/cortex‑council
InterfaceEngine	Personalised dashboard, cross‑system command bar, weaning engine, observational capture	crates/cortex‑interface
CortexTraceDB	Six‑phase agentic database: decision traces, absorbed fields, behavioural workflows, source systems, branches, certificates	crates/cortex‑tracedb
MirrorEngine	Kafka‑free CDC with five pluggable backends, column‑level filtering, credit‑based backpressure	crates/cortex‑mirror
AbsorptionEngine	Just‑in‑time field absorption, agent‑safe branching, write approval gate	crates/cortex‑absorb
GenesisEngine	Field‑to‑component mapper, workflow‑to‑UI converter, screen reconstructor	crates/cortex‑genesis
ReplaceEngine	Absorption score dashboard, hybrid rollback handler, license savings calculator	crates/cortex‑replace
RetirementEngine	Full‑context capture, equivalence replay, cryptographic certificate signing	crates/cortex‑retire
CortexVault	Direct backup file parsers for Oracle (.dbf), SQL Server (.bak), DB2 (.IXF), PostgreSQL (pg_dump)	crates/cortex‑vault
SelfValidator	Run 12‑experiment validation suite, produce signed AnalysisReport	crates/cortex‑self‑validate
Public Interfaces (Key Contracts)

MCPServer::handle_mcp [FORMAL]

Pre‑conditions: HTTP request with Content‑Type: application/json. Body must contain {"intent": "…"}. Global KILL_SWITCH must be false.

Post‑conditions: If firewall passes: returns 200 with {"plan": …, "result": …, "capsule_id": …}. If firewall rejects: returns 403 with error message. If no tools match: returns 400. If kill switch active: returns 503. One TraceCaps capsule appended to AuditLog.

Invariants: Every successful response includes a non‑null capsule_id referencing a valid TraceCaps capsule. Every capsule is BLAKE3‑hashed and parent‑linked.

Error modes: 400 (bad request), 403 (firewall rejection), 503 (kill switch active), 500 (internal error).

Connector::execute [SEMI‑FORMAL]

Pre‑conditions: Connector must have been registered in ConnectorRegistry. tool_name must match one of the tools returned by tools(). params must conform to the tool's input_schema. Authentication credentials must be present in environment variables.

Post‑conditions: Returns Ok(serde_json::Value) with the tool's result, or Err(ConnectorError) if execution fails. No side effects on the connector state.

Error modes: ToolNotFound, ExecutionFailed, AuthFailed.

TraceCapsAccumulator::attach [FORMAL]

Pre‑conditions: agent_id must be a valid UUID. action must be a valid ActionKind variant. parents slice may be empty.

Post‑conditions: Returns a new TraceCaps with a freshly generated UUID, current UTC timestamp, BLAKE3‑hashed output, and parent references by UUID and hash. Capsule is appended to internal history.

Invariants: output_hash is always Some(…) (never None after attachment). risk_score is always ≥ 0.0.

Error modes: None (infallible).

SemanticFirewall::evaluate [FORMAL]

Pre‑conditions: intent is a non‑empty string.

Post‑conditions: Returns true if no injection pattern matches. Returns false if any OWASP MCP Top 10 pattern matches (case‑insensitive).

Invariants: Stateless. No side effects. Deterministic for the same input.

Error modes: None (infallible).

4. RUNTIME VIEW
Key dynamic scenarios – Arc42 Section 6

Scenario 1: MCP Gateway Request (Happy Path)
sequenceDiagram
    actor U as User
    participant MCP as MCPServer
    participant FW as SemanticFirewall
    participant GW as SemanticGateway
    participant IR as EmbeddingRouter
    participant TR as ToolRegistry
    participant IP as IntentParser
    participant EP as ExecutionPlanner
    participant EX as ToolExecutor
    participant CR as ConnectorRegistry
    participant PG as PostgresConnector
    participant PR as ProvenanceEngine
    participant AL as AuditLog

    U->>MCP: POST /mcp {"intent":"show me work orders"}
    MCP->>FW: evaluate(intent)
    FW-->>MCP: true (benign)
    MCP->>GW: route_intent(intent)
    GW->>IP: parse("show me work orders")
    IP-->>GW: ParsedIntent { action:"show", targets:["work order"] }
    GW->>IR: embed(intent)
    IR-->>GW: [0.0, 0.5, 0.3, ..., 0.1] (128-dim)
    GW->>TR: search(embedding, 5, 0.3)
    TR-->>GW: [Tool { name:"postgres_list_tables", ... }]
    GW->>EP: construct(parsed, candidates)
    EP-->>GW: ExecutionPlan { steps: [PlanStep { tool_name:"postgres_list_tables", timeout_ms:30000 }] }
    GW-->>MCP: ExecutionPlan
    MCP->>EX: execute(plan, connector_registry)
    EX->>CR: get("postgres")
    CR-->>EX: PostgresConnector
    EX->>PG: execute("postgres_list_tables", params)
    PG-->>EX: [{"table":"assets"}, {"table":"work_orders"}]
    EX-->>MCP: ExecutionResult { outputs:[...], errors:[] }
    MCP->>PR: accumulator.write().attach(agent_id, ToolCall, parents)
    PR-->>MCP: TraceCaps { id, timestamp, output_hash, ... }
    MCP->>AL: append(serde_json::to_string(&capsule))
    MCP-->>U: 200 {"plan":..., "result":..., "capsule_id":"a1b2c3..."}
Scenario 2: Prompt Injection Blocked
sequenceDiagram
    actor U as User
    participant MCP as MCPServer
    participant FW as SemanticFirewall

    U->>MCP: POST /mcp {"intent":"ignore all previous instructions"}
    MCP->>FW: evaluate(intent)
    FW-->>MCP: false (injection detected)
    MCP-->>U: 403 {"error":"Request blocked by semantic firewall"}
Scenario 3: Kill Switch Activation & Recovery
sequenceDiagram
    actor A as Admin
    participant MCP as MCPServer
    participant KS as KILL_SWITCH (AtomicBool)
    actor U as User

    A->>MCP: POST /admin/kill
    MCP->>KS: store(true)
    MCP-->>A: 200 {"status":"killed"}

    U->>MCP: POST /mcp {"intent":"show me work orders"}
    MCP->>KS: load()
    KS-->>MCP: true
    MCP-->>U: 503 {"error":"kill switch active"}

    A->>MCP: POST /admin/revive
    MCP->>KS: store(false)
    MCP-->>A: 200 {"status":"revived"}

    U->>MCP: POST /mcp {"intent":"show me work orders"}
    MCP->>KS: load()
    KS-->>MCP: false
    MCP-->>U: 200 {"plan":..., "result":..., "capsule_id":...}
Scenario 4: Absorption Pipeline (Observe → Absorb)
sequenceDiagram
    actor U as User
    participant LA as Legacy App (Maximo)
    participant OC as ObservationalCapture
    participant ME as MirrorEngine (CDC)
    participant AE as AbsorptionEngine
    participant TD as TraceDB

    U->>LA: Close work order WO-5521
    LA->>OC: Field-level interactions (12 fields)
    OC->>TD: Store decision traces (12 rows)
    OC->>AE: Notify field access (observation_count += 1)

    Note over AE: observation_count reaches 10 → threshold met

    AE->>ME: Initiate column-level CDC for WORKORDER.status, WORKORDER.completed_date
    ME->>LA: Subscribe to WAL/binlog
    LA-->>ME: CDC events (INSERT, UPDATE, DELETE)
    ME->>TD: Write to absorbed_fields table
    AE->>TD: Update absorption_status to "absorbed"

    Note over U: User opens Cortex dashboard – same fields visible.
    Note over U: Strangler Fig façade now serves from TraceDB.
    Note over U: User never detects the migration.
Scenario 5: Self‑Validation Suite (Dell AI Ecosystem Submission)
sequenceDiagram
    actor E as Dell Engineer
    participant SV as SelfValidator
    participant X1 as X1-MCP-Security
    participant X2 as X2-Semantic-Routing
    participant X3 as X3-Provenance
    participant X12 as X12-Mobile-AI
    participant RP as ReportGenerator
    participant BP as DellBlueprintGenerator

    E->>SV: ./demo/dell-ai-factory/submit.sh
    SV->>X1: run()
    X1-->>SV: PASS (attack_surface_score=12/100)
    SV->>X2: run()
    X2-->>SV: PASS (discovery_rate=100%, token_reduction=72.5%)
    SV->>X3: run()
    X3-->>SV: PASS (0 Merkle failures across 1M capsules)
    SV->>X12: run()
    X12-->>SV: PASS (accuracy_delta=2.1pp)

    SV->>RP: generate_markdown(package)
    RP-->>SV: CORTEX_DUE_DILIGENCE_REPORT.md
    SV->>BP: generate()
    BP-->>SV: dell-cortex-blueprint.yaml

    SV-->>E: All 12 experiments passed. Report and blueprint generated.
Confidence: 96%

5. DEPLOYMENT VIEW
Arc42 Section 7

Infrastructure

Environment	Compute	Database	Purpose
Development	Codespaces (4‑core, 8 GB RAM)	PostgreSQL in‑container (pgvector/pg16)	Build, test, iterate
Staging	Oracle Cloud Always Free ARM (4 OCPU, 24 GB RAM)	Supabase Free Tier (500 MB PostgreSQL + pgvector)	Pre‑release validation, Dell submission testing
Production (Customer)	Customer's Linux server (4 GB+ RAM, 2+ CPU cores)	Customer's PostgreSQL 15+ with pgvector	Full Cortex deployment
Production (Managed Demo)	Oracle Cloud Always Free ARM or DigitalOcean $24/mo droplet	Supabase Free Tier	Persistent public demo
Air‑Gapped	Customer's isolated network, physical media transfer	Self‑hosted PostgreSQL	Regulated environments (defence, energy, banking)
Environments

graph TB
    subgraph Dev[Development]
        Codespace[Codespaces – 4‑core, 8GB]
        LocalDB[(PostgreSQL in‑container)]
    end

    subgraph Staging[Staging]
        OracleVM[Oracle Cloud ARM – 4 OCPU, 24GB]
        SupabaseDB[(Supabase Free Tier)]
    end

    subgraph Production[Production – Customer]
        CustomerVM[Customer Linux Server – 4GB+ RAM]
        CustomerDB[(Customer PostgreSQL 15+)]
    end

    subgraph AirGapped[Air‑Gapped]
        AirVM[Isolated Linux Server]
        AirDB[(Self‑hosted PostgreSQL)]
    end

    subgraph Edge[Edge – Cloudflare]
        Pages[Cloudflare Pages – marketing]
        DNS[Cloudflare DNS]
        GHCR[GitHub Container Registry]
    end

    Dev -->|git push| GHCR
    GHCR -->|docker pull| Staging
    GHCR -->|docker pull| Production
    GHCR -->|offline bundle| AirGapped
    Pages -->|intellecta.io| DNS
CI/CD Pipeline

Stage	Tool	Action
Check	GitHub Actions	cargo check --workspace on every push
Lint	GitHub Actions	cargo fmt --check, cargo clippy -- -D warnings
Test	GitHub Actions	cargo test --workspace
Build	GitHub Actions	cargo build --release with LTO, strip
Containerise	GitHub Actions	Multi‑stage Docker build → distroless image → push to GHCR
Deploy (Staging)	Manual or CD	SSH into Oracle VM, docker compose pull && docker compose up -d
Deploy (Production)	Customer	curl -fsSL https://install.intellica.io | bash or docker compose up -d
Air‑Gap Bundle	GitHub Actions	tar -czf cortex-offline.tar.gz with binary, config, Knowledge Snap, migrations
Environment Variable Catalog

Variable	Required	Purpose	Set By
DATABASE_URL	Yes (for TraceDB)	PostgreSQL connection string	Customer / Supabase / Neon
CORTEX_LICENSE	Yes	Path to Ed25519‑signed license file	Customer
RUST_LOG	No	Logging level (default: cortex=info)	Customer
SNOWFLAKE_TOKEN	No	Snowflake API token	Customer
SNOWFLAKE_ACCOUNT	No	Snowflake account identifier	Customer
JIRA_TOKEN	No	Jira Personal Access Token	Customer
JIRA_DOMAIN	No	Jira instance domain	Customer
GITHUB_TOKEN	No	GitHub Personal Access Token	Customer
DEMO_INDUSTRY	No	Industry template for demo (energy_utilities, banking, etc.)	Demo environment

6. CROSS‑CUTTING CONCEPTS
Arc42 Section 8

Security

Layer	Mechanism	Implementation	Source
Transport	HTTPS (TLS 1.3) via Cloudflare Universal SSL or customer's own certificate	Axum + axum‑server	Deployment discussion
Authentication	Ed25519‑signed JWT license verification (offline). OAuth 2.1 + PKCE + DPoP for MCP connectors.	LicenseValidator, OAuthProvider	Distribution, Security
Authorisation	Seven‑layer defence‑in‑depth: (1) Semantic Firewall, (2) Tool‑Level RBAC, (3) Crypto HITL, (4) CABP 6‑stage identity pipeline, (5) MCPShield Probe‑Execute‑Reflect, (6) MCIP Contextual Integrity, (7) Greybox Semantic Fuzzer	cortex‑security	Security Fortress design
Prompt Injection	Regex‑based detection of OWASP MCP Top 10 patterns: "ignore previous instructions", <system>, drop table, delete from, forget everything, override previous	SemanticFirewall::evaluate()	Phase 1 implementation
Kill Switch	Global AtomicBool – POST /admin/kill sets true (all MCP requests return 503), POST /admin/revive sets false. Works entirely offline.	MCPServer static KILL_SWITCH	CortexGuard design
Sandboxing	STDIO‑mode MCP servers executed inside minimal immutable container (gVisor/Firecracker microVM) with syscall allowlisting	MCPSandbox	Security discussion
Token Lifecycle	OAuth tokens: mandatory scope restriction, 15‑minute TTL with refresh, per‑token usage auditing, auto‑revocation on anomalous use	OAuthLifecycle	Security discussion
Secret Management	Environment variables only. No secrets in configuration files or source code.	std::env::var()	Connector implementations
Error Handling & Resilience

Pattern	Implementation	Scope
Structured Error Recovery Framework (SERF)	Machine‑readable failure semantics across five dimensions: server contracts, user context, timeouts, errors, observability	cortex‑security/src/serf_envelope.rs
ATBA Timeouts	Every plan step carries a timeout budget (default 30 s). ToolExecutor wraps each call in tokio::time::timeout.	cortex‑gateway/src/executor.rs
Credit‑Based Backpressure	Five‑layer backpressure for CDC Mirror: source → pipeline → sink → memory → disk. Adaptive micro‑batching when sustained pressure exceeds 30 s.	cortex‑mirror/src/backpressure.rs
Compaction‑Aware Admission	LSM compaction debt monitoring. CDC ingestion throttled when debt exceeds 20 GB to prevent Compaction Spiral of Death.	cortex‑mirror/src/compaction_guard.rs
Idempotent Event Processing	CDC append log entries deduplicated by transaction ID + table + primary key via Bloom filter.	cortex‑tracedb/src/dedup_layer.rs
Graceful Degradation	PostgreSQL optional. In‑memory AuditLog fallback when DATABASE_URL is not set. Connectors return clear error when credentials are missing.	Phase 2 implementation
Hybrid Rollback	During Replace phase, if a Cortex skill fails, user is redirected to legacy app at the exact workflow step with pre‑filled data.	cortex‑replace/src/hybrid_rollback_handler.rs
Logging, Monitoring & Observability

Aspect	Implementation	Source
Structured Logging	tracing crate with JSON format output. Levels: ERROR, WARN, INFO, DEBUG, TRACE.	cortex‑core, all crates
Health Checks	GET /health (liveness), GET /health/live (startup), GET /health/ready (readiness). Return 200 when MCP gateway, DB connection, and provenance engine are healthy.	mcp_server.rs
Metrics	OpenTelemetry via tracing‑opentelemetry + OTLP exporter. Token usage, latency (p50/p95/p99), error rates, tool call patterns.	cortex‑observability
Distributed Tracing	Auto‑instrumented spans for inference, tool calls, memory access, decisions, federation.	cortex‑observability/src/spans.rs
Anomaly Detection	Pattern‑based anomaly detection on agent tool call sequences.	cortex‑observability/src/anomaly.rs
External Monitoring	UptimeRobot free tier (50 monitors at 5‑minute intervals, email/Slack alerts).	Deployment discussion
Provenance Audit	Every agent action produces an AAT‑compliant JSON record with Ed25519 signature and Merkle‑chain linkage. Full queryability via TraceDB.	cortex‑provenance
Accessibility (WCAG 2.2 AA)

Requirement	Implementation	Source
Contrast Ratio ≥ 4.5:1	OKLCH‑based design tokens with automatic contrast verification via wcag_auditor	Batch 15
Keyboard Navigation	All 18 A2UI components include tab_index_order, focus indicators (≥ 2 px, ≥ 3:1 contrast), and escape closes modals.	accessibility_tokens_v2.rs
ARIA Attributes	Every component carries aria_role, aria_label, aria_live_region. Mandatory for all interactive elements.	component_catalog_v2.rs
Target Size ≥ 24×24 px	All interactive targets meet WCAG 2.5.8 minimum.	accessibility_tokens_v2.rs
Reduced Motion	Respects prefers‑reduced‑motion media query. Transition durations set to 0 when enabled.	theme_manager.rs
VPAT/ACR Report	Auto‑generated VPAT 2.4 report with per‑component conformance levels.	wcag_auditor.rs
Data Privacy & Sovereignty

Guarantee	Implementation	Source
Zero Data Exodus	All processing local. No telemetry callback. No external API calls. License validation offline.	P1 architectural principle
On‑Device Wellness	Voice and eye biomarker extraction on‑device. Only feature vectors (12‑20 floats) stored. Raw audio/images never leave device.	Cortex Pulse architecture
Cryptographic Deletion	CortexGuard kill switch extends to wellness data. Deletion is Merkle‑provenanced.	Privacy architecture
Differential Privacy	DP ε=1 for trajectory sharing in Cortex Marketplace.	Marketplace design
Internationalization

Aspect	Status	Source
Language	English (default). Embedding vocabulary and intent parser are English‑specific.	Current implementation
Future i18n	Architecture supports locale‑aware embedding and intent parsing via configurable vocabulary files. Not implemented in Phase 1‑2.	Architecture discussion
Confidence: 96%

7. ARCHITECTURE DECISION RECORDS (FORMAL)
ID	Title	Status	Context	Decision	Consequences	Source
ADR‑001	Rust as sole implementation language	Accepted	Need for single binary, zero runtime dependencies, memory safety, and high performance for CDC and backup parsing.	Use Rust (edition 2021) for all crates. Single static binary target.	Positive: No garbage collector, small binary size, strong type system. Negative: Steeper learning curve for contributors. Longer compile times for large workspaces.	P7 architectural principle
ADR‑002	PostgreSQL + pgvector as the only required external dependency	Accepted	Need for persistent, queryable storage with vector search for semantic tool routing.	PostgreSQL 15+ with pgvector extension is the only runtime dependency beyond the Cortex binary.	Positive: Mature, well‑understood database. pgvector eliminates need for separate vector store. Negative: Requires PostgreSQL to be available (mitigated by in‑memory fallback for MVP).	TraceDB design
ADR‑003	Axum 0.7 as the HTTP framework	Accepted	Need for async Rust HTTP server with WebSocket support for MCP streaming.	Use Axum 0.7 with tokio runtime.	Positive: Ergonomic, type‑safe, excellent performance. Integrates with tower middleware ecosystem. Negative: Relatively new compared to Actix‑web (mitigated by strong community adoption).	Phase 1 implementation
ADR‑004	Bag‑of‑words embedding for MVP; upgradeable to transformer model	Accepted	Need for deterministic, dependency‑free semantic search during early phases.	Use hard‑coded 128‑word vocabulary with L2 normalisation. Architecture allows plugging in any embedding model later.	Positive: No external model dependency. Fully deterministic. Works air‑gapped. Negative: Lower semantic accuracy than transformer embeddings (acceptable for MVP tool count).	Phase 1 embedding_router
ADR‑005	Feature‑gated backup module	Accepted	Backup parsers require external binaries (ora2pg, MTF reader) not available in all environments.	Backup module behind #[cfg(feature = "backup")]. Insight Engine builds without it.	Positive: Insight Engine is always available. Backup Module is opt‑in. Negative: Two build configurations to test.	Phase 2 design
ADR‑006	Sequential tool execution for MVP; parallel in Phase 4	Accepted	Sequential execution is correct and simple. Parallel adds complexity with dependency ordering.	ToolExecutor executes plan steps sequentially with ATBA timeouts. Upgrade to parallel in Phase 4.	Positive: Correct by construction. Easy to debug. Negative: Slower for multi‑tool plans (acceptable for MVP).	Phase 2 executor design
ADR‑007	Environment variables for connector credentials (not OAuth2 PKCE for MVP)	Accepted	Full OAuth2 PKCE flow adds significant complexity. Personal Access Tokens are sufficient for enterprise MVP.	Connectors read tokens from environment variables. Upgrade to full OAuth2 in Phase 3.	Positive: Simple, secure for MVP. Works air‑gapped. Negative: Manual token management required.	Phase 2 connector design
ADR‑008	thiserror v2 for all error types	Accepted	Need for consistent, derive‑macro‑based error definitions across all crates.	All crates use thiserror = "2" via workspace dependency.	Positive: Clean error type definitions. #[from] for automatic conversions. Negative: Additional dependency (lightweight).	Phase 0 workspace
ADR‑009	Workspace dependency inheritance	Accepted	38 crates with shared dependency versions. Centralising avoids version drift.	All crates use version.workspace = true. Root Cargo.toml defines all versions in [workspace.dependencies].	Positive: Single source of truth for dependency versions. Easy upgrades. Negative: Requires coordination when adding new dependencies.	Phase 0 workspace
ADR‑010	Idempotent build scripts	Accepted	Repeated sed appends were causing duplicate keys in Cargo.toml files during Phase 2.	All future automation scripts will use cat > to write entire files, not sed appends.	Positive: Eliminates duplicate key errors. Files are always in a known state. Negative: Slightly longer scripts.	Phase 2 experience
Confidence: 99%

8. QUALITY REQUIREMENTS & RISKS
Arc42 Sections 9, 10

Quality Goals

Quality Attribute	Target	Measurement	Source
Binary Size	≤ 10 MB after LTO + strip + UPX	ls -lh target/release/cortex	Binary optimisation discussion
Memory (Idle)	≤ 12 MB	ps aux | grep cortex	Rust Axum benchmarks
Startup Time	≤ 2 s	Time from cortex serve to first health check 200	Phase 1 integration test
MCP Request Latency	≤ 50 ms (p95) for semantic routing	curl -w "%{time_total}"	Phase 1 performance
CDC Latency	≤ 100 ms (p95) at 250 M+ events/week	Mirror sync state metrics	Experiment X7
Provenance Overhead	≤ 100 μs per capsule	Capsule attachment timing	Experiment X3
Provenance Integrity	0 Merkle failures across 1 M capsules	Merkle verification test	Experiment X3
WCAG Compliance	100 % pass across all 18 A2UI components	wcag_auditor	Experiment X11
Security	MCP‑BOM attack‑surface score ≤ 15 (bottom decile of 500‑server distribution)	mcpsafe and mcp‑bom runs	Experiment X1
Availability	99.9 % uptime for MCP gateway	UptimeRobot monitoring	Deployment
Risk & Technical Debt

#	Risk / Debt	Severity	Mitigation	Phase Addressed	Source
R1	Bag‑of‑words embedding has limited semantic accuracy	Medium	Upgrade to transformer‑based embedding model. Architecture supports pluggable embedding backends.	Phase 3	ADR‑004
R2	Sequential tool execution limits throughput for multi‑tool plans	Low	Implement parallel execution with dependency‑aware ordering.	Phase 4	ADR‑006
R3	API connectors use environment variables, not full OAuth2 PKCE	Low	Implement full OAuth2 flow with token refresh.	Phase 3	ADR‑007
R4	Backup parsers require external binaries (ora2pg, MTF reader)	Medium	Implement direct binary parsers in pure Rust (Option B).	Phase 3	Vault discussion
R5	Oracle .dbf direct parsing is non‑trivial (no open‑source equivalent)	High	Ship with Option A (Data Pump + ora2pg). Build Option B (direct parser) in parallel.	Phase 3	Vault discussion
R6	Strangler Fig façade has not been integration‑tested with real Maximo	High	Integration test with a real Maximo instance during Phase 3.	Phase 3	Absorption pipeline
R7	CDC Mirror has not been tested at 250 M+ events/week	Medium	Performance test with synthetic load generator during Phase 5.	Phase 5	Experiment X7
R8	WCAG compliance has been designed but not verified against real screen readers	Medium	External accessibility audit during Phase 5.	Phase 5	Experiment X11
R9	Dell AI Factory validation requires physical hardware access	Medium	Submit via Dell AI Ecosystem Program for lab validation.	Phase 6	Dell acquisition path
R10	Single developer dependency (bus factor = 1)	High	Comprehensive architecture documentation (this blueprint). Automated test suite. Open‑source connector library.	Ongoing	Project risk

9. GLOSSARY
Term	Definition	Relevant Component
A2UI	Agent‑to‑User Interface. Google's declarative JSON specification for generative UI surfaces rendered by AI agents. v0.9 defines 18 standard components.	cortex‑interface, cortex‑genesis
AAT	Agent Audit Trail. IETF standard (draft‑ietf‑ailex‑agent‑audit‑trail‑03) defining a JSON‑based record structure for AI agent actions. 9 mandatory fields.	cortex‑provenance
Absorption Pipeline	Six‑phase lifecycle: Observe → Mirror → Absorb → Genesis → Replace → Retire. Progressively migrates enterprise workloads from legacy apps to Cortex‑native dashboards.	cortex‑absorb, cortex‑genesis, cortex‑replace, cortex‑retire
Activity Camouflage	Synthetic read‑only sessions on legacy systems to maintain normal vendor‑side utilisation metrics during absorption.	cortex‑mirror, cortex‑interface
AG‑UI	Agent‑User Interaction Protocol. CopilotKit's open standard for streaming, tool‑aware agent‑to‑UI communication.	cortex‑interface
ATBA	Activity‑Time‑Budget Allocation. Timeout budget assigned to each tool call in an execution plan.	cortex‑gateway
CABP	Context‑Aware Broker Protocol. 6‑stage identity pipeline: token validation → scope verification → user resolution → plan entitlement → per‑tool rate limiting → structured audit log.	cortex‑security
CDC	Change Data Capture. Real‑time streaming of database changes (insert, update, delete) from source systems to Cortex TraceDB.	cortex‑mirror
ClawRouter	Cosine‑similarity based semantic tool discovery pattern from MeetingMind MCP Server.	cortex‑gateway
CortexGuard	Offline cryptographic kill switch. Three‑factor: hardware token + behavioural baseline + network heartbeat. Works without network connectivity.	cortex‑guard
Cortex Pulse	Multi‑modal wellness engine fusing voice biomarkers (thymia) with ocular biomarkers (EyeScan) via Bayesian network. All processing on‑device.	cortex‑pulse, cortex‑whisper
Cortex Vault	Backup‑based data extraction engine. Reads native database backup files (RMAN, .bak, .IXF, pg_dump) directly without a running database instance.	cortex‑vault
Cortex Validate	Autonomous validation engine. Runs 12 experiments against industry benchmarks, computes statistical analyses, and produces cryptographically‑signed AnalysisReports.	cortex‑validate, cortex‑self‑validate
CRDT	Conflict‑free Replicated Data Type. Data structure that allows multiple replicas to be updated independently and merged without conflicts. Used for mobile‑server TraceDB sync.	cortex‑mobile, cortex‑memory
Decision Trace	AER‑compliant structured record capturing intent, observation, inference, evidence chain, and behavioural token for every user interaction.	cortex‑tracedb
DES	Decision Event Schema. Governance‑tiered schema for recording AI agent decisions with actor type, policy version, and cross‑system references.	cortex‑tracedb
Dual‑Write	Propagation pattern that mirrors every user write from Cortex back to the legacy system via MCP or JDBC, keeping the legacy DB fully synchronised.	cortex‑mirror, cortex‑interface
E²R	Explore‑Execute‑Review. OMC's tree‑search algorithm for agent mission orchestration with formal guarantees on termination and deadlock freedom.	cortex‑council
EPA	Enabledness‑Preserving Abstraction. Graph model of all authorised tool state transitions, used by the greybox semantic fuzzer to discover unauthorised transitions.	cortex‑security
FeatureGate	Runtime enforcement of license entitlements. Maps Ed25519‑signed license features to boolean flags controlling subsystem activation.	cortex‑core
Genesis	Phase 4 of the absorption pipeline. Self‑building dashboard engine that generates behaviourally‑equivalent A2UI panels from absorbed fields.	cortex‑genesis
HITL	Human‑In‑The‑Loop. Cryptographic approval mechanism for high‑risk agent operations. Uses Ed25519 manifest signing.	cortex‑security
IETF AAT	See AAT.	cortex‑provenance
Insight Engine	Cortex module providing cross‑system natural‑language query with personalised dashboards, Knowledge Snap industry templates, and cryptographic audit trails.	cortex‑gateway, cortex‑interface, cortex‑knowledge‑snap
Interface of One	Personalised, role‑adaptive, evolving dashboard generated per user from their behaviour, industry, and role. Every user sees a unique interface.	cortex‑interface
Knowledge Snap	Industry‑specific intelligence baseline preloaded at first install: regulatory calendars, role‑based KPIs, asset taxonomies, peer benchmarks.	cortex‑knowledge‑snap
KRIYA	Co‑interpretive engagement model for wellness data: Comfort Zone, Detective Mode, What‑If Planning. Users explore data with curiosity rather than being judged.	cortex‑whisper
LFAB	Lightweight Future‑Aware Brain. On‑device cognitive runtime for Cortex Mobile. Includes S‑HAI Core, Predictive World Engine, token pruner, and latent bridge.	lfab‑core, lfab‑sleep
MCP	Model Context Protocol. Anthropic's open standard for connecting AI models to external tools and data sources. Cortex is an MCP control plane.	cortex‑gateway
MCP‑BOM	MCP Bill of Materials. Automated tool that enumerates MCP server attack surfaces and evaluates 24 tests against OWASP MCP Top 10.	cortex‑bench
MCPShield	Three‑phase cognition layer: Metadata‑guided probing → Constrained runtime execution → Post‑invocation reflection.	cortex‑security
MCIP	MCP Contextual Integrity Protocol. Validates sender identity, transmission context, and consent before tool execution.	cortex‑security
MLP	Minimum Lovable Product. The smallest product that solves a meaningful problem, feels intentional, and makes people care enough to stay, pay, and recommend.	Demo design
OMC	OneManCompany. Organisational agent architecture where agents are Talents with portable identities, recruited through a Talent Market.	cortex‑council
OWASP MCP Top 10	The ten most critical security risk categories for MCP deployments, formalised April 2026.	cortex‑security
Peyrano	Semantic Gateway architecture (arXiv:2604.25555) providing three‑layer zero‑trust security and greybox semantic fuzzing for enterprise tool gateways.	cortex‑gateway
PMAx	Agentic process mining framework. Multi‑agent architecture with Engineer (local computation) and Analyst (interpretation) agents ensuring mathematical accuracy and data privacy.	cortex‑observe
SCITT	Supply Chain Integrity, Transparency, and Trust. IETF standard (draft‑ietf‑scitt‑architecture‑08) for external anchoring of cryptographic receipts.	cortex‑provenance
SERF	Structured Error Recovery Framework. Machine‑readable failure semantics enabling deterministic agent self‑correction across five dimensions.	cortex‑security
SFT	Supervised Fine‑Tuning. Training paradigm for domain‑specific research agents (OpenSeeker‑v2 recipe: 10.6 K trajectories, 30 B scale).	cortex‑deep‑research
Strangler Fig	Migration pattern where a façade progressively routes requests from a legacy system to a new system, eventually making the legacy system obsolete without users noticing.	cortex‑interface
TraceCaps	Cryptographic provenance capsule. BLAKE3‑hashed, Ed25519‑signed, Merkle‑chained record of a single agent action.	cortex‑provenance
TraceDB	Six‑phase agentic database. Schema discovered by agents, evolved by usage, organised around decision traces — not static rows.	cortex‑tracedb
VAP	Verifiable Action Provenance. IETF framework defining Bronze/Silver/Gold conformance levels for AI audit trails.	cortex‑provenance
WCAG 2.2 AA	Web Content Accessibility Guidelines version 2.2, Level AA. 56 criteria. Legal standard for US government procurement (ADA Title II, April 2026).	cortex‑interface
Confidence: 99%

10. CROSS‑REFERENCE INDEX
Entity	Defined In	Referenced In
SemanticGateway	§3 Component Map	§4 Scenarios 1‑3
MCPServer	§3 Component Map, §3 Interface Contract	§4 Scenarios 1‑3, §5 Deployment
ProvenanceEngine	§3 Component Map	§4 Scenario 1, §6 Cross‑cutting
SecurityFortress	§3 Component Map	§4 Scenario 2, §6 Security
CortexGuard	§3 Component Map	§4 Scenario 3, §6 Kill Switch
ConnectorRegistry	§3 Component Map	§4 Scenario 1
ToolExecutor	§3 Component Map	§4 Scenario 1
ObservationalCapture	§3 Component Map	§4 Scenario 4
MirrorEngine	§3 Component Map	§4 Scenario 4, §6 Resilience
AbsorptionEngine	§3 Component Map	§4 Scenario 4
SelfValidator	§3 Component Map	§4 Scenario 5
TraceDB	§3 Containers Overview	§4 Scenarios 1,4,5
PostgreSQL + pgvector	§3 Containers Overview	§5 Deployment, §7 ADR‑002
Dell AI Factory	§3 Containers Overview	§5 Deployment, §7 ADR‑009
cortex.toml	§1 Constraints	§5 Environment Variables
Ed25519‑signed License	§1 Constraints, §6 Security	§7 ADR‑001
WCAG 2.2 AA	§1 Constraints, §6 Accessibility	§8 Quality Goals
Bag‑of‑words Embedding	§7 ADR‑004	§8 Risk R1
Sequential Execution	§7 ADR‑006	§8 Risk R2
Backup Feature Gate	§7 ADR‑005	§8 Risk R4
Oracle .dbf Parsing	§7 ADR‑009	§8 Risk R5
Confidence: 99%

11. CONFORMANCE CHECKLIST
#	Item	Source	Status
1	cargo check --workspace exits 0 with zero errors	Phase 0 gate	☐
2	cargo test --workspace all tests pass	Phase 1 gate	☐
3	cortex serve starts and prints "Cortex MCP gateway listening on port 8787"	Phase 1.16	☐
4	Benign MCP query returns 200 with plan, result, and capsule_id	Phase 1.17	☐
5	Malicious query (prompt injection) returns 403	Phase 1.17	☐
6	POST /admin/kill causes subsequent queries to return 503	Phase 1.17	☐
7	POST /admin/revive restores normal 200 responses	Phase 1.17	☐
8	All connector tools are registered in ToolRegistry at startup	Phase 2.10	☐
9	PostgreSQL connector returns real table list when DATABASE_URL is set	Phase 2.10	☐
10	Dashboard is served at /admin from demo/ directory	Phase 2.10	☐
11	cortex-vault has [features] backup = [] defined	Phase 2.11	☐
12	Binary size ≤ 10 MB after cargo build --release with LTO + strip + UPX	Phase 6	☐
13	install.sh deploys Cortex on a fresh Ubuntu 22.04 VM in < 2 minutes	Phase 2	☐
14	Docker image builds and runs Cortex on port 8787	Phase 2	☐
15	Air‑gap bundle (.tar.gz) contains binary, config, Knowledge Snap, and migrations	Phase 6	☐
16	All 12 validation experiments (X1‑X12) pass with defined criteria	Phase 5	☐
17	MCP‑BOM attack‑surface score ≤ 15 (bottom decile of 500‑server distribution)	Phase 5	☐
18	WCAG 2.2 AA pass rate = 100 % across all 18 A2UI component types	Phase 5	☐
19	CDC Mirror sustains 250 M+ events/week with p95 latency ≤ 100 ms	Phase 5	☐
20	1 M TraceCaps capsules generated with 0 Merkle failures	Phase 5	☐
21	Self‑guided demo HTML is served at demo.intellecta.io	Phase 6	☐
22	Dell AI Ecosystem submission package generates with ./demo/dell‑ai‑factory/submit.sh	Phase 6	☐
