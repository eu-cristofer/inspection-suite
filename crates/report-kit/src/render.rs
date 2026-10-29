//! Document renderers.

use crate::doc::ReportDoc;
use crate::error::ReportError;

/// Renders a report to DOCX bytes.
///
/// Stub in the frozen interface: always returns [`ReportError::NotImplemented`].
pub fn to_docx(doc: &ReportDoc) -> Result<Vec<u8>, ReportError> {
    let _ = doc;
    Err(ReportError::NotImplemented)
}

/// Renders a report to PDF bytes via embedded typst (decision D-010).
///
/// Stub in the frozen interface: always returns [`ReportError::NotImplemented`].
pub fn to_pdf(doc: &ReportDoc) -> Result<Vec<u8>, ReportError> {
    let _ = doc;
    Err(ReportError::NotImplemented)
}
