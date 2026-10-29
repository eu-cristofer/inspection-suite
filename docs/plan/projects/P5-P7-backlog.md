# P5–P7 — Post-MVP projects (outline)

These are outlines, not sprint-ready specs. Each one is expanded into a full spec, in the format of P0–P4, at the gate that unblocks it.

## P5 — PDF Review: port onto fresh upstream and adopt `evidence-format`

Repo: new standalone `eu-cristofer/pdfcraft`, started from upstream `3109bb23` (D-013) · Lane: A, **runs first, before P1** · Unblocked by: G0 · Source of the work: `eu-cristofer/pdfcraft_first`, branch `feat/crop-panel-unification` @ 17649c1b (local checkout `~/GitHub/pdfcraft.back`)

**Next action:** expand into a sprint-ready spec (it is now on lane A's critical path), then register it in the new repo.

- **Goal:** bring the crop-review / snapshot-export work onto current upstream, in a layout that stays cheap to sync from then on.
- **Steps:**
  1. Tag `backup/crop-review-v0.1` in `pdfcraft_first`. The repo itself stays as the read-only archive.
  2. Work on a feature branch of the new repo, whose `main` = upstream `3109bb23`.
  3. Re-add the crop review / snapshot export as **new files**, with minimal hooks into the upstream layout. Known conflicts appeared in 16 files in the 2026-10-28 dry run (`engine/src/export.rs`, `engine/src/lib.rs`, `ui-egui/src/{lib,canvas,control,panels,editing,commands,chrome}.rs`, and others). Upstream has since split several of these files into modules.
  4. Bring the fork docs (specs, SOPs, `techind/`) and the 001–005 sprint archives across under `docs/fork/`. Upstream-owned docs stay untouched.
  5. Move the ~2,800 fork lines of `engine/src/export.rs` into a fork-owned crate (e.g. `crates/evidence-export`) that depends on suite `evidence-format` (as a git dependency) for manifest types. Keep the v1.1 snapshot manifest byte-compatible.
  6. Add a nightly CI job that builds the fork on the latest `upstream/main`.
  7. Set a weekly sync routine (`git fetch upstream` + merge or rebase; document it in `docs/fork/fork-sync-sop.md`).
- **Done:** the fork diff touches at most about 10 upstream files with small hooks, and the nightly upstream build is green.
- **Open question:** open an upstream issue about an "area/crop export" extension point. If upstream accepts it, the fork gets thinner.

## P6 — Pump Performance GUI (egui)

Repo: inspection-suite (`apps/pump-gui`) · Lane: B · Unblocked by: G3

- **Interface:**
  - forms for the job (pump, rated, setup) and an editable test-point grid, with CSV import and paste from Excel;
  - live charts and verdicts as data is entered;
  - a "Generate package" button.
- **Code:** reuses `pump-core` and `report-kit` unchanged, so the GUI holds no calculations.
- **Look:** follows the PDF tool's look (egui) for suite consistency.
- **Packaging:** portable Windows build.

## P7 — Vendor-curve digitising (PDF crop → pump tool)

Repos: both · Lane: C · Unblocked by: G3 + P5

- **PDF tool:** a new crop kind `curve_figure`. The user calibrates the axes (two clicks per axis, with values and units) and picks points along the curve, which are exported as `kind = "digitized_curve"` items (points in axis units, plus the crop PNG as evidence).
- **Pump tool:** `pump-cli check --vendor-curve <package>` overlays the vendor's guaranteed curve on the test curve, reports the deviation along the curve, and records the source PDF page and crop as traceable evidence.
- **Prerequisite:** extend `evidence-format` with the `digitized_curve` kind (a decision entry is required, because this is a contract change).

## Further backlog (not scheduled)

- Mechanical run test, vibration and bearing temperature records (API 610 other test types).
- NPSH test reduction (3% head-drop determination).
- Viscosity correction (non-water fluids).
- Client templates loaded from `costa-inspection-private/templates/`.
- Code signing (macOS notarisation, Windows Authenticode).
- Localisation (pt-BR UI and reports).
