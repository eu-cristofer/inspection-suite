# AGENTS.md

Binding rules for every human and agent working in this repo. If this file conflicts with `CLAUDE.md`, this file wins.

## Never panic

- Every input file is untrusted. Parsing and calculation functions return `Result` with the crate's error enum.
- No `unwrap`, `expect`, `panic!`, `unreachable!`, `todo!` or `unimplemented!` outside tests, unless provably infallible, with a comment saying why.
- No input-derived indexing: use `get`, not `[]`.
- Use checked arithmetic on input-derived numbers.
- Guard division by zero and NaN/inf before using a result.
- Cap every allocation whose size comes from input.

## Units

- Physical quantities use typed units (`uom`).
- Raw `f64` is allowed only inside numeric kernels, and must be documented as SI.

## Standards

- Never commit text, tables or figures from API, ISO or HI standards.
- Acceptance limits come only from data files (`tolerances.toml`) whose values the owner enters from the licensed copy.
- No API 610 number may appear in code.

## Data

- Never commit client names, real job data or real tolerance files to this public repo.
- Such material lives in the private repo `eu-cristofer/costa-inspection-private`.

## Licences and provenance

- All own code is `MIT OR Apache-2.0`.
- Every dependency must pass `cargo deny check` (permissive allow-list in `deny.toml`); copyleft is never linked.
- Every bundled asset or vendored crate gets a `NOTICE` entry with its licence.

## Crates

- Every new crate sets `[lints] workspace = true` and uses workspace-inherited package fields.
- Members are pre-declared in the root `Cargo.toml`; do not edit the member list to add a stub that already exists.
