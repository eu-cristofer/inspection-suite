# Dev journey

Status: living document · Last reviewed: 2026-10-29
Official plan: [`plan/official-plan.md`](plan/official-plan.md) · Decisions: [`decisions.md`](decisions.md)

This is the running log of the project: where we are, what happened, what's next. Update it at the end of every working session and at every gate. Newest entries go at the top of the log.

---

## Status board

| ID | Project | Repo | Lane | Status | Progress | Next action |
|---|---|---|---|---|---|---|
| P0 | `001_suite-bootstrap` | inspection-suite | user | ⬜ not registered | 0/3 | Create repos `eu-cristofer/inspection-suite` (public) + `eu-cristofer/costa-inspection-private` (private), `git init`, register P0 |
| P1 | `006_pdf-release-v0-1` | fork | A | ⬜ not registered | 0/5 | Register in fork on a branch from `feat/crop-panel-unification` |
| P2 | `002_pump-core` | inspection-suite | B | ⏸ blocked by G0 | 0/7 | Wait for P0 |
| P3 | `003_report-kit` | inspection-suite | C | ⏸ blocked by G0 | 0/6 | Wait for P0 |
| P4 | `004_pump-cli` | inspection-suite | B | ⏸ blocked by P2, P3 | 0/6 | — |
| P5–P7 | backlog | — | — | 💤 later | — | Expand specs at G1 / G3 |

Legend: ⬜ not started · 🟦 in progress · ✅ done · ⏸ blocked · 💤 backlog

### Gates

| Gate | Target | Status |
|---|---|---|
| G0 — Contract frozen | Day 1 | ⬜ |
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
