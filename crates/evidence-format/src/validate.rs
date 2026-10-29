//! Manifest validation that reports every issue found.

use std::fmt;

use serde_json::Value;

use crate::error::FormatError;
use crate::manifest::Manifest;
use crate::paths::normalize_package_relative_path;

/// Maximum JSON nesting depth the path walk descends into.
const MAX_DEPTH: usize = 64;

/// A single validation problem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    /// JSON pointer to the offending value.
    pub pointer: String,
    /// Human-readable description.
    pub message: String,
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.pointer, self.message)
    }
}

fn issue(issues: &mut Vec<ValidationIssue>, pointer: impl Into<String>, message: impl Into<String>) {
    issues.push(ValidationIssue { pointer: pointer.into(), message: message.into() });
}

/// Appends an escaped JSON-pointer segment (RFC 6901).
fn child(base: &str, segment: &str) -> String {
    format!("{base}/{}", segment.replace('~', "~0").replace('/', "~1"))
}

/// Validates a manifest, collecting all issues instead of stopping at the first.
pub fn validate(m: &Manifest) -> Result<(), Vec<ValidationIssue>> {
    let mut issues = Vec::new();

    if !m.package_kind.supported_versions().contains(&m.manifest_version.as_str()) {
        issue(
            &mut issues,
            "/manifest_version",
            format!(
                "unsupported manifest_version `{}` for {} (supported: {})",
                m.manifest_version,
                m.package_kind.as_str(),
                m.package_kind.supported_versions().join(", ")
            ),
        );
    }

    let allowed = m.package_kind.allowed_item_kinds();
    for (i, item) in m.items.iter().enumerate() {
        let expected = format!("item_{}", i.saturating_add(1));
        if item.id != expected {
            issue(&mut issues, format!("/items/{i}/id"), format!("expected id `{expected}`, found `{}`", item.id));
        }
        if !allowed.contains(&item.kind.as_str()) {
            issue(
                &mut issues,
                format!("/items/{i}/kind"),
                format!("kind `{}` not allowed for {} (allowed: {})", item.kind, m.package_kind.as_str(), allowed.join(", ")),
            );
        }
    }

    match serde_json::to_value(m) {
        Ok(value) => walk(&value, "", 0, &mut issues),
        Err(e) => issue(&mut issues, "", format!("cannot serialize manifest: {e}")),
    }

    if issues.is_empty() { Ok(()) } else { Err(issues) }
}

fn check_path(s: &str, pointer: &str, issues: &mut Vec<ValidationIssue>) {
    match normalize_package_relative_path(s) {
        Ok(normalized) if normalized == s => {}
        Ok(normalized) => issue(issues, pointer, format!("path `{s}` is not normalized (expected `{normalized}`)")),
        Err(FormatError::Path(msg)) => issue(issues, pointer, format!("{msg}: `{s}`")),
        Err(other) => issue(issues, pointer, other.to_string()),
    }
}

/// Walks the manifest JSON and checks every `path`/`*_path` string and every `evidence` array entry.
fn walk(value: &Value, pointer: &str, depth: usize, issues: &mut Vec<ValidationIssue>) {
    if depth > MAX_DEPTH {
        issue(issues, pointer, "nesting too deep");
        return;
    }
    match value {
        Value::Object(map) => {
            for (key, v) in map {
                let p = child(pointer, key);
                if (key == "path" || key.ends_with("_path"))
                    && let Value::String(s) = v
                {
                    check_path(s, &p, issues);
                }
                if key == "evidence"
                    && let Value::Array(arr) = v
                {
                    for (i, el) in arr.iter().enumerate() {
                        if let Value::String(s) = el {
                            check_path(s, &format!("{p}/{i}"), issues);
                        }
                    }
                }
                walk(v, &p, depth.saturating_add(1), issues);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                walk(v, &format!("{pointer}/{i}"), depth.saturating_add(1), issues);
            }
        }
        _ => {}
    }
}
