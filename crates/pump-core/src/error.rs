//! Error type for pump-core.

/// Errors returned by pump-core operations.
#[derive(Debug, thiserror::Error, PartialEq)]
#[non_exhaustive]
pub enum PumpError {
    /// The operation is a frozen-interface stub with no logic yet.
    #[error("not implemented yet")]
    NotImplemented,
    /// An input value was missing or out of range.
    #[error("invalid input `{field}`: {reason}")]
    InvalidInput {
        /// Name of the offending field.
        field: String,
        /// Why the value was rejected.
        reason: String,
    },
    /// The tolerance file could not be parsed or is inconsistent.
    #[error("tolerance file: {0}")]
    Tolerance(String),
    /// The curve fit failed.
    #[error("curve fit: {0}")]
    Fit(String),
}
