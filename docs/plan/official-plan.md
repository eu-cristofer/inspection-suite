# Inspection Suite — Official Plan

Status: approved for sprint planning · Version 1.3 · Last reviewed: 2026-10-29
Owner: Cristofer Antoni, for Costa Inspection · Tracking: [`../journey.md`](../journey.md) · Decisions: [`../decisions.md`](../decisions.md)

## Revision history

| Version | Date | Change |
|---|---|---|
| 1.0 | 2026-10-29 | First official plan. Defines the suite, 3 repos, 8 projects (P0–P7), 3 lanes and the MVP scope. |
| 1.1 | 2026-10-29 | typst accepted for PDF reports (D-010); organisation Costa Inspection, slug `costa-inspection` (D-011). |
| 1.2 | 2026-10-29 | Repos stay on `eu-cristofer` permanently (D-012, supersedes D-011). |
| 1.3 | 2026-10-29 | New standalone repo `eu-cristofer/pdfcraft` from fresh upstream; old fork archived as `pdfcraft_first`; lane A runs P5 (port) before P1 (release) (D-013). |

---

## 1. Purpose

An inspection company needs software that turns inspection inputs into **traceable, reproducible evidence and reports**. The suite is open source (`MIT OR Apache-2.0`) and used in-house at no cost. Company-specific data and templates stay private.

Tools in scope:

| Tool | What it does | Starting point |
|---|---|---|
| **`<Suite>` PDF Review** | Review vendor/client PDFs, crop evidence, OCR, export an evidence package (manifest + DOCX/LaTeX + PNG crops) | Works today in the PdfCraft fork `eu-cristofer/pdfcraft`, branch `feat/crop-panel-unification` (17649c1b) |
| **`<Suite>` Pump Performance** | API 610 shop-performance-test acceptance: reduce test points, correct to rated speed, fit curves, check tolerances, report PASS/FAIL | New |

`<Suite>` is a placeholder; the name must be chosen before the first public release (gate G1).

## 2. Guiding principles

1. **Ship what works first.** The PDF tool goes out as v0.1 before any refactor or upstream sync.
2. **Rule of two.** Shared code is extracted only when a second tool consumes it. Until then, copying a small piece is cheaper than an early abstraction.
3. **The contract comes before parallel work.** Lanes run in parallel only against frozen interface types (P0).
4. **The calculations must be trustworthy.**
   - Pump calculations are pure (no I/O, no UI), use typed units, and are proven by golden tests from signed-off past jobs.
   - Acceptance limits are **data from the user's licensed copy of API 610**, never hard-coded from memory, and every report records the edition used.
5. **Thin fork.** The PDF fork changes as few upstream-owned files as possible, because upstream moves at about 100+ commits a day.
6. **No panics, untrusted input.** These are the PdfCraft conventions, carried over to every suite crate.
7. **Provenance.**
   - The text of API, ISO or HI standards is never committed.
   - Client data and real jobs never go into public repos.
   - ArtCraft trademarks never go into suite builds.

## 3. Repositories

| Repo | Contents | Visibility | Created by |
|---|---|---|---|
| `eu-cristofer/pdfcraft` (standalone, from upstream `3109bb23`; D-013) | PDF Review tool. Syncs from `storytold/pdfcraft` via an `upstream` remote. `eu-cristofer/pdfcraft_first` = read-only archive of the old fork. | public | exists |
| `eu-cristofer/inspection-suite` | Cargo workspace: `evidence-format`, `pump-core`, `report-kit`, `apps/pump-cli`; this plan | public | P0 |
| `eu-cristofer/costa-inspection-private` | real `tolerances.toml` per API 610 edition, golden cases from past jobs, client report templates | private | P0 |

## 4. Architecture

