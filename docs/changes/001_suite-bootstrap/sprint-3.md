# Sprint 3 — Freeze the interface types, CI and the private repo layout

## Goal

Merge the public types and signatures that `002_pump-core` and `003_report-kit` code against, with no logic. Add a 3-OS CI pipeline and write the private data repo layout. Closing this sprint is gate **G0**: D-008 moves to accepted.

## Files

**Modify:**
- root `Cargo.toml` `[workspace.dependencies]`:
  - `uom = { version = "0.36", default-features = false, features = ["f64", "si", "std", "use_serde"] }` (or the current version; check `cargo deny`);
  - `thiserror = "2"`.
- `crates/pump-core/Cargo.toml`: `uom`, `serde`, `thiserror`, `evidence-format` (path).
- `crates/report-kit/Cargo.toml`: `serde`, `thiserror`, `evidence-format` (path).
- `docs/decisions.md`: D-008 status → `accepted (G0, <date>)`.
- `docs/journey.md`:
  - P0 row → ✅;
  - G0 → ✅;
  - P2 and P3 → "ready to register";
  - a log entry.

**Create, in `crates/pump-core/src/`**: the types only. Every function body returns `Err(PumpError::NotImplemented)`. All types derive `Debug, Clone, serde::Serialize, serde::Deserialize`. Quantities use `uom::si::f64` types.
- `lib.rs`: module declarations and re-exports.
- `job.rs`:
  - `Job { pump: PumpId, rated: RatedPoint, setup: TestSetup, fit: FitOptions }`;
  - `PumpId { tag: String, service: Option<String>, manufacturer: Option<String>, model: Option<String>, serial: Option<String>, stages: u16 }`;
  - `RatedPoint { flow: VolumeRate, head: Length, power: Power, speed: AngularVelocity, efficiency: Option<Ratio>, npshr: Option<Length> }`;
  - `TestSetup { suction_id: Length, discharge_id: Length, z_suction: Length, z_discharge: Length, density: MassDensity, power_basis: PowerBasis }`;
  - `enum PowerBasis { Shaft, MotorInput { motor_efficiency: Ratio } }`;
  - `FitOptions { degree_head: u8, degree_power: u8, degree_efficiency: u8 }`, with `Default` 3/3/2.
- `point.rs`:
  - `TestPoint { flow: VolumeRate, p_suction: Pressure, p_discharge: Pressure, speed: AngularVelocity, power: Power }`;
  - `ReducedPoint { measured: TestPoint, head: Length, hydraulic_power: Power, shaft_power: Power, efficiency: Ratio, speed_deviation: Ratio, at_rated_speed: RatedSpeedPoint }`;
  - `RatedSpeedPoint { flow: VolumeRate, head: Length, shaft_power: Power }`.
- `curve.rs`:
  - `Curve { degree: u8, coefficients: Vec<f64> /* SI, ascending powers of flow in m3/s */, r_squared: f64, rms_residual: f64 }`;
  - `Curves { head: Curve, power: Curve, efficiency: Curve }`;
  - `RatedReadout { head: Length, power: Power, efficiency: Ratio, shutoff_head: Option<Length> }`.
- `tolerance.rs`:
  - `StandardRef { name: String, edition: String, source: String }`;
  - `ToleranceSet { standard: StandardRef, example: bool, criteria: Vec<Criterion> }`;
  - `Criterion { id: String, kind: CriterionKind, clause: String }`;
  - `enum CriterionKind`:
    - `RelativeBand { quantity: Quantity, bands: Vec<Band> }`;
    - `MaxRelative { quantity: Quantity, upper_pct: f64 }`;
    - `MaxAbsoluteVsGuarantee { quantity: Quantity }`;
    - `SpeedDeviationMax { max_pct: f64 }`;
    - `HeadRisingToShutoff`;
    - `ShutoffBand { lower_pct: f64, upper_pct: f64 }`;
  - `Band { applies_if_rated_head_max: Option<Length>, lower_pct: f64, upper_pct: f64 }`;
  - `enum Quantity { Head, Power, Efficiency, Npshr }`;
  - `pub fn parse_tolerances(toml_text: &str) -> Result<ToleranceSet, PumpError>` (stub).
- `verdict.rs`:
  - `Limit { lower: Option<f64>, upper: Option<f64>, unit: String }`;
  - `enum VerdictStatus { Pass, Fail, NotEvaluated { reason: String } }`;
  - `Verdict { criterion_id: String, clause: String, limit: Limit, measured: Option<f64>, deviation_pct: Option<f64>, status: VerdictStatus }`;
  - `enum Outcome { Pass, Fail }`.
- `report.rs`:
  - `AcceptanceReport { job: Job, points: Vec<ReducedPoint>, curves: Curves, at_rated: RatedReadout, verdicts: Vec<Verdict>, overall: Outcome, standard: StandardRef, tool: evidence_format::ToolInfo }`;
  - `pub fn evaluate(job: &Job, points: &[TestPoint], tolerances: &ToleranceSet) -> Result<AcceptanceReport, PumpError>` (stub).
