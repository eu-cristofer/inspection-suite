# P1 — PDF Review v0.1 release (`006_pdf-release-v0-1`)

Repo: new standalone `eu-cristofer/pdfcraft` (D-013), on top of the P5 port · Lane: **A** · Depends on: **P5** · Gate: **G1**

> **Re-baselined 2026-10-29 (D-013).** This spec was first registered in the old fork (now `eu-cristofer/pdfcraft_first`, branch `feat/006-pdf-release-v0-1`); that registration is superseded. File paths and line numbers below come from the old base. Re-verify them on the new repo when registering after P5 (upstream split several `ui-egui` files into modules). Release tags restart with the new repo's own scheme, set at registration.

## Goal

Publish the PDF Review tool exactly as it works today, under the suite's own name and with no ArtCraft trademarks, as an unsigned v0.1.0 for macOS and a portable Windows zip that includes the OCR models.

## Scope

- **In:** branding, packaging metadata, release workflow, fork documentation layout, release.
- **Out:**
  - upstream sync and rebase (that's P5);
  - new features;
  - signing and notarisation;
  - Linux, FreeBSD and web builds.

## Background facts (verified 2026-10-28)

- The ArtCraft marks are used in `crates/ui-egui/src/widgets.rs:559-575`, through `include_bytes!` of `docs/brand/artcraft-logo*.svg` and `artcraft-mark.svg`. `docs/brand/LICENSE-brand.txt` forbids them in forks and modified versions.
- App icon: `apps/pdfcraft/src/main.rs:54-56` (`assets/app-icon/pdfcraft-1024.png`, `hicolor/256x256/apps/ai.storyteller.pdfcraft.png`).
- `getartcraft.com` URLs appear in:
  - `packaging/macos/Credits.rtf`;
  - `packaging/linux/nfpm.yaml`;
  - `packaging/linux/ai.storyteller.pdfcraft.metainfo.xml.in`;
  - `packaging/windows/pdfcraft.wxs:41-42`.
- `.github/workflows/unsigned-release.yml` zips only `pdfcraft.exe` and `pdfcraft-cli.exe`, so OCR models are missing. `packaging/windows/package.ps1` already produces `pdfcraft-<ver>-windows-<arch>-portable.zip` with `models\` + `portable.txt`.
- The fork's spec for a no-admin portable Windows build already exists in the fork's docs (2026-10-28).

## Sprints

### Sprint 1 — Branding module and removal of the in-app marks
- **Files:**
  - new `crates/ui-egui/src/branding.rs` with constants `APP_NAME`, `SUITE_NAME`, `APP_ID` (reverse DNS, e.g. `com.costainspection.<suite>-pdf`), `HOMEPAGE_URL`, and `BASED_ON = "Based on PdfCraft (MIT OR Apache-2.0)"`;
  - `widgets.rs`: replace the `artcraft_*` functions with a `suite_mark` that uses a contributor-original SVG in `assets/brand-suite/`;
  - the call sites of the old functions;
  - window title and About text.
- **Placeholders:** values may stay `<Suite>` placeholders, but the release (Sprint 5) is gated on U1.
- **Reuse:** the existing `egui::Image::from_bytes` pattern in `widgets.rs`.
- **Tests:**
  - a unit test asserting that no `include_bytes!` path contains `docs/brand`;
  - existing panel snapshot tests updated (`crates/ui-egui/tests/it/panel_snapshots.rs`).
- **Acceptance:** `cargo test -p pdfcraft-ui-egui -j 2`, `cargo clippy -p pdfcraft-ui-egui -j 2 --all-targets -- -D warnings`, `cargo xtask assets`.
- **Done:** `grep -rn "artcraft" crates apps` returns no code references.
- **Assumes:** fork at 17649c1b, clean tree.

### Sprint 2 — App icon, ids and packaging metadata
- **App icon:** replace the icon assets and update `main.rs:54-56`. New assets need `ATTRIBUTION.toml` entries (contributor-original, `MIT OR Apache-2.0`).
- **Packaging metadata:** update the URLs and names in `pdfcraft.wxs`, `nfpm.yaml`, `metainfo.xml.in` and `Credits.rtf`.
- **Remove the brand directory:** delete `docs/brand/` and its `ATTRIBUTION.toml` entries, and fix `NOTICE`.
- **Note:** a new app id means a new settings directory. That's acceptable for a fork; document it in `CHANGELOG.md`.
- **Tests:** `cargo xtask assets`; `packaging-lint` workflow logic run locally where possible.
- **Acceptance:** `cargo xtask assets && cargo test -p pdfcraft -j 2`, and `grep -rni "artcraft\|getartcraft" crates apps packaging docs --exclude-dir=archive` returns nothing outside the "based on" notice.
- **Done:** a clean grep, and `NOTICE` states the fork origin.
- **Assumes:** Sprint 1.

### Sprint 3 — Portable Windows zip with OCR models
- **Files:** `.github/workflows/unsigned-release.yml`. The Windows job calls `packaging/windows/package.ps1` with the portable output, signing skipped, and uploads `*-portable.zip`. Artifact names use `APP_NAME`.
- **Reuse:** `package.ps1`; the fork's no-admin portable spec.
- **Tests:** `workflow_dispatch` dry run on a `-rc1` tag. Download the zip, then check that it contains `models\`, `portable.txt` and both exes.
- **Acceptance:** the workflow run is green; a listing of the zip contents is attached to the journey entry.
- **Done:** the release notes template mentions portable mode and no admin.
- **Assumes:** Sprints 1–2.

### Sprint 4 — Fork documentation layout
- **Files:**
  - move fork-only docs into `docs/fork/`: `crop-review-workspace.md`, `snapshot-export-spec.md`, `snapshot-export-discovery.md`, `selected-rectangle-export.md`, `rectangle-driven-export.md`, `fork-sync-sop.md`, `SOP/`, `techind/`, `to-do-later.md`, and the Windows portable spec;
  - restore upstream's `README.md`, `CLAUDE.md` and `ROADMAP.md` to their upstream text, adding one "Fork notes → docs/fork/README.md" line;
  - archive `docs/changes/002_selected-rec`, `003_export-flow` and `005_export-contract-hardening`, following the `/sprint-loop archive` rules (statuses: 002 and 003 are "active", so first check that they are complete or close them as superseded).
- **Tests:** link check (`grep` for broken relative links to moved files).
- **Acceptance:** `cargo test -p pdfcraft-engine -j 2` (docs referenced by tests, if any), plus the link check.
- **Done:** the diff of `README.md`/`CLAUDE.md`/`ROADMAP.md` against `83ba2868` is one line each.
- **Assumes:** Sprints 1–3.

### Sprint 5 — Release v0.1.0
- **Gate:** U1 is set (no `<Suite>` placeholder remains).
- **Steps:**
  - fill the `branding.rs` values and the CHANGELOG `0.1.0` section;
  - optionally rename the repo to `eu-cristofer/<suite>-pdf` (D-012; GitHub redirects the old URL);
  - merge into the fork's `main` via PR;
  - tag `v0.1.0`;
  - run the unsigned-release workflow.
- **Smoke test checklist (Windows, non-admin account, and macOS):**
  - launch;
  - open a sample PDF;
  - create 3 crops;
  - edit their metadata;
  - save the project folder, then reopen it;
  - export a package with DOCX and OCR;
  - check that `manifest.json` lists the 3 items in order.
- **Acceptance:** the checklist is completed and recorded in `docs/journey.md`.
- **Done:** a GitHub pre-release exists, and the journey has a G1 entry.
- **Assumes:** Sprints 1–4.
