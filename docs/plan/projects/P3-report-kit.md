# P3 — Report kit (`003_report-kit`)

Repo: `inspection-suite`, crate `crates/report-kit` · Lane: **C** · Depends on: P0 (G0) · Gate: **G2** · Feeds: P4

## Goal

Build a domain-neutral library that turns a `ReportDoc` into DOCX and PDF, draws charts as PNG, and writes a validated evidence package atomically. It knows nothing about pumps.

## Scope

- **In:** DOCX, PDF, line charts with tolerance boxes, the package writer, and a fixed default report style.
- **Out:** templating languages exposed to users (backlog), HTML output, and reading reports back in.

## Sprints

### Sprint 1 — DOCX via `pdfcraft-export`
- **Dependency:** `pdfcraft-export = { git = "https://github.com/storytold/pdfcraft", rev = "<pinned>" }`. Check that it builds outside the PdfCraft workspace; if workspace inheritance blocks it, vendor `crates/export` under `vendor/pdfcraft-export/`, keeping its licence and `NOTICE`.
- **Reuse:**
  - `pdfcraft_export::docx(pages, title)` and the `Page/Block/Table/Cell/Image` types (`pdfcraft.back/crates/export/src/lib.rs`, l.23–82, 812);
  - the mapping pattern in `snapshot_export_report_docx` (`pdfcraft.back/crates/engine/src/export.rs:1272`).
- **Files:** `src/docx.rs`: `to_docx(&ReportDoc)` maps sections, paragraphs, tables, images and verdicts (rendered as a 2-column table row with PASS/FAIL text).
- **Tests:** unzip the bytes and assert that `word/document.xml` contains the title, a table cell and the image relationship; snapshot test of the XML (normalised).
- **Acceptance:** `cargo test -p report-kit && cargo clippy -p report-kit --all-targets -- -D warnings && cargo deny check`.
- **Done:** the DOCX opens in Word/LibreOffice (manual check recorded in the journey).
- **Assumes:** P0 `ReportDoc` types.

### Sprint 2 — Charts (`plotters`)
- **Files:** `src/chart.rs`: `chart_png(&ChartSpec, w, h)`.
- **Behaviour:**
  - series kinds: points (test data), line (fitted curve), marker (guarantee point), and `ToleranceBox` (rectangle around the guarantee from lower/upper limits);
  - optional secondary y axis (η);
  - bitmap backend with an embedded OFL font (`assets/fonts/`, with `ATTRIBUTION`), so no system fonts are needed.
- **Tests:** deterministic output (same spec gives the same bytes, or the same hash); PNG decodes with the right size; empty series gives an error.
- **Acceptance:** same commands as Sprint 1.
- **Done:** an example image is committed under `docs/images/` for review.
- **Assumes:** Sprint 1.

### Sprint 3 — PDF with typst (D-010, accepted)
- **Step 1, validation:**
  - embed `typst` + `typst-pdf` with an in-memory `World` (embedded OFL fonts, no network, no package downloads);
  - measure the release binary size added and the compile time of a 5-page report, and record both in `docs/journey.md`;
  - only a blocking result (e.g. it cannot run offline on Windows) stops the sprint: write `BLOCKED:` in the tracker and propose a new decision entry (fallback `genpdf`/`printpdf` or `.tex`). Never switch silently.
- **Step 2, implementation:**
  - `src/pdf.rs` generates Typst markup from `ReportDoc`, escaping all text (never interpolate raw user strings);
  - images are passed as in-memory files;
  - a fixed A4 style with header (title, job meta), footer (tool name and version, page X/Y) and verdict styling.
- **Tests:** generated PDF text extracted (e.g. via `lopdf`) contains the title and verdict labels; malicious input such as `#`, `]`, `\` or `$` in user text renders literally.
- **Acceptance:** same commands, and a Windows CI job builds it.
- **Done:** measurements are recorded in the journey and D-010 is confirmed; the PDF opens in the suite's PDF Review tool.
- **Assumes:** Sprint 2.

### Sprint 4 — Package writer
- **Files:** `src/package.rs`: `write_package(target, &Manifest, files)`.
- **Behaviour:**
  1. validate the manifest with `evidence_format::validate`;
  2. check that every evidence path in the manifest is present in `files`, and that there are no extra files outside the allowed list;
  3. write into a sibling staging directory, `fsync`, then rename it atomically;
  4. never leave a half-written package (cleanup on error);
  5. refuse to overwrite an existing target unless `overwrite = true`.
- **Reuse:** the staging-and-verify pattern in `write_snapshot_export_package` (`pdfcraft.back/crates/engine/src/export.rs:1452`) and `normalize_package_relative_path` (l.681).
- **Tests:** happy path; missing evidence file; `..` path; target exists; simulated failure mid-write leaves no target.
- **Acceptance:** same commands.
- **Done:** behaviour documented in the crate README.
- **Assumes:** Sprint 3, `evidence-format` from P0.

### Sprint 5 — Default report style and accessibility
- **Style:** one consistent style across DOCX and PDF: fonts, heading levels, table styles, a verdict colour plus **text** (never colour alone), page numbering, and a document properties title.
- **Tests:** snapshot tests updated; contrast of the verdict colours checked by a constant table.
- **Acceptance:** same commands.
- **Done:** the style is described in `docs/report-style.md`.
- **Assumes:** Sprint 4.

### Sprint 6 — Hardening and G2
- **Robustness:**
  - property tests: random `ReportDoc`s (unicode, empty sections, huge tables) give no panic and either `Ok` or a typed error;
  - performance check: a 50-page report with 20 charts builds in under N seconds (N recorded).
- **Acceptance:** `cargo test -p report-kit && cargo clippy -p report-kit --all-targets -- -D warnings && cargo deny check`.
- **Done:** a G2 entry in the journey; the CHANGELOG entry.
- **Assumes:** Sprints 1–5.
