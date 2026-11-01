# Dev journey

Status: living document · Last reviewed: 2026-10-29
Official plan: [`plan/official-plan.md`](plan/official-plan.md) · Decisions: [`decisions.md`](decisions.md)

This is the running log of the project: where we are, what happened, what's next. Update it at the end of every working session and at every gate. Newest entries go at the top of the log.

---

## Status board

| ID | Project | Repo | Lane | Status | Progress | Next action |
|---|---|---|---|---|---|---|
| P0 | `001_suite-bootstrap` | inspection-suite | user | ✅ done (ready to archive) | 3/3 | Merge PR into `main`; archive with `/sprint-loop archive` |
| P1 | `006_pdf-release-v0-1` | fork `eu-cristofer/pdfcraft` | A | ✅ done (ready to archive) | 6/6 | Owner merges fork PR #10; archive the project; P5 starts after G1 |
| P2 | `002_pump-core` | inspection-suite | B | ⬜ ready to register | 0/7 | Register after the P0 PR merges; run in its own worktree |
| P3 | `003_report-kit` | inspection-suite | C | ⬜ ready to register | 0/6 | Register after the P0 PR merges; run in its own worktree |
| P4 | `004_pump-cli` | inspection-suite | B | ⏸ blocked by P2, P3 | 0/6 | — |
| P5–P7 | backlog | — | — | 💤 later | — | P5 after G1 (in the fork); P6–P7 at G3 |

Legend: ⬜ not started · 🟦 in progress · ✅ done · ⏸ blocked · 💤 backlog

### Gates

| Gate | Target | Status |
|---|---|---|
| G0 — Contract frozen | Day 1 | ✅ 2026-10-29 |
| G1 — PDF Review v0.1 public | Week 1 | ✅ 2026-11-01 |
| G2 — Report kit ready | Week 2 | ⬜ |
| G3 — Pump Performance v0.1 | Week 3–4 | ⬜ |

### User inputs

| ID | Input | Needed by | Status |
|---|---|---|---|
| U1 | Suite and tool names, icon | G1 (P1 s5) | ✅ **WitnessPoint**: WitnessPoint PDF Review, WitnessPoint Pump Performance; placeholder icon kept (D-018) |
| U2 | API 610 edition(s) + tolerance values from licensed copy | P2 s5 | ⬜ open |
| U3 | 2–3 anonymised past shop tests with signed-off results | P2 s6 | ⬜ open |
| U4 | Sample of the expected report layout | P3 s1 / P4 s3 | ⬜ open |
| U5 | GitHub org name for the suite repos | P0 s1 | ✅ personal account `eu-cristofer` (D-012; supersedes D-011) |

---

## How to run the next step (sprint planning)

For each project, in its own repo and terminal:

```bash
# 1. open the repo, on a feature branch
# 2. in Claude Code: plan mode, paste or point to the project spec, approve it, then
/sprint-loop register <slug>          # e.g. 001_suite-bootstrap, using docs/plan/projects/P0-suite-bootstrap.md
# 3. run it
/loop /sprint-loop next <slug>        # one sprint per run, or /sprint-loop run <slug>
```

Order: register **P0** and **P1** now. Register **P2** and **P3** right after G0 and run them in parallel (separate worktrees). Register **P4** when P2 sprint 5 is done.

---

## Log

### 2026-11-01 — G1 reached: WitnessPoint PDF Review v0.1 is public
- Lane/project: A · `006_pdf-release-v0-1` sprint 5 done
- The owner ran the Windows smoke (non-admin account, portable zip): passed in full.
- With the macOS smoke of 2026-10-31, the whole checklist passed on the published pre-release `0.14.0-unsigned.20261031`.
- Fork PR https://github.com/eu-cristofer/pdfcraft/pull/10: CHANGELOG results, sprint 5 ticked, `Status: ready to archive`.
- **Next:** the owner merges #10; archive `006_pdf-release-v0-1`; P5 (upstream sync) can start.

