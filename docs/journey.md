# Dev journey

Status: living document · Last reviewed: 2026-10-29
Official plan: [`plan/official-plan.md`](plan/official-plan.md) · Decisions: [`decisions.md`](decisions.md)

This is the running log of the project: where we are, what happened, what's next. Update it at the end of every working session and at every gate. Newest entries go at the top of the log.

---

## Status board

| ID | Project | Repo | Lane | Status | Progress | Next action |
|---|---|---|---|---|---|---|
| P0 | `001_suite-bootstrap` | inspection-suite | user | ✅ done (ready to archive) | 3/3 | Merge PR into `main`; archive with `/sprint-loop archive` |
| P1 | `006_pdf-release-v0-1` | fork `eu-cristofer/pdfcraft` | A | 🟦 registered (branch `feat/006-pdf-release-v0-1`, commit 29119187, local only) | 0/5 | `/sprint-loop run 006_pdf-release-v0-1` in `~/GitHub/pdfcraft.back` (sprint 5 waits for U1) |
| P2 | `002_pump-core` | inspection-suite | B | ⬜ ready to register | 0/7 | Register after the P0 PR merges; run in its own worktree |
| P3 | `003_report-kit` | inspection-suite | C | ⬜ ready to register | 0/6 | Register after the P0 PR merges; run in its own worktree |
| P4 | `004_pump-cli` | inspection-suite | B | ⏸ blocked by P2, P3 | 0/6 | — |
| P5–P7 | backlog | — | — | 💤 later | — | P5 after G1 (in the fork); P6–P7 at G3 |

Legend: ⬜ not started · 🟦 in progress · ✅ done · ⏸ blocked · 💤 backlog

### Gates

| Gate | Target | Status |
|---|---|---|
| G0 — Contract frozen | Day 1 | ✅ 2026-10-29 |
| G1 — PDF Review v0.1 public | Week 1 | ⬜ |
| G2 — Report kit ready | Week 2 | ⬜ |
| G3 — Pump Performance v0.1 | Week 3–4 | ⬜ |

### User inputs

| ID | Input | Needed by | Status |
|---|---|---|---|
| U1 | Suite and tool names, icon | G1 (P1 s5) | ⬜ open |
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
