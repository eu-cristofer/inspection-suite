# 001_suite-bootstrap — Motivation

## Problem

Costa Inspection is building an open-source inspection suite: a PDF Review tool (the PdfCraft fork) and an API 610 Pump Performance tool. The pump tool is built by **two parallel AI sprint lanes**:
- `002_pump-core`: calculations;
- `003_report-kit`: DOCX, PDF and charts.

Parallel lanes collide unless:
- the shared files are settled first: the root `Cargo.toml`, the lint config and CI;
- the types the lanes exchange are frozen before either lane starts.

Without a frozen contract, each lane invents its own version of `AcceptanceReport`/`ReportDoc`, and the integration project (`004_pump-cli`) becomes a rewrite.

## Approach

1. **Skeleton first.** Declare every MVP member crate as a compiling stub, so later lanes never edit the workspace root.
2. **Port the existing contract instead of inventing one.** `evidence-format` reads the fork's existing `pdfcraft.snapshot_export` v1.1 manifests unchanged and adds the `suite.pump_test` package kind.
3. **Freeze the types, not the logic.**
   - The `pump-core` and `report-kit` public types and signatures are merged with `Err(NotImplemented)` bodies.
   - After G0, changing them needs a decision entry.
4. **Conventions carried over from PdfCraft:**
   - no panics on untrusted input;
   - workspace lints;
   - scoped builds;
   - tests proven to fail;
   - only the permissive licence allow-list.
   Plus suite rules: no standard text (API/ISO/HI), no client data, acceptance limits only from data files.

## Key decisions

| Date | Decision | Ref |
|---|---|---|
| 2026-10-29 | 3 repos; this workspace holds everything Costa Inspection owns | D-004 |
| 2026-10-29 | Ship first, rule of two for shared crates | D-005 |
| 2026-10-29 | API 610 acceptance limits are data | D-006 |
| 2026-10-29 | Interface contract frozen at the end of this project | D-008 (pending → accepted at G0) |
| 2026-10-29 | PDF reports via embedded typst | D-010 |
| 2026-10-29 | All repos on `eu-cristofer` permanently | D-012 |

Full log: [`docs/decisions.md`](../../decisions.md).

## Risks

| Risk | Mitigation |
|---|---|
| Frozen types turn out wrong once P2/P3 start | Types are minimal and `Serialize`-only. Changes go through a decision entry and a heads-up to both lanes. Adding a field is cheap; renaming is not. |
| `evidence-format` breaks compatibility with the fork's v1.1 manifests | Round-trip test on a real fixture copied from the fork; unknown fields are preserved via `extra` |
| CI is slow or flaky on Windows/macOS runners | Scoped jobs; pinned action SHAs; matrix limited to fmt/clippy/test/deny |
| `cargo deny` rejects a needed crate (e.g. `schemars`, `uom`) | All candidates are MIT/Apache; check with `cargo deny check licenses` before adding |
