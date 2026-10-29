//! Evidence-package manifest contract: envelope, per-kind payloads, JSON Schema and validation.
//!
//! Two package kinds are supported:
//! - `pdfcraft.snapshot_export` (versions 1.0 and 1.1), produced by the PdfCraft fork and read unchanged, without field loss;
//! - `suite.pump_test` (version 1.0).
//!
//! The manifest is canonical: tools exchange evidence packages only through it. Unknown fields are preserved in `extra` maps, so
//! round-tripping never drops data.
//!
//! Versioning policy: additive fields bump the minor version; renames or removals bump the major version and need a decision
//! entry in `docs/decisions.md`.

mod error;
mod manifest;
mod paths;
mod schema;
mod validate;

pub use error::FormatError;
pub use manifest::{Item, Manifest, PackageKind, ToolInfo, from_json, to_json};
pub use paths::normalize_package_relative_path;
pub use schema::json_schema;
pub use validate::{ValidationIssue, validate};
