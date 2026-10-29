# P4 — Pump Performance CLI v0.1 (`004_pump-cli`)

Repo: `inspection-suite`, `apps/pump-cli` · Lane: **B** (lane C joins for sprints 4–5) · Depends on: P2, P3 (G2) · Gate: **G3**

## Goal

Ship an inspector-usable command-line tool that runs:

`pump-cli check job.toml test.csv -o out/`

and produces `report.pdf`, `report.docx`, `evidence/*.png`, `data/*.csv` and a validated `manifest.json` (`package_kind = "suite.pump_test"`). The exit code says PASS, FAIL or input error.

## Input formats

**`job.toml`**
```toml
[pump]                 # identification
tag = "P-101A"
service = "Cooling water"
manufacturer = "..."
model = "..."
serial = "..."
stages = 1

[rated]                # datasheet guarantees, unit-tagged strings
flow = "250 m3/h"
head = "85 m"
power = "78 kW"
speed = "2980 rpm"
efficiency = "76 %"    # optional
npshr = "4.2 m"        # optional

[setup]
suction_id = "200 mm"
discharge_id = "150 mm"
z_suction = "0.35 m"
z_discharge = "0.90 m"
density = "998 kg/m3"
power_basis = "shaft"  # or "motor_input" + motor_efficiency = "95.1 %"

[fit]
degree_head = 3
degree_power = 3
degree_efficiency = 2

[acceptance]
tolerances = "../tolerances/api610-<edition>.toml"   # relative to job.toml

[report]
title = "Shop performance test — P-101A"
witness = "..."
date = "2026-11-12"
client = "..."
```

**`test.csv`**: one row per point. The header carries the units, e.g.
`point,flow[m3/h],p_suction[bar],p_discharge[bar],speed[rpm],power[kW]`

## Sprints

### Sprint 1 — Input parsing
- **Files:**
  - `src/input/job.rs`: `toml` + serde, converted into `pump_core::Job` using `pump_core::units`;
  - `src/input/csv.rs`: the `csv` crate, parsing the header units once and the rows into `Vec<TestPoint>`.
- **Errors** must be friendly and carry the file, line, column and field, e.g. `test.csv:7 column "power[kW]": "7O.2" is not a number`.
- **Tests:** valid example files; one test per error class (missing field, bad unit, bad number, empty file, duplicate point id, wrong column count).
- **Acceptance:** `cargo test -p pump-cli && cargo clippy -p pump-cli --all-targets -- -D warnings`.
- **Done:** `examples/job.toml`, `examples/test.csv` and `docs/input-format.md`.
- **Assumes:** P2 Sprint 1 units and the P0 types.

### Sprint 2 — `check` command (no reports yet)
- **Files:** `src/main.rs` (clap): `check <job> <csv> [--tolerances <file>] [--allow-example] [--json]`.
- **Behaviour:**
  - runs `pump_core::evaluate`;
  - prints a terminal summary: the readout at rated, then one line per criterion with limit, measured value, deviation and PASS/FAIL;
  - `--json` prints the `AcceptanceReport`;
  - exit codes: 0 = PASS, 1 = FAIL, 2 = input or usage error, 3 = internal error.
- **Tests:** `assert_cmd` integration tests over the public golden cases from P2.
- **Acceptance:** same commands.
- **Done:** `pump-cli check` is usable without report output.
- **Assumes:** P2 done.

### Sprint 3 — `AcceptanceReport` → `ReportDoc` + `ChartSpec`
- **Files:** `src/report_build.rs`.
- **Report sections:**
  1. Summary: overall verdict, pump id, standard and edition, witness and date.
  2. Guarantees vs measured at rated.
  3. Acceptance criteria table (criterion, clause, limit, measured, deviation, verdict).
  4. Test data (raw and reduced) tables.
  5. Curves: an H-Q chart with a tolerance box; P-Q and η-Q charts.
  6. Method and assumptions (formulas, fit degree, R², density, power basis).
  7. Traceability: tool version and commit, tolerance file `source`, and an input file SHA-256.
- **Tests:** a snapshot of the `ReportDoc` (as JSON) for a golden case.
- **Acceptance:** same commands.
- **Done:** reviewed by the user against U4 (expected layout).
- **Assumes:** P3 types and Sprint 2.

### Sprint 4 — Write the package (lanes B + C)
- **Files:** `src/package_build.rs`.
- **Behaviour:**
  - builds the `Manifest` (`suite.pump_test`, items: `item_1` = the acceptance summary, `item_2..` = charts, data CSVs and the input copies);
  - calls `report_kit::{to_pdf, to_docx, chart_png, write_package}`;
  - `-o <dir>`, `--formats pdf,docx` (default both) and `--overwrite`.
- **Package layout:**
  ```
  out/
    manifest.json
    report.pdf
    report.docx
    evidence/hq.png  evidence/pq.png  evidence/eq.png
    data/test.csv  data/reduced.csv
    inputs/job.toml  inputs/tolerances.toml
  ```
- **Tests:** end-to-end on a golden case: `manifest.json` validates, all files exist, and the PDF text contains the overall verdict.
- **Acceptance:** same commands, plus `cargo test -p pump-cli --test e2e`.
- **Done:** the package opens in the suite's PDF Review tool.
- **Assumes:** Sprint 3, G2.

### Sprint 5 — `validate` command and reproducibility
- **Commands:**
  - `pump-cli validate <package-dir>`: schema check, every file present, and the SHA-256 of the inputs matches what the manifest records;
  - `pump-cli recheck <package-dir>`: re-run the evaluation from `inputs/` and diff the verdicts (catches tool-version drift).
- **Tests:** a tampered package fails `validate`; a recheck of a fresh package reports no differences.
- **Acceptance:** same commands.
- **Done:** `docs/reproducibility.md`.
- **Assumes:** Sprint 4.

### Sprint 6 — Release v0.1.0 (G3)
- **Files:** `.github/workflows/release.yml`. It builds the macOS universal binary (ad-hoc signed) and the Windows x64 portable zip (exe + examples + README + licences, with no admin rights needed), then creates the pre-release.
- **Steps:**
  - run every private golden case locally (`SUITE_PRIVATE_GOLDEN`);
  - smoke test on Windows as a non-admin user;
  - CHANGELOG `0.1.0`.
- **Acceptance:** the release workflow is green; the golden run passes; the smoke checklist is recorded in the journey.
- **Done:** a GitHub pre-release exists, and the journey has a G3 entry. The tool is then cleared for in-house jobs with a second-person review of the first 3 real reports.
- **Assumes:** Sprints 1–5.