### 2026-10-31 — v0.1 pre-release published; macOS smoke passed
- Lane/project: A · `006_pdf-release-v0-1` sprint 5
- PR #9 merged by the owner (`5e65848d`). Release run 41237856261 (macOS, Windows, release: all green) published https://github.com/eu-cristofer/pdfcraft/releases/tag/0.14.0-unsigned.20261031 (pre-release): macOS universal dmg, macOS CLI zip, Windows x64 portable zip. CI ran `ocr-smoke.sh` on all three packages.
- **macOS smoke on the published files** (downloaded; SHA-256 matched the release digests):
  - `ocr-smoke.sh` passed on the CLI zip and on the `.app` from the dmg;
  - codesign `--strict --deep` OK; bundle id `com.costainspection.witnesspoint-pdf`; x86_64 + arm64; process name "WitnessPoint PDF Review"; no update prompt;
  - in the app (synthetic 3-line scan, no client data): drew 3 crops, set caption and note on each, saved, closed, reopened: crops, captions and notes kept;
  - Export Crop Review Package with `report.docx`: item_1..item_3 in order, 3 PNGs, `ocr_text` exactly each crop's line;
  - About: "WitnessPoint PDF Review", "Based on PdfCraft by the ArtCraft Team…", the fork's help and GitHub links.
- Deviation from the checklist: the UI has no "save project folder" command (only the tests call it), so "save the project folder, close, reopen" was done with File ▸ Save (the crops are embedded in the PDF).
- Upstream strings still visible (on the "left on purpose" list in the CHANGELOG): the Help menu says "About PdfCraft"; the export dialog says "…PdfCraft will create inside it"; the macOS executable is named `PdfCraft`.
- **Next:** the owner runs the Windows non-admin smoke from the portable zip; then tick sprint 5, mark G1, record the results in the fork CHANGELOG.

### 2026-10-31 — Local macOS build finds the upstream update prompt (PR #9)
- Lane/project: A · `006_pdf-release-v0-1` sprint 5
- PR #8 merged (`5dcb99f4`). Release run 41132856075: **Windows passed**, including the new 3-crop OCR package check; **macOS never got a runner** (GitHub arm64 capacity), so nothing was published.
- Owner's idea: build macOS locally. `package.sh --arch universal` on `main` (6 min): dmg and CLI zip are universal (x86_64 + arm64), codesign `--strict --deep` OK, bundle id `com.costainspection.witnesspoint-pdf`, LaunchServices name "WitnessPoint PDF Review"; `ocr-smoke.sh` passed on the CLI zip and on the `.app`.
- **Blocker found by launching the app:** "PdfCraft v0.14.0 to 0.15.0 — UPDATE AND RESTART". The updater reads upstream's feed and would install upstream's app over ours. Not clicked.
- Fix in PR https://github.com/eu-cristofer/pdfcraft/pull/9: no check at start; Help ▸ Check for updates opens this fork's releases page; 2 guard tests (both fail without the fix); parity evidence updated. Re-verified on a new local build: no prompt; About shows "WitnessPoint PDF Review", "Based on PdfCraft…", the fork's links. CI green.
- **Next:** owner merges #9 and dispatches a fresh release (not a re-run of 41132856075, which would ship the prompt).

### 2026-10-30 — Release held back: crop OCR read the wrong area (PR #8)
- Lane/project: A · `006_pdf-release-v0-1` sprint 5
- PR #7 merged by the owner (`0882c74d`). The owner dispatched the release (run 40992395586).
- While preparing the smoke test, found that **every crop's OCR text came from the area at twice its coordinates**: the OCR tile was scaled twice (fork commit `40fa1c31`, 2026-10-26). The evidence image was right, its text wasn't. Hidden while OCR never worked (D-017). It affects the Crops panel, the Export dialog and the package tools.
- The release run was **cancelled before publishing** (nothing public).
- Fix in PR https://github.com/eu-cristofer/pdfcraft/pull/8 (`ff02ff1b`): the tile is scaled once; a new engine test (fails without the fix); a false-pass automation test corrected; the release OCR smoke script now exports a 3-crop package and checks each crop's text (fails on the unfixed build).
- Local smoke (dev build, models beside the CLI): a scanned page with 3 lines gives item_1..item_3 in order, 3 PNGs, `report.docx`, and each crop's `ocr_text` is exactly its line.
- **Next:** PR #8 CI green, then the owner merges and dispatches the release again (same tag `0.14.0-unsigned.20261029`, `publish=true`); macOS smoke on the published dmg; Windows smoke by the owner.