- `error.rs`: `#[non_exhaustive] enum PumpError { NotImplemented, InvalidInput { field: String, reason: String }, Tolerance(String), Fit(String) }` via `thiserror`.

**Create, in `crates/report-kit/src/`**: the types only. Function bodies return `Err(ReportError::NotImplemented)`.
- `doc.rs`:
  - `ReportDoc { title: String, subtitle: Option<String>, meta: Vec<(String, String)>, sections: Vec<Section> }`;
  - `Section { heading: String, blocks: Vec<Block> }`;
  - `enum Block { Paragraph(String), Table(Table), Image(Figure), Verdict { label: String, pass: bool, detail: String } }`;
  - `Table { header: Vec<String>, rows: Vec<Vec<String>>, caption: Option<String> }`;
  - `Figure { png: Vec<u8>, caption: String, width_mm: f32 }`.
- `chart.rs`:
  - `ChartSpec { title: String, x: Axis, y: Axis, y2: Option<Axis>, series: Vec<Series>, boxes: Vec<ToleranceBox> }`;
  - `Axis { label: String, unit: String, min: Option<f64>, max: Option<f64> }`;
  - `Series { name: String, style: SeriesStyle, points: Vec<(f64, f64)>, secondary_axis: bool }`;
  - `enum SeriesStyle { Points, Line, Marker }`;
  - `ToleranceBox { x_min: f64, x_max: f64, y_min: f64, y_max: f64, label: String }`;
  - `pub fn chart_png(spec: &ChartSpec, width_px: u32, height_px: u32) -> Result<Vec<u8>, ReportError>`.
- `render.rs`: `pub fn to_docx(doc: &ReportDoc) -> Result<Vec<u8>, ReportError>`; `pub fn to_pdf(doc: &ReportDoc) -> Result<Vec<u8>, ReportError>`.
- `package.rs`: `pub fn write_package(target: &std::path::Path, manifest: &evidence_format::Manifest, files: &[(String, Vec<u8>)], overwrite: bool) -> Result<(), ReportError>`.
- `error.rs`: `#[non_exhaustive] enum ReportError { NotImplemented, Invalid(String), Io(String), Render(String) }`.

**Create also:**
- `.github/workflows/ci.yml`:
  - triggers: `push` to `main` and `pull_request`;
  - matrix `os: [ubuntu-latest, macos-14, windows-latest]`;
  - steps: checkout → `dtolnay/rust-toolchain@stable` (with `rustfmt`, `clippy`) → `Swatinem/rust-cache` → `cargo fmt --check` (ubuntu only) → `cargo clippy --workspace --all-targets -- -D warnings` → `cargo test --workspace`;
  - a separate `deny` job on ubuntu with `EmbarkStudios/cargo-deny-action`;
  - all third-party actions pinned by full commit SHA, with the version in a comment (as in `~/GitHub/pdfcraft.back/.github/workflows/ci.yml`);
  - `permissions: contents: read`.
- `docs/private-repo-layout.md`: the layout of `eu-cristofer/costa-inspection-private`:
  ```
  README.md            # rules: never push to a public remote; anonymise clients (Client-A, Job-001)
  tolerances/README.md # file format = docs/tolerances.md (written in 002); values only from the licensed API 610 copy;
                       # file name api610-<edition>.toml; `source` field names table, person and date
  tolerances/          # one file per edition
  golden/README.md     # case layout: <case>/{job.toml,test.csv,tolerances.toml,expected.toml}; expected = signed-off numbers
  golden/
  templates/           # client report expectations (U4)
  ```
  The harness in `002_pump-core` reads `SUITE_PRIVATE_GOLDEN=<path>/golden`.

## Reuse

- Contract shapes from `docs/plan/projects/P0-suite-bootstrap.md` §Sprint 3 (copied above as the source of truth).
- Action pins and the workflow shape from `~/GitHub/pdfcraft.back/.github/workflows/ci.yml`.

## Tests

- `crates/pump-core/tests/contract.rs`:
  - build a sample `Job`, `ToleranceSet` and `AcceptanceReport` by hand, serialise to JSON and back, and assert equality (proves the contract is serialisable);
  - `evaluate` returns `Err(PumpError::NotImplemented)`.
- `crates/report-kit/tests/contract.rs`: the same for `ReportDoc` and `ChartSpec`; each stub returns `NotImplemented`.

## Acceptance

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

CI green on all 3 OSes once the repo is on GitHub. If the remote does not exist yet, record "CI pending remote" in the journey; that does not block ticking.

## Definition of done

- All acceptance commands pass.
- The contract types are merged.
- D-008 is accepted.
- The journey shows G0 ✅ and P2/P3 ready to register.
- The CHANGELOG has a bullet.
- The tracker is ticked, and the project is marked `Status: ready to archive`.

## Assumes

- Sprint 1: workspace and stubs.
- Sprint 2: `evidence-format` with `Manifest` and `ToolInfo`.
