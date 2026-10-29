# P2 — Pump calculation core (`002_pump-core`)

Repo: `inspection-suite`, crate `crates/pump-core` · Lane: **B** · Depends on: P0 (G0) · Feeds: P4

## Goal

Build a pure, unit-safe library that turns API 610 shop performance-test data into an `AcceptanceReport` with per-criterion PASS/FAIL verdicts. The acceptance limits are loaded as data.

## Scope

- **In:**
  - units and parsing of unit-tagged values;
  - test-point reduction;
  - affinity-law speed correction;
  - curve fitting and readout at rated flow;
  - a data-driven acceptance engine;
  - the golden-test harness.
- **Out:**
  - file I/O (the caller passes `&str` or structs);
  - reports and charts;
  - NPSH-test reduction beyond comparing a supplied NPSHr value;
  - mechanical run, vibration and hydrostatic tests (backlog).

## Engineering rules

- **Typed units everywhere.** Use `uom` SI `f64` quantities. Raw `f64` is allowed only inside numeric kernels, with SI documented.
- **No hard-coded limits.** No API 610 number appears in code. Criteria, limits, the edition and clause/table references all come from `ToleranceSet`.
- **Self-describing verdicts.** Every `Verdict` carries: criterion id, clause reference (string from data), limit (lower/upper, unit), measured value, deviation in %, and pass/fail.
- **Explicit gravity.** `g` = 9.80665 m/s² (standard gravity), as a named constant.
- **Total functions.** Empty, NaN, zero-speed, zero-density or non-monotonic input returns `PumpError`, never a panic.

## Sprints

### Sprint 1 — Units and quantity parsing
- **Files:** `src/units.rs`.
- **Behaviour:**
  - `parse_quantity::<Q>("125 m3/h")`;
  - an accepted-units table per quantity: flow (m3/h, m3/s, l/s, gpm(US)), pressure (bar, kPa, MPa, psi; gauge vs absolute flag), head/length (m, ft, mm, in), power (kW, W, hp), speed (rpm, 1/min), density (kg/m3) and ratio (%, fraction);
  - display helpers that round only for display.
- **Tests:** each unit converts both ways (property test: `to → from` is the identity within 1e-12 relative); unknown units and empty strings return errors.
- **Acceptance:** `cargo test -p pump-core && cargo clippy -p pump-core --all-targets -- -D warnings`.
- **Done:** the accepted-units table is documented in the crate README.
- **Assumes:** P0 types exist.

### Sprint 2 — Test-point reduction
- **Files:** `src/reduce.rs`. The `ReducedPoint` fields are filled.
- **Formulas:**
  - velocity `V = Q / (π d²/4)` for suction and discharge;
  - total head `H = (Pd − Ps)/(ρ g) + (Vd² − Vs²)/(2 g) + (Zd − Zs)`, using gauge elevations relative to a datum;
  - hydraulic power `P_hyd = ρ g Q H`;
  - shaft power: `Shaft` gives P directly, `MotorInput` gives `P_shaft = P_in · η_motor`;
  - efficiency `η = P_hyd / P_shaft`.
- **Tests:** hand-calculated synthetic cases (spreadsheet values written into the test); zero/negative flow; `η > 1` gives an error flagged as implausible input.
- **Acceptance:** same commands as Sprint 1.
- **Done:** every formula is documented with symbols and units in doc comments.
- **Assumes:** Sprint 1.

### Sprint 3 — Speed correction (affinity laws)
- **Files:** `src/affinity.rs`.
- **Behaviour:**
  - convert each reduced point to the rated speed n₀: `Q₀ = Q·(n₀/n)`, `H₀ = H·(n₀/n)²`, `P₀ = P·(n₀/n)³`, with η unchanged;
  - record `speed_deviation = (n − n₀)/n₀` per point.
  - Whether a deviation is acceptable is decided by a criterion (`speed_deviation_max`), not here.
- **Tests:** round trip n→n₀→n is the identity; a point at n₀ is unchanged.
- **Acceptance:** same commands.
- **Done:** affinity behaviour is documented, including that it applies only to test points (not to the rated point).
- **Assumes:** Sprint 2.

