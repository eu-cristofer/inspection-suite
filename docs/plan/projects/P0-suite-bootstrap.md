# P0 — Suite bootstrap (`001_suite-bootstrap`)

Repo: `inspection-suite` (new) · Lane: user + Claude, sequential · Depends on: — · Gate: **G0**

## Goal

Create the suite workspace, freeze the interface contracts, and get CI green, so that lanes B (P2) and C (P3) can work in parallel without editing shared files.

## Scope

- **In:** repo skeleton, conventions, licence files, stub crates for every MVP member, `evidence-format` v1, frozen interface types for `pump-core` and `report-kit`, CI, and the private repo layout.
- **Out:** any calculation, any report rendering, any CLI behaviour.

## Target layout

```
inspection-suite/
├── Cargo.toml              # [workspace] members, [workspace.package], [workspace.lints], [workspace.dependencies]
├── LICENSE-MIT  LICENSE-APACHE  NOTICE  README.md  CHANGELOG.md
├── CLAUDE.md  AGENTS.md    # conventions (see Sprint 1)
├── deny.toml  rustfmt.toml  clippy.toml
├── .github/workflows/ci.yml
├── crates/
│   ├── evidence-format/    # real code (Sprint 2)
│   ├── pump-core/          # stub + frozen types (Sprint 3)
│   └── report-kit/         # stub + frozen types (Sprint 3)
├── apps/pump-cli/          # stub binary printing --version
├── examples/               # synthetic job.toml / test.csv / tolerances.example.toml (filled by P2/P4)
└── docs/ plan/ journey.md decisions.md changes/ archive/
```

## Sprints

### Sprint 1 — Workspace skeleton and conventions
- **Files:**
  - root `Cargo.toml` (edition 2024, `rust-version` pinned, `[workspace.lints]`: forbid `unsafe_code`, deny `clippy::unwrap_used`, `clippy::expect_used` and `clippy::panic` outside tests);
  - licences and `NOTICE` (copyright line: "Copyright (c) 2026 Costa Inspection and the `<Suite>` contributors");
  - `CLAUDE.md`/`AGENTS.md`;
  - `deny.toml`, copied from `pdfcraft.back/deny.toml` (allow-list: MIT, MIT-0, Apache-2.0, BSD-2/3, ISC, Zlib, Unicode-3.0, BSL-1.0, CC0-1.0);
  - `rustfmt.toml`, `clippy.toml`;
  - the stub crates (`lib.rs` containing only a doc comment) and `apps/pump-cli` printing the version.
- **Reuse:** convention text from `pdfcraft.back/CLAUDE.md`: never panic, review at four zooms, prove that a test fails, scoped builds.
- **Additional rules for `CLAUDE.md`:**
  - no standard text (API/ISO/HI) in the repo;
  - no client data;
  - acceptance limits only from data files.
- **Tests:** `cargo build --workspace`.
- **Acceptance:** `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`.
- **Done:** workspace builds; every crate has `[lints] workspace = true`.
- **Assumes:** empty repo `eu-cristofer/inspection-suite` created on GitHub.

### Sprint 2 — `evidence-format` v1
- **Model.** A common envelope shared by all package kinds, plus typed payloads per kind.
  ```rust
  pub struct Manifest { pub package_kind: PackageKind, pub manifest_version: String,
                        pub tool: ToolInfo, pub created_utc: String, pub title: String,
                        pub items: Vec<Item>, #[serde(flatten)] pub extra: Map<String, Value> }
  pub enum PackageKind { SnapshotExport /* "pdfcraft.snapshot_export" */, PumpTest /* "suite.pump_test" */ }
  pub struct ToolInfo { pub name: String, pub version: String, pub commit: Option<String> }
  pub struct Item { pub id: String /* item_N */, pub kind: String, pub evidence: Vec<String> /* package-relative */,
                    #[serde(flatten)] pub payload: Map<String, Value> }
  pub fn from_json(&str) -> Result<Manifest, FormatError>;  pub fn to_json(&Manifest) -> Result<String, FormatError>;
  pub fn validate(&Manifest) -> Result<(), Vec<ValidationIssue>>;   // ids unique & ordered, paths relative & normalized, kind known for package_kind
  pub fn json_schema(kind: PackageKind) -> String;                  // via schemars
  ```
- **Compatibility requirement.** The crate must **read the fork's existing v1.1 snapshot manifests unchanged**. Copy one real manifest produced by the fork's tests into `tests/fixtures/snapshot_v1_1.json`, and require a round-trip that preserves every field (`extra` keeps unknown fields).
- **Reuse:** the field set and rules in `pdfcraft.back/crates/engine/src/export.rs`:
  - `snapshot_export_manifest_json` (l.858);
  - `normalize_package_relative_path` (l.681);
  - the constants at l.734–749 (`pdfcraft.snapshot_export`, `"1.1"`, `evidence/`).
  The normative spec is `pdfcraft.back/docs/snapshot-export-spec.md`.