```
                 ┌──────────────── inspection-suite (Cargo workspace) ────────────────┐
 job.toml ──┐    │                                                                     │
 test.csv ──┼──► │ apps/pump-cli ──► pump-core ──► AcceptanceReport ──► report-kit ──┐ │
 tolerances ┘    │      (I/O, CLI)    (pure calc,                       (DOCX, PDF,  │ │
                 │                     uom units)                        charts)     │ │
                 │                                   evidence-format ◄───────────────┘ │
                 │                                   (manifest types, schema, validate)│
                 └──────────────────────────────────────▲──────────────────────────────┘
                                                        │ adopts later (P5)
 <suite>-pdf (fork of PdfCraft): crop review ──► snapshot export package (pdfcraft.snapshot_export v1.1)
```

Crate responsibilities and allowed dependencies (each arrow is a "may depend on"):

| Crate | Responsibility | May depend on | Must not |
|---|---|---|---|
| `evidence-format` | Package manifest envelope + per-kind payloads, JSON Schema, `validate()` | serde, serde_json, schemars | do I/O beyond optional path checks; depend on pump or report |
| `pump-core` | API 610 performance-test calculations and acceptance verdicts | uom, serde, toml (tolerance parsing from `&str`), nalgebra or own LSQ | do file I/O, UI, rendering |
| `report-kit` | `ReportDoc` → DOCX / PDF; charts → PNG; package writer | `pdfcraft-export` (git, pinned), typst (+typst-pdf), plotters, evidence-format | know about pumps |
| `apps/pump-cli` | Parse inputs, orchestrate, write package, exit codes | all of the above, clap, csv, toml | contain calculations |

## 5. Projects

| ID | Sprint project slug (repo) | Lane | Depends on | Size | Spec |
|---|---|---|---|---|---|
| P0 | `001_suite-bootstrap` (inspection-suite) | user + Claude, sequential | — | ~1 day, 3 sprints | [P0](projects/P0-suite-bootstrap.md) |
| P1 | pdf-release-v0-1 (new `pdfcraft`, re-registered after P5) | **A** | P5 | 2–3 days, 5 sprints | [P1](projects/P1-pdf-release-v0.1.md) |
| P2 | `002_pump-core` (inspection-suite) | **B** | P0 | ~1.5 weeks, 7 sprints | [P2](projects/P2-pump-core.md) |
| P3 | `003_report-kit` (inspection-suite) | **C** | P0 | ~1 week, 6 sprints | [P3](projects/P3-report-kit.md) |
| P4 | `004_pump-cli` (inspection-suite) | **B**, with C joining | P2, P3 | ~1 week, 6 sprints | [P4](projects/P4-pump-cli.md) |
| P5 | crop-review port onto fresh upstream + `evidence-format` adoption (new `pdfcraft`) | **A** (first) | P0 | ~1–2 weeks | [backlog](projects/P5-P7-backlog.md) |
| P6 | pump GUI (egui) (inspection-suite) | B | P4 | ~1–2 weeks | [backlog](projects/P5-P7-backlog.md) |
| P7 | vendor-curve digitising: PDF crop → pump tool (both) | C | P4, P5 | ~1–2 weeks | [backlog](projects/P5-P7-backlog.md) |

Slugs follow `/sprint-loop` global numbering per repo. The fork already used 001–005, so its next number is 006. Each spec file is written to be registered directly: `/sprint-loop register <slug>` with the spec as the approved plan.

**Why this split.** There are 3 repos because the fork must track upstream, the suite is fully owned, and private data must stay private. There are 3 lanes because there is one orchestrator plus AI loops. More lanes would cost more in review and merge time than they save.

## 6. Schedule (sequential and parallel)

```
           Day 0        Week 1            Week 2            Week 3            Week 4+
Lane A  │ P5 port onto fresh upstream ──────────────┐ P1 rebrand + release ── v0.1 (G1) ──────►
User    │ P0 bootstrap ─┐ (G0)        │
Lane B  │               └─ P2 pump-core ──────────────┐ P4 pump-cli ── v0.1 (G3) │ P6 GUI ►
Lane C  │               └─ P3 report-kit ───── (G2) ──┘ (joins P4 s4–s5)         │ P7 ──►
Inputs  │ U1 name · U2 API 610 edition+limits · U3 golden jobs (needed by P2 s6)
```

