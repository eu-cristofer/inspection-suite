//! Package-relative path normalization, ported from the PdfCraft fork's `normalize_package_relative_path`.

use std::path::{Component, Path};

use crate::error::FormatError;

/// Normalizes a package-relative path.
///
/// Rejects paths that are empty, absolute, use backslashes or escape the package root. Interior `.` segments and duplicate
/// separators are removed.
pub fn normalize_package_relative_path(path: &str) -> Result<String, FormatError> {
    let err = |m: &str| FormatError::Path(m.to_string());
    if path.is_empty() {
        return Err(err("package path is empty"));
    }
    if path.contains('\\') {
        return Err(err("package path must use '/' separators"));
    }
    // A leading '/' is rejected explicitly so Windows, where it is not "absolute", behaves the same.
    if path.starts_with('/') || Path::new(path).is_absolute() {
        return Err(err("package path must be relative"));
    }
    let mut parts = Vec::new();
    for component in Path::new(path).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => return Err(err("package path cannot escape the package root")),
            Component::RootDir | Component::Prefix(_) => return Err(err("package path must be relative")),
            Component::Normal(part) => parts.push(part.to_str().ok_or_else(|| err("package path must be valid UTF-8"))?),
        }
    }
    if parts.is_empty() {
        return Err(err("package path is empty"));
    }
    Ok(parts.join("/"))
}
