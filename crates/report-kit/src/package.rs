//! Atomic evidence-package writer.

use std::path::Path;

use crate::error::ReportError;

/// Atomically writes an evidence package (manifest plus files) to `target`.
///
/// Stub in the frozen interface: always returns [`ReportError::NotImplemented`].
pub fn write_package(target: &Path, manifest: &evidence_format::Manifest, files: &[(String, Vec<u8>)], overwrite: bool) -> Result<(), ReportError> {
    let _ = (target, manifest, files, overwrite);
    Err(ReportError::NotImplemented)
}
