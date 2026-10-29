# Sprint 2 — `evidence-format` v1

## Goal

Implement the shared evidence-package manifest contract:
- typed envelope, JSON (de)serialisation, validation and JSON Schema;
- reads the PdfCraft fork's existing `pdfcraft.snapshot_export` v1.1 manifests **unchanged**;
- adds the new `suite.pump_test` package kind.

## Files

**Modify:**
- `Cargo.toml` (root): `[workspace.dependencies]` gets `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"` (with `preserve_order`) and `schemars = "0.8"` (or the current major; check `cargo deny`).
- `crates/evidence-format/Cargo.toml`: depend on these.

**Create, in `crates/evidence-format/src/`:**
- `lib.rs`: the public API, re-exports and crate docs.
- `manifest.rs`:
  ```rust
  pub struct Manifest {
      pub package_kind: PackageKind,          // "pdfcraft.snapshot_export" | "suite.pump_test"
      pub manifest_version: String,           // "1.1" for snapshot_export, "1.0" for pump_test
      pub items: Vec<Item>,
      #[serde(flatten)] pub extra: serde_json::Map<String, serde_json::Value>, // source_pdf, coordinate_space, rendering, tool, title, created_utc, …
  }
  pub enum PackageKind { SnapshotExport, PumpTest }   // serde rename to the exact strings
  pub struct Item {
      pub id: String,                          // "item_N", 1-based, contiguous, in order
      pub kind: String,                        // e.g. "snapshot", "pump_acceptance", "chart", "data"
      #[serde(flatten)] pub extra: serde_json::Map<String, serde_json::Value>,
  }
  pub fn from_json(text: &str) -> Result<Manifest, FormatError>;
  pub fn to_json(m: &Manifest) -> Result<String, FormatError>;   // 2-space pretty, stable key order
  ```
  `ToolInfo { name, version, commit: Option<String> }` is a typed helper for writers of `suite.pump_test` (stored as the `tool` key in `extra`); snapshot manifests may lack it.
- `validate.rs`: `pub fn validate(m: &Manifest) -> Result<(), Vec<ValidationIssue>>`. It checks:
  - `manifest_version` is supported for the kind;
  - item ids are unique and `item_1..item_n` in order;
  - item `kind` is allowed for the package kind (snapshot_export: `snapshot`; pump_test: `pump_acceptance`, `chart`, `data`, `input`);
  - every string value under any key named `path` or ending in `_path`, plus every entry of an `evidence` array, is a normalised package-relative path (no absolute paths, no `..`, no backslashes, no empty segments).
  All issues are collected, not just the first.
- `paths.rs`: `pub fn normalize_package_relative_path(p: &str) -> Result<String, FormatError>`. **Port the rules** of the function of the same name in `~/GitHub/pdfcraft.back/crates/engine/src/export.rs:681`.
- `schema.rs`: `pub fn json_schema(kind: PackageKind) -> String` via `schemars`.
- `error.rs`: `FormatError` and `ValidationIssue` (path-in-document plus message). Implement `Display` and `std::error::Error`.

**Create also:**
- `crates/evidence-format/tests/fixtures/snapshot_v1_1.json`: a real manifest produced by the fork. Generate it by running the fork's package test, or copy the expected JSON from the tests in `~/GitHub/pdfcraft.back/crates/engine/src/export.rs` (module `snapshot_export_package_tests`, l.2476+). The writer is `snapshot_export_manifest_json` at l.858.
- `crates/evidence-format/tests/fixtures/pump_test_v1_0.json`: a hand-written minimal example.
- `docs/evidence-format.md`: the envelope, both package kinds, item kinds, the path rules and the versioning policy (additive fields are minor; renames and removals are major and need a decision entry). Link it to the fork's normative spec (`docs/snapshot-export-spec.md` in `eu-cristofer/pdfcraft`).

## Reuse

- `~/GitHub/pdfcraft.back/crates/engine/src/export.rs`:
  - constants l.734–749 (`SNAPSHOT_EXPORT_PACKAGE_KIND = "pdfcraft.snapshot_export"`, `SNAPSHOT_EXPORT_MANIFEST_VERSION = "1.1"`, the `evidence/` dir);
  - `snapshot_export_manifest_json` (l.858), for the field set and order;
  - `normalize_package_relative_path` (l.681).
- `~/GitHub/pdfcraft.back/docs/snapshot-export-spec.md`: the normative rules (manifest canonical, ids, ordering).

## Tests

In `crates/evidence-format/tests/`:
- **round_trip:** `snapshot_v1_1.json` → `from_json` → `to_json` gives JSON values equal to the original (compare as `serde_json::Value`, so formatting differences don't matter). The same for `pump_test_v1_0.json`.
- **validate_ok:** both fixtures validate.
- **validate_rejects** (one test each): `../x.png`, `/abs.png`, `evidence\\x.png`, `a//b.png`, duplicate id, out-of-order id (`item_2` before `item_1`), unknown item kind for the package kind, unsupported `manifest_version`. Assert the specific `ValidationIssue`.
- **multiple_issues:** a manifest with 3 problems reports all 3.
- **schema_snapshot:** `json_schema(PumpTest)` matches a committed snapshot file (update it deliberately).
- **paths:** a unit test per normalisation rule, ported from the fork's behaviour.

## Acceptance

```bash
cargo fmt --check
cargo clippy -p evidence-format --all-targets -- -D warnings
cargo test -p evidence-format
cargo deny check
```

## Definition of done

- All acceptance commands pass.
- The real fork fixture round-trips with no field loss.
- `docs/evidence-format.md` exists.
- The CHANGELOG has a bullet.
- The tracker is ticked.

## Assumes

- Sprint 1: the workspace, lints, the `evidence-format` stub and `deny.toml` exist.
- The fork checkout exists at `~/GitHub/pdfcraft.back` (branch `feat/crop-panel-unification`) for the fixture and the reference code.