- **Tests:**
  - fixture round-trip;
  - `validate` rejects `../x.png`, absolute paths, duplicate ids and an unknown kind;
  - a schema snapshot test.
- **Acceptance:** `cargo test -p evidence-format && cargo clippy -p evidence-format --all-targets -- -D warnings`.
- **Done:** `docs/evidence-format.md` describes the envelope and both package kinds.
- **Assumes:** Sprint 1.

### Sprint 3 — Freeze the interface types, CI and the private repo
- **`pump-core` public types only** (no logic; functions return `unimplemented`-free stubs: `Err(PumpError::NotImplemented)`):
  ```rust
  pub struct Job { pub pump: PumpId, pub rated: RatedPoint, pub setup: TestSetup, pub fit: FitOptions }
  pub struct PumpId { pub tag: String, pub service: Option<String>, pub manufacturer: Option<String>,
                      pub model: Option<String>, pub serial: Option<String>, pub stages: u16 }
  pub struct RatedPoint { pub flow: VolumeRate, pub head: Length, pub power: Power, pub speed: AngularVelocity,
                          pub efficiency: Option<Ratio>, pub npshr: Option<Length> }
  pub struct TestSetup { pub suction_id: Length, pub discharge_id: Length, pub z_suction: Length, pub z_discharge: Length,
                         pub density: MassDensity, pub power_basis: PowerBasis }
  pub enum PowerBasis { Shaft, MotorInput { motor_efficiency: Ratio } }
  pub struct TestPoint { pub flow: VolumeRate, pub p_suction: Pressure, pub p_discharge: Pressure,
                         pub speed: AngularVelocity, pub power: Power }
  pub struct ToleranceSet { pub standard: StandardRef, pub example: bool, pub criteria: Vec<Criterion> }
  pub struct AcceptanceReport { pub job: Job, pub points: Vec<ReducedPoint>, pub curves: Curves,
                                pub at_rated: RatedReadout, pub verdicts: Vec<Verdict>, pub overall: Outcome,
                                pub standard: StandardRef, pub tool: ToolInfo }
  pub fn evaluate(job: &Job, points: &[TestPoint], tolerances: &ToleranceSet) -> Result<AcceptanceReport, PumpError>;
  ```
  (`ReducedPoint`, `Curves`, `RatedReadout`, `Verdict`, `Criterion`, `Outcome` and `StandardRef` are declared with their fields; P2 fills in the logic. All of them are `Serialize`.)
- **`report-kit` public types only:**
  ```rust
  pub struct ReportDoc { pub title: String, pub subtitle: Option<String>, pub meta: Vec<(String, String)>, pub sections: Vec<Section> }
  pub struct Section { pub heading: String, pub blocks: Vec<Block> }
  pub enum Block { Paragraph(String), Table(Table), Image(Figure), Verdict { label: String, pass: bool, detail: String } }
  pub struct Table { pub header: Vec<String>, pub rows: Vec<Vec<String>>, pub caption: Option<String> }
  pub struct Figure { pub png: Vec<u8>, pub caption: String, pub width_mm: f32 }
  pub struct ChartSpec { pub title: String, pub x: Axis, pub y: Axis, pub series: Vec<Series>, pub boxes: Vec<ToleranceBox> }
  pub fn to_docx(&ReportDoc) -> Result<Vec<u8>, ReportError>;  pub fn to_pdf(&ReportDoc) -> Result<Vec<u8>, ReportError>;
  pub fn chart_png(&ChartSpec, width_px: u32, height_px: u32) -> Result<Vec<u8>, ReportError>;
  pub fn write_package(target: &Path, manifest: &Manifest, files: &[(String, Vec<u8>)]) -> Result<(), ReportError>;
  ```
- **CI:** `.github/workflows/ci.yml` runs fmt, clippy, test and deny on `ubuntu-latest`, `macos-14` and `windows-latest`, with actions pinned by SHA (as in the fork).
- **Private repo `costa-inspection-private`** (`eu-cristofer/costa-inspection-private`, private)**:**
  - `README.md` (rules: no public pushes, anonymise clients);
  - `tolerances/README.md`, which explains the file format and where values come from;
  - `golden/README.md`, which explains the case layout;
  - `templates/`.
- **Acceptance:** `cargo test --workspace`, and CI green on all 3 OSes.
- **Done:**
  - `docs/decisions.md` records "contract frozen" (D-008);
  - any change to these types afterwards needs a decision entry and a heads-up to both lanes.
- **Assumes:** Sprints 1–2.
