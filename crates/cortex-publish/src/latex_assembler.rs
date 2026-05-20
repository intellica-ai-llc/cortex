use cortex_validate::result_aggregator::AnalysisReport;
use super::figure_generator::RenderedFigure;
use super::table_generator::RenderedTable;

pub struct LatexAssembler;

impl LatexAssembler {
    pub fn new() -> Self { Self }

    /// Assemble a complete LaTeX document from figures and tables.
    ///
    /// Produces a compilable document with:
    ///   - Title, author, date
    ///   - Abstract (from the report)
    ///   - Figures section with all TikZ figures
    ///   - Results section with all LaTeX tables
    ///   - Data availability statement
    pub fn assemble(
        &self,
        report: &AnalysisReport,
        figures: &[RenderedFigure],
        tables: &[RenderedTable],
    ) -> String {
        let mut doc = String::new();
        doc.push_str(r"\documentclass{article}
\usepackage{booktabs}
\usepackage{pgfplots}
\usepackage{caption}
\usepackage{hyperref}
\title{Cortex Validation Report: ");
        doc.push_str(&report.experiment_name);
        doc.push_str(r"}
\author{Intellecta Cortex Validate\texttrademark}
\date{");
        doc.push_str(&report.generated_at.to_rfc3339());
        doc.push_str(r"}
\begin{document}
\maketitle
\begin{abstract}
This report presents the results of experiment ");
        doc.push_str(&report.experiment_id);
        doc.push_str(" (");
        doc.push_str(&report.experiment_name);
        doc.push_str(r") conducted on ");
        doc.push_str(&report.generated_at.to_rfc3339());
        doc.push_str(r".
\end{abstract}
\section{Figures}
");
        for fig in figures {
            doc.push_str(&fig.tikz_code);
            doc.push_str("\n");
        }
        doc.push_str(r"\section{Results}
");
        for tab in tables {
            doc.push_str(&tab.latex_code);
            doc.push_str("\n");
        }
        doc.push_str(r"\section{Data Availability}
All raw data, experiment parameters, and lineage metadata are available
in the Cortex TraceDB under lineage ID: ");
        doc.push_str(&report.lineage.lineage_id);
        doc.push_str(r". Content hash: ");
        doc.push_str(&report.content_hash);
        doc.push_str(r".
\end{document}");
        doc
    }
}
