//! Cortex Publish — Figure & Table Renderer for Validation Results.
//!
//! Generates publication-grade visualizations from AnalysisReports
//! without agent-driven paper writing. The researcher interprets;
//! Cortex renders data and figures.
//!
//! Output formats:
//!   - Vega-Lite JSON specs (interactive exploration)
//!   - TikZ/PGFPlots figures (LaTeX-native, SSVG-Bench quality)
//!   - LaTeX tabular tables (ready for manuscript integration)
//!   - CSV raw data (for external analysis tools)

pub mod figure_generator;
pub mod table_generator;
pub mod latex_assembler;

use std::sync::Arc;

pub struct PublishEngine {
    pub figure_gen: Arc<figure_generator::FigureGenerator>,
    pub table_gen: Arc<table_generator::TableGenerator>,
    pub latex_assembler: Arc<latex_assembler::LatexAssembler>,
}

impl PublishEngine {
    pub fn new() -> Self {
        Self {
            figure_gen: Arc::new(figure_generator::FigureGenerator::new()),
            table_gen: Arc::new(table_generator::TableGenerator::new()),
            latex_assembler: Arc::new(latex_assembler::LatexAssembler::new()),
        }
    }

    /// Process an AnalysisReport into a complete publication bundle.
    ///
    /// Produces:
    ///   - `figures/` directory with TikZ .tex files and rendered PDFs
    ///   - `tables/` directory with LaTeX .tex files
    ///   - `data/` directory with CSV exports
    ///   - `results.tex` master document assembling all components
    pub fn publish(
        &self,
        report: &cortex_validate::result_aggregator::AnalysisReport,
    ) -> PublishBundle {
        let figures = self.figure_gen.generate_from_report(report);
        let tables = self.table_gen.generate_from_report(report);
        let master = self.latex_assembler.assemble(report, &figures, &tables);

        PublishBundle {
            report_id: report.report_id.clone(),
            figures,
            tables,
            master_document: master,
            generated_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublishBundle {
    pub report_id: String,
    pub figures: Vec<figure_generator::RenderedFigure>,
    pub tables: Vec<table_generator::RenderedTable>,
    pub master_document: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}