### 2026-10-29 — P1 sprint 5: final names, PR #7 green, waiting for the owner's merge
- Lane/project: A · `006_pdf-release-v0-1` sprint 5
- U1 decided (owner delegated): **WitnessPoint**, D-018.
- Done (fork): `84be5e3b` final names: app id / bundle id `com.costainspection.witnesspoint-pdf`, `WitnessPoint PDF Review.app`, Windows version info, own MSI UpgradeCode, release title; `44b99d9f` the MSI's context-menu release detection uses the new code too (caught by CI). Two export tests that only passed while OCR couldn't run now pin the right behaviour (the blank-page test fails with the D-017 models). Drafted by the Sonnet drafter; README, CHANGELOG, window title, notes quoting and tests fixed by the orchestrator.
- Verified: ui-egui 580 + 1192, xtask 55, pdfcraft app, clippy, fmt, assets. PR https://github.com/eu-cristofer/pdfcraft/pull/7: all 9 checks green (incl. FreeBSD, gate, Windows ARM64 install and run).
- **Waiting on the owner:** merge PR #7 (the agent may not merge without review). Then dispatch `unsigned-release.yml` on `main` with tag `0.14.0-unsigned.20261029` and `publish=true`, run the smoke checklist, tick sprint 5 and mark G1.