Critical path: **P0 → P2 → P4**. P3 has slack. Lane A (P5 → P1) runs in parallel with all of it (D-013).

## 7. Gates and milestones

| Gate | Condition | Unblocks |
|---|---|---|
| G0 — Contract frozen | P0 merged: workspace compiles, `evidence-format` + interface types merged, CI green on Linux/macOS/Windows | P2, P3 |
| G1 — PDF v0.1 public | P1 done, suite name chosen (U1), no ArtCraft marks (grep is clean), portable Windows zip with OCR models, smoke test passed | public announcement, P5 |
| G2 — Report kit ready | DOCX + PDF + chart + package writer pass their snapshot tests; typst PDF output validated (size, offline fonts) | P4 sprint 4 |
| G3 — Pump v0.1 | `pump-cli check` reproduces every golden case; package validates; Windows non-admin run works | in-house use on jobs, P6, P7 |

## 8. User inputs (not delegable to AI loops)

| ID | Input | Needed by | Where it goes |
|---|---|---|---|
| U1 | Suite and tool names, icon | P1 sprint 1 (placeholder ok), **required at G1** | fork `branding` module |
| U2 | API 610 edition(s) used in contracts + tolerance values from the licensed copy | P2 sprint 5 | `costa-inspection-private/tolerances/api610-<edition>.toml` |
| U3 | 2–3 anonymised past shop tests with signed-off results (inputs and final numbers) | P2 sprint 6 | `costa-inspection-private/golden/<case>/` |
| U4 | Sample of the current report layout the client expects | P3 sprint 1 | `costa-inspection-private/templates/` |
| U5 | GitHub home ✅ resolved: personal account `eu-cristofer` | P0 sprint 1 | repo URLs (D-012) |

## 9. Quality gates (every PR, every repo)

- `cargo fmt --check`.
- `cargo clippy -p <crate> --all-targets -- -D warnings`.
- `cargo test -p <crate>`, scoped to the touched crates.
- `cargo deny check` (licences restricted to the permissive allow-list copied from PdfCraft).
- No `unwrap/expect/panic!` in non-test code unless provably infallible and commented.
- Every bug fix and every acceptance criterion gets a test that has been **seen failing** (revert the fix and confirm).
- CI matrix: Linux, macOS, Windows.

## 10. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Tolerance values entered wrongly | Wrong PASS/FAIL on a client job | Values in data, with edition and table recorded in every report. Golden tests from signed-off jobs. Example file refuses to run without `--allow-example`. |
| typst embedding too heavy or font issues offline | No PDF report | validation step at the start of P3 s3; if blocking, a new decision entry (fallback `genpdf`/`printpdf` or `.tex`) |
| `pdfcraft-export` git dependency breaks on upstream changes | DOCX build fails | Pin to a rev; bump deliberately; vendoring is the fallback (permissive licence) |
| Upstream drift makes P5 expensive | Fork stagnates | P5 scheduled right after G1; nightly "build on upstream main" CI in the fork |
| Brand/trademark breach | Legal exposure | G1 blocks release on a clean grep; P1 s1/s2 |
| Single orchestrator overloaded | Lanes stall | At most 3 lanes; P3 has slack; P1 is short |

## 11. Definition of done for the MVP

- `<Suite>` PDF Review v0.1.0 is published (macOS ad-hoc signed and Windows portable), with no ArtCraft marks.
- `<Suite>` Pump Performance v0.1.0 is published:
  - `pump-cli check job.toml test.csv -o out/` gives a PDF + DOCX + validated evidence package, with exit code 0/1/2;
  - every golden case is reproduced.
- The plan, journey and decision log are up to date, and each project's sprint folder is archived.
