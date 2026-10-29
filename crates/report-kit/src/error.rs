//! Error type for report-kit.

/// Errors returned by report-kit operations.
#[derive(Debug, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum ReportError {
    /// The operation is a frozen-interface stub with no logic yet.
    #[error("not implemented yet")]
    NotImplemented,
    /// The report input was invalid.
    #[error("invalid report input: {0}")]
    Invalid(String),
    /// An I/O operation failed.
    #[error("i/o: {0}")]
    Io(String),
    /// Rendering failed.
    #[error("render: {0}")]
    Render(String),
}
