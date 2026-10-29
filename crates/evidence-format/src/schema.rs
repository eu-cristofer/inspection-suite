//! JSON Schema generation.

use serde_json::Value;

use crate::error::FormatError;
use crate::manifest::{Manifest, PackageKind};

/// Returns the pretty-printed JSON Schema of the manifest, titled for the given package kind.
pub fn json_schema(kind: PackageKind) -> Result<String, FormatError> {
    let mut schema = schemars::schema_for!(Manifest);
    schema.insert("title".to_string(), Value::String(format!("{} manifest", kind.as_str())));
    schema.insert("x-package-kind".to_string(), Value::String(kind.as_str().to_string()));
    let mut text = serde_json::to_string_pretty(&schema).map_err(|e| FormatError::Json(e.to_string()))?;
    text.push('\n');
    Ok(text)
}
