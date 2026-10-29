# 001_suite-bootstrap

Creates the `inspection-suite` Cargo workspace (repo `eu-cristofer/inspection-suite`):
- conventions, licences, CI and stub crates for every MVP member;
- the frozen interface contracts (`evidence-format` v1, the `pump-core` and `report-kit` public types);
- the layout of the private data repo.

This is gate **G0** of the [official plan](../../plan/official-plan.md). It unblocks `002_pump-core` (lane B) and `003_report-kit` (lane C) to run in parallel. Source spec: [`docs/plan/projects/P0-suite-bootstrap.md`](../../plan/projects/P0-suite-bootstrap.md).

# 001_suite-bootstrap — Loop Protocol

Each run MUST:
1. Read this README top to bottom. Pick the FIRST sprint whose checkbox is `[ ]`.
   Never skip ahead; never start a sprint whose predecessor is unchecked.
2. Open its sprint-N.md and verify the "Assumes" line against the code. If an
   assumption fails, fix forward only if trivial; otherwise write
   `BLOCKED: <reason>` under the tracker line and STOP (do not tick).
3. Implement exactly the sprint's scope (via `/sprint-loop next <project>` or
   `/sprint-loop run <project>`: the drafter drafts, the orchestrator
   reviews/applies). Resist scope creep.
4. Run the sprint's Acceptance commands — all must pass.
5. Update the changelog (one bullet per user-visible change).
6. Tick the checkbox `[x]` and append `— done YYYY-MM-DD`.
7. Commit in one commit: `<001_suite-bootstrap> sprint N: <title>`.
8. When all sprints are checked, mark the project `Status: ready to archive` and
   archive it later with `/sprint-loop archive <project>`.

Status: active

- [x] [Sprint 1 — Workspace skeleton and conventions](./sprint-1.md) — done 2026-10-29
- [ ] [Sprint 2 — `evidence-format` v1](./sprint-2.md)
- [ ] [Sprint 3 — Freeze the interface types, CI and the private repo layout](./sprint-3.md)