### 2026-10-29 — Release workflow fixed; OCR repaired in every package (D-017)
- Lane/project: A · `006_pdf-release-v0-1` (fixes before sprint 5)
- Done (fork, branch `feat/006-pdf-release-v0-1`):
  - `unsigned-release.yml`: macOS job on `macos-15` (run 40854286030 never got a `macos-14` runner, so nothing was published); new `publish` input, off by default, so feature-branch test runs build only;
  - OCR: every build returned junk (`1`, `T`) because `17649c1b` had replaced the models with mismatched checkpoints during the `.rten` URL brownout. Backported upstream `da8cea2b`'s ONNX models and lookup; the macOS CLI zip now ships the models;
  - `packaging/ocr-smoke.sh`: the packaged CLI (Windows portable zip, macOS CLI zip, the `.app`'s `Resources/models`) turns text into a picture, OCRs it and checks the words; the release can't publish unless it passes.
- Verified: run 40860103527 green on `macos-15` + Windows (build only). Locally: engine real-model OCR tests 3/3 (were 0/3), `pdfcraft-ocr` 14/14, automation 280/280 with and without models, fmt, clippy, assets, layers; smoke script passes with models and fails without them.
- CI run 40862374532: green; OCR smoke test passed on the Windows portable zip, the macOS CLI zip and the macOS `.app` (build only, nothing published).
- Next: sprint 5, which still needs U1.

### 2026-10-29 — P1 sprint 4 done; P1 waits for U1
- `c4ec8e6b`: fork docs moved to `docs/fork/` with an index; `AGENTS.md`, `CLAUDE.md` and `ROADMAP.md` byte-identical to upstream again (upstream's binding `AGENTS.md` had been overwritten on 2026-10-25 by a generated guide, now `docs/fork/agent-notes.md`); the fork's own short README (D-016, `README.md merge=ours`); the screenshot showing the ArtCraft mark deleted; 002 (superseded), 003 and 005 archived.
- **Blocked on U1:** sprint 5 needs the suite and app name (and optionally an icon). It also gives the MSI its own `UpgradeCode` and replaces the `ai.storyteller.pdfcraft` id and the "PdfCraft" product name in packaging.

### 2026-10-29 — P1 sprints 2 and 3 done
- Sprint 2 (`d92caf7b`):
  - ArtCraft Discord, website and app page links replaced by Help (the fork's issue tracker) and "This app on GitHub", keeping "PdfCraft on GitHub" as attribution;
  - placeholder app icon in every format;
  - `docs/brand/` deleted;
  - `NOTICE` says "modified version by Costa Inspection, not made or endorsed by ArtCraft";
  - installer publisher is Costa Inspection.
- Found for sprint 5: the MSI `UpgradeCode` is upstream's (our MSI would replace an installed upstream PdfCraft); the `ai.storyteller.pdfcraft` id and the "PdfCraft" product name are still in about 15 files.
- Sprint 3 (`edaeb243`, `34fb8fae`): unsigned-release builds the Windows **portable zip** with `package.ps1`. Run 40854286030: the zip has both exes, `portable.txt`, `models/` (2 OCR models and their licences), licences, `NOTICE` and font notices. The macOS job never started (no hosted macOS runner available), so nothing was published.
- The owner enabled Actions on the fork (settings, plus the fork workflow confirmation on the Actions page).
- **Next:** sprint 4.

### 2026-10-29 — P1 sprint 1b done: suite green
- `448d5da7`: real bug fixed in crop review (a quick corner-handle drag missed the handle, because the drag start was hit-tested where the pointer was when egui reported the drag); the OCR dropdown now has an accessible name; outdated tests updated (label helper, panel snapshot list, fill & sign cursor); new deterministic Crops panel snapshot.
- ui-egui: **1775 passed, 0 failed** with `CRAFT_FONTS_DIR` (absolute path to a craft-fonts checkout, cloned at `~/GitHub/craft-fonts`); without it, only the 10 CJK/Thai font tests fail.
- **Next:** sprint 2.

### 2026-10-29 — P1 sprint 1 done; sprint 1b added (D-015)
- Sprint 1 (`494c4b95`): `branding` module, contributor-original suite mark replacing the ArtCraft logo, "Based on PdfCraft" in About, window title from `APP_NAME`.
- Baseline commits `0a8d8cd6` and `9aaede9b` fix trivial problems in the earlier fork work: fmt, clippy on rustc 1.99, a stale OCR test, `crops_panel` missing from PAGE_MODULES.
- ui-egui: 1755 passed, 20 failed, all of them pre-existing: crop accessible name, stale snapshot test, fill_sign cursor, 10 CJK/Thai glyph tests without `craft-fonts`. The owner approved a repair sprint 1b (D-015).
- The ArtCraft community links (Discord, website) were found and added to sprint 2 (`ae3c9d67`).
- **Next:** sprint 1b, then sprint 2.

### 2026-10-29 — Design restored (D-014 supersedes D-013)
- Owner instruction: "follow strictly our design".
- The standalone `eu-cristofer/pdfcraft` created earlier today was deleted (it held only an upstream copy), and the fork was renamed back from `pdfcraft_first` to **`eu-cristofer/pdfcraft`** (a fork of `storytold/pdfcraft`).
- `~/GitHub/pdfcraft.back` points at the fork again; `~/GitHub/pdfcraft` is a plain upstream clone again.
- Lane A: **P1 now** (registered as `006_pdf-release-v0-1`), **P5 after G1**. Plan v1.4 = v1.2 text.
- **Next:** run P1 in the fork; merge PR #1 and start P2 + P3.

### 2026-10-29 — Fresh start for the PDF tool (D-013)
- The old fork was renamed `eu-cristofer/pdfcraft_first` and is kept as the read-only archive of the 001–006 work (local `~/GitHub/pdfcraft.back`).
- A new **standalone** `eu-cristofer/pdfcraft` starts from current upstream `storytold/pdfcraftbb23`. It is not a GitHub fork, because an account can hold only one fork of a repo.
- Lane A order is now **P5 (port the crop-review work as fork-owned files) → P1 (rebrand + release)**; G1 moves about 1–2 weeks later.
- `eu-cristofer/pdfcraft` created (public, standalone). `~/GitHub/pdfcraft` now has `origin` = the new repo and `upstream` = `storytold/pdfcraft`, with `main` pushed at upstream `3109bb23`.
- `~/GitHub/pdfcraft.back` repointed to `eu-cristofer/pdfcraft_first`, so old-fork branches can no longer land in the new repo by accident.
- **Next:** expand the P5 spec and register it in `~/GitHub/pdfcraft`.

### 2026-10-29 — G0 reached: `001_suite-bootstrap` complete
- Sprint 1 (`a03f116`): workspace, licences, AGENTS/CLAUDE conventions, lint and `cargo deny` gates, stub crates.
- Sprint 2 (`eca7a13`): `evidence-format` v1. Reads the fork's v1.1 snapshot manifests unchanged and adds `suite.pump_test`; validation reports every issue.
- Sprint 3: frozen `pump-core` and `report-kit` contract types (D-008 accepted), 3-OS CI, private repo layout (`docs/private-repo-layout.md`).
- 37 tests; clippy, fmt and `cargo deny` clean locally. The fork repo was renamed on GitHub to `eu-cristofer/pdfcraft_first` (the old URL redirects).
- **Next:** merge the P0 PR; register `002_pump-core` and `003_report-kit` and run them in parallel (separate worktrees).

### 2026-10-29 — P0 and P1 registered
- `001_suite-bootstrap` registered in `inspection-suite` (branch `feat/001-suite-bootstrap`): 3 sprints.
- `006_pdf-release-v0-1` registered in the fork (branch `feat/006-pdf-release-v0-1`): 5 sprints. Release tag keeps the fork scheme `0.14.0-unsigned.<YYYYMMDD>`; "v0.1" is the product label.
- GitHub repos `eu-cristofer/inspection-suite` and `eu-cristofer/costa-inspection-private` not created yet (needs the owner's action; required before P0 sprint 3 CI and P1 sprint 3).
- **Next:** create the two GitHub repos; run both projects (`/sprint-loop run <slug>`) in separate terminals.

### 2026-10-29 — GitHub home: eu-cristofer, permanently
- D-012 supersedes D-011: all repos stay on the personal account `eu-cristofer`; no organisation.
- Costa Inspection remains copyright holder and `com.costainspection` id prefix.
- **Next:** create `eu-cristofer/inspection-suite` and `eu-cristofer/costa-inspection-private`, register P0 and P1.

### 2026-10-29 — typst and organisation decided
- D-010 accepted: PDF reports via embedded typst (P3 sprint 3 validates binary size and offline fonts first).
- D-011: GitHub organisation **Costa Inspection** (`costa-inspection`); private repo `costa-inspection-private`; id prefix `com.costainspection`.
- Still open: U1 (suite/app brand name), U2–U4.
- **Next:** create the org and repos on GitHub, register P0 and P1.

### 2026-10-29 — Official plan written
- Planning documents created in `inspection-suite/docs/`:
  - [official plan](plan/official-plan.md);
  - project specs P0–P4 (sprint-ready) and the P5–P7 backlog;
  - [decision log](decisions.md);
  - this journey.
- Decisions D-004 … D-010 were recorded:
  - 3 repos, 3 lanes;
  - ship first, rule of two;
  - API 610 with limits as data;
  - pump tool CLI first, with PDF + DOCX + package;
  - typst pending a spike.
- Answers from the user: team = user + AI sprint loops; brand name later (placeholder); pump interface = CLI first; outputs = PDF, DOCX and the evidence package.
- **Next:** U5 (org name), create `inspection-suite` on GitHub, register P0 and P1.

### 2026-10-28 — Review of the fork and upstream
- Compared the fresh upstream clone (`~/GitHub/pdfcraft`, `storytold/pdfcraft@3109bb23`) with the fork (`~/GitHub/pdfcraft.back`, `eu-cristofer/pdfcraft`, branch `feat/crop-panel-unification@17649c1b`).
- Upstream is **373 commits ahead** of the fork point `83ba2868` (2026-10-25). It changed 558 files and split the large `lib.rs`/`canvas.rs`/`control.rs` files into modules.
- The fork has 60 own commits (+10k lines), mostly in new files. The dry-run merge conflicts in **16 files**; the biggest liability is the ~2,800 lines added to the upstream-owned `crates/engine/src/export.rs`.
- Compliance findings:
  - ArtCraft trademarks are still embedded (`crates/ui-egui/src/widgets.rs:559-575`) while the fork publishes unsigned releases (D-003);
  - the Windows release zip lacks the OCR models.
- Licence check: MIT OR Apache-2.0, all dependencies permissive (`deny.toml`), fonts OFL. In-house commercial use and open-source publication are allowed (D-002).

### 2026-10-25 → 2026-10-28 — Fork feature work (before this plan)
- Fork created from `storytold/pdfcraft@83ba2868`.
- Five sprint projects were delivered with `/sprint-loop` in the fork:
  - `001_snapshot-export-contract`: manifest, LaTeX, raster evidence (archived 2026-10-26);
  - `002_selected-rec`: rectangle comments → export items (tracker still "active");
  - `003_export-flow`: headless and desktop rectangle export, OCR (tracker still "active");
  - `004_crop-review-workspace`: crops panel, metadata, project-folder persistence (archived 2026-10-26);
  - `005_export-contract-hardening`: manifest v1.1, `report.marker.json`, optional `report.docx`, embedded persistence (ready to archive).
- Also: a fork-sync SOP, an unsigned release workflow, a Windows no-admin portable spec, and course material (`docs/techind/`).

---

## Session entry template

```markdown
### YYYY-MM-DD — <short title>
- Lane/project: <A/B/C> · <slug> sprint N
- Done: <what changed, commits/PRs>
- Verified: <commands run, results>
- Issues / decisions: <blockers, D-xxx added>
- Next: <the single next action>
```
