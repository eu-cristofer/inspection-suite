//! Manifest envelope and (de)serialization.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::FormatError;

/// The evidence-package manifest envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Manifest {
    /// Which kind of package this manifest describes.
    pub package_kind: PackageKind,
    /// Manifest version, e.g. `"1.1"`.
    pub manifest_version: String,
    /// Evidence items, ids `item_1..item_n` in order.
    pub items: Vec<Item>,
    /// All other top-level keys, preserved verbatim.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Known package kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum PackageKind {
    /// PdfCraft snapshot export.
    #[serde(rename = "pdfcraft.snapshot_export")]
    SnapshotExport,
    /// Pump acceptance-test package.
    #[serde(rename = "suite.pump_test")]
    PumpTest,
}

impl PackageKind {
    /// The wire name of this kind.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotExport => "pdfcraft.snapshot_export",
            Self::PumpTest => "suite.pump_test",
        }
    }

    /// Manifest versions this crate can read for the kind.
    pub fn supported_versions(self) -> &'static [&'static str] {
        match self {
            Self::SnapshotExport => &["1.0", "1.1"],
            Self::PumpTest => &["1.0"],
        }
    }

    /// Item kinds allowed in packages of this kind.
    pub fn allowed_item_kinds(self) -> &'static [&'static str] {
        match self {
            Self::SnapshotExport => &["snapshot"],
            Self::PumpTest => &["pump_acceptance", "chart", "data", "input"],
        }
    }
}

/// One evidence item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Item {
    /// Item id, `item_{n}` with n starting at 1.
    pub id: String,
    /// Item kind, constrained by the package kind.
    pub kind: String,
    /// Kind-specific fields, preserved verbatim.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Producing tool, stored under the optional top-level `tool` key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ToolInfo {
    /// Tool name.
    pub name: String,
    /// Tool version.
    pub version: String,
    /// Optional source commit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

impl Manifest {
    /// Creates an empty manifest of the given kind and version.
    pub fn new(package_kind: PackageKind, manifest_version: impl Into<String>) -> Self {
        Self { package_kind, manifest_version: manifest_version.into(), items: Vec::new(), extra: serde_json::Map::new() }
    }

    /// Typed accessor for the optional `tool` key in `extra`.
    pub fn tool(&self) -> Result<Option<ToolInfo>, FormatError> {
        match self.extra.get("tool") {
            None => Ok(None),
            Some(value) => serde_json::from_value::<ToolInfo>(value.clone()).map(Some).map_err(|e| FormatError::Tool(e.to_string())),
        }
    }

    /// Sets the `tool` key.
    pub fn set_tool(&mut self, tool: &ToolInfo) -> Result<(), FormatError> {
        let value = serde_json::to_value(tool).map_err(|e| FormatError::Tool(e.to_string()))?;
        self.extra.insert("tool".to_string(), value);
        Ok(())
    }
}

/// Parses a manifest from JSON text.
pub fn from_json(text: &str) -> Result<Manifest, FormatError> {
    serde_json::from_str(text).map_err(|e| FormatError::Json(e.to_string()))
}

/// Serializes a manifest as pretty JSON with a trailing newline.
pub fn to_json(manifest: &Manifest) -> Result<String, FormatError> {
    let mut text = serde_json::to_string_pretty(manifest).map_err(|e| FormatError::Json(e.to_string()))?;
    text.push('\n');
    Ok(text)
}