### Sprint 4 — Curve fitting and readout
- **Files:** `src/fit.rs`.
- **Behaviour:**
  - least-squares polynomial fit (`FitOptions.degree` 2..=4; default 3 for H-Q and P-Q, 2 for η-Q), solved by QR (`nalgebra`) or by normal equations with scaling;
  - outputs coefficients, R² and RMS residual per curve;
  - `RatedReadout`: H, P and η at the rated flow;
  - readout outside the tested flow range returns an error (no extrapolation);
  - shutoff head (Q=0) is read out only if a Q=0 point was tested.
- **Tests:** an exact polynomial is recovered exactly; noisy data gives a bounded residual; fewer points than degree+1 gives an error; rated flow outside the range gives an error.
- **Acceptance:** same commands.
- **Done:** fit diagnostics are exposed for the report.
- **Assumes:** Sprint 3.

### Sprint 5 — Tolerance file and acceptance engine
- **Files:** `src/tolerance.rs` (TOML parsing from `&str`) and `src/accept.rs`. `evaluate()` is implemented end to end.
- **Tolerance file format (generic criterion kinds, no API numbers in code):**
  ```toml
  [standard]
  name = "API 610"
  edition = "<from contract>"          # required
  source = "Table <n>, entered by <who> on <date> from licensed copy"
  example = false                       # true in the public example file → refused unless allow_example

  [[criterion]]
  id = "rated_head"
  kind = "relative_band"                # value at rated flow vs guarantee
  quantity = "head"
  clause = "<clause/table ref>"
  bands = [ { applies_if_rated_head_max = "<x> m", lower_pct = <a>, upper_pct = <b> }, ... ]

  [[criterion]]
  id = "rated_power"
  kind = "max_relative"                 # upper limit only
  quantity = "power"
  clause = "..."
  upper_pct = <c>

  [[criterion]]
  id = "npshr"
  kind = "max_absolute_vs_guarantee"
  quantity = "npshr"
  clause = "..."

  [[criterion]]
  id = "test_speed"
  kind = "speed_deviation_max"
  clause = "..."
  max_pct = <d>

  # optional curve-shape rules, e.g. kind = "head_rising_to_shutoff", kind = "shutoff_band"
  ```
- **Overall outcome:** PASS only if every applicable criterion passes. A criterion that cannot be evaluated (e.g. no NPSHr data) is `NotEvaluated`, which is reported and never counted as a pass.
- **Ship** `examples/tolerances.example.toml` with `example = true` and obviously fake values.
- **Tests:**
  - each criterion kind, at the boundaries (exactly at the limit, just inside, just outside);
  - band selection by rated head;
  - `example = true` is refused;
  - an unknown criterion kind gives an error.
- **Acceptance:** same commands.
- **Done:** `docs/tolerances.md` explains the format and the rule that values come only from the licensed standard.
- **Assumes:** Sprint 4; **U2 is needed only for real files, not for this sprint.**

### Sprint 6 — Golden-test harness and real cases
- **Files:** `tests/golden.rs`.
- **Harness:**
  - discovers `tests/golden/*/` (public, synthetic) and, if `SUITE_PRIVATE_GOLDEN` is set, `costa-inspection-private/golden/*/`;
  - each case has `job.toml`, `test.csv`, `tolerances.toml` and `expected.toml` (head/power/η at rated within stated abs/rel tolerances, verdict per criterion, and overall);
  - minimal internal parsers for the TOML and CSV in tests are allowed; the real parsers belong to P4.
- **Content:** 3 synthetic public cases (one PASS, one head-FAIL, one power-FAIL) plus the U3 private cases.
- **Tests:** the harness itself; a deliberately broken expected value must make the run fail (proving the harness works).
- **Acceptance:** `cargo test -p pump-core --test golden`, and with `SUITE_PRIVATE_GOLDEN` set, every private case passes.
- **Done:** every U3 case is reproduced; any mismatch is investigated and recorded in the journey.
- **Assumes:** Sprint 5 and U2/U3 provided.

### Sprint 7 — Hardening and documentation
- **Robustness:**
  - property tests (proptest) feed random valid jobs and points into `evaluate`: no panic, and either `Ok` or a typed error;
  - fuzz-like inputs: NaN, inf, huge values.
- **Documentation:** `crates/pump-core/README.md` with the method, formulas, assumptions and limitations (e.g. no viscosity correction, water-like fluids only), so a reviewer can audit it.
- **Acceptance:** `cargo test -p pump-core && cargo clippy -p pump-core --all-targets -- -D warnings && cargo deny check`.
- **Done:** the CHANGELOG has a pump-core 0.1 entry, and the journey is updated.
- **Assumes:** Sprints 1–6.
