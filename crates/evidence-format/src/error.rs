//! Error type for the evidence-format crate.

/// Errors produced while reading, writing or interpreting a manifest.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FormatError {
    /// The JSON text could not be parsed or serialized.
    #[error("invalid manifest JSON: {0}")]
    Json(String),
    /// A package-relative path violated the path rules.
    #[error("invalid package path: {0}")]
    Path(String),
    /// The `tool` entry had the wrong shape.
    #[error("invalid `tool` entry: {0}")]
    Tool(String),
}
