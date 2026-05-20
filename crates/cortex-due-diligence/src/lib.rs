//! Cortex Due Diligence — Automated AI M&A Technical Due Diligence Report.
//!
//! Based on the Skadden AI M&A due diligence framework [reference:19] and the
//! Agathon AI Due Diligence Checklist (40 verification points across
//! technical, commercial, regulatory, and talent dimensions) [reference:20].
//!
//! This crate generates a complete, reference‑grade technical due diligence
//! report suitable for:
//!   • Internal buyer engineering review (Dell AI Ecosystem validation)
//!   • Third‑party AI audit (EU AI Act, NERC CIP, SOC 2 readiness)
//!   • M&A target evaluation (buyer technical due diligence)
//!   • Venture capital technical assessment (Series A‑C)
//!
//! The report is structured as a single Markdown file that can be rendered
//! to PDF via pandoc or any Markdown‑to‑PDF converter.

pub mod dd_report_generator;
