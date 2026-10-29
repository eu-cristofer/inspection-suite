//! Domain-neutral report rendering: ReportDoc to DOCX and PDF, charts to PNG, atomic evidence-package writer.
//!
//! The public types are a frozen contract (decision D-008): changes need a decision entry in `docs/decisions.md` and a heads-up
//! to the pump-core and report-kit lanes.

mod chart;
mod doc;
mod error;
mod package;
mod render;

pub use chart::{Axis, ChartSpec, Series, SeriesStyle, ToleranceBox, chart_png};
pub use doc::{Block, Figure, ReportDoc, Section, Table};
pub use error::ReportError;
pub use package::write_package;
pub use render::{to_docx, to_pdf};
