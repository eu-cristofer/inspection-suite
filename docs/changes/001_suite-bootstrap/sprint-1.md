# Sprint 1 — Workspace skeleton and conventions

## Goal

Turn the docs-only repo into a compiling Cargo workspace with:
- licences;
- contributor conventions;
- the lint and licence gates;
- a stub for every MVP member crate.

After this sprint, no later project needs to edit the root `Cargo.toml` to add a member.

## Files

**Create:**
- `Cargo.toml` (workspace root):
  - `[workspace] resolver = "3"`, `members = ["crates/evidence-format", "crates/pump-core", "crates/report-kit", "apps/pump-cli"]`;
  - `[workspace.package]`: `version = "0.0.1"`, `edition = "2024"`, a pinned `rust-version` (current stable), `license = "MIT OR Apache-2.0"`, `repository = "https://github.com/eu-cristofer/inspection-suite"`, `authors = ["Costa Inspection and the <Suite> contributors"]`;
  - `[workspace.lints.rust]`: `unsafe_code = "forbid"`;
  - `[workspace.lints.clippy]`: `unwrap_used = "deny"`, `expect_used = "deny"`, `panic = "deny"`, `todo = "deny"`, `unimplemented = "deny"`;
  - `[workspace.dependencies]`: initially empty, filled by later sprints.
- `clippy.toml`: `allow-unwrap-in-tests = true`, `allow-expect-in-tests = true`, `allow-panic-in-tests = true`.
- `rustfmt.toml`: copy from `~/GitHub/pdfcraft.back/rustfmt.toml`.
- `deny.toml`: copy from `~/GitHub/pdfcraft.back/deny.toml`. Keep the `[licenses] allow` list exactly (MIT, MIT-0, Apache-2.0, Apache-2.0 WITH LLVM-exception, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, BSL-1.0, CC0-1.0). Drop any PdfCraft-specific bans or exceptions that name crates the suite doesn't use.
- `LICENSE-MIT` and `LICENSE-APACHE`: standard texts. The MIT copyright line is `Copyright (c) 2026 Costa Inspection and the <Suite> contributors`.
- `NOTICE`: project name, the copyright line, "Licensed under the MIT License or the Apache License, Version 2.0, at your option", and a "Third-party material" section (empty for now).
- `CHANGELOG.md`: `## [Unreleased]` with a first bullet for this sprint.
- `CLAUDE.md` and `AGENTS.md` (AGENTS wins on conflict):
  - **Never panic.** Every input file is untrusted; return `Result` with the crate's error enum. No `unwrap`/`expect`/`panic!` outside tests unless provably infallible, with a comment.
  - **Units.** Physical quantities use typed units (`uom`); raw `f64` only inside numeric kernels, documented as SI.
  - **Standards.** Never commit text, tables or figures from API/ISO/HI standards. Acceptance limits come only from data files (`tolerances.toml`) whose values the owner enters from the licensed copy. No API 610 number may appear in code.
  - **Data.** Never commit client names, real job data or real tolerance files to this public repo; they live in the private repo `eu-cristofer/costa-inspection-private`.
  - **Quality gates** before every commit:
    - `cargo fmt --check`;
    - `cargo clippy -p <crate> --all-targets -- -D warnings`;
    - `cargo test -p <crate>`;
    - `cargo deny check`.
    Scope builds to the touched crates.
  - **Tests.** Prove a test fails before the fix by reverting the fix. Golden tests reproduce signed-off results.
  - **Commits and branches.** Use the sprint-loop protocol (`docs/changes/`). Feature branches only, with a PR into `main`.
  - **Pointers.** `docs/plan/official-plan.md`, `docs/journey.md`, `docs/decisions.md` (update the journey at the end of every session).
- Stub crates, each with `Cargo.toml` (`version.workspace = true`, `edition.workspace = true`, `license.workspace = true`, `[lints] workspace = true`) and `src/lib.rs` containing only a `//!` doc comment that states the crate's responsibility (one line each, from official plan §4):
  - `crates/evidence-format`;
  - `crates/pump-core`;
  - `crates/report-kit`.
- `apps/pump-cli`: `Cargo.toml` (`[[bin]] name = "pump-cli"`) and `src/main.rs` printing `pump-cli <version>` via `env!("CARGO_PKG_VERSION")` for `--version`, and a one-line usage otherwise. No clap yet.

**Modify:**
- `.gitignore`: keep `/target/`. Do **not** ignore `Cargo.lock`: this workspace ships binaries, so the lockfile is committed.
- `README.md`: add a "Building" section (`cargo build`, `cargo test`).

## Reuse

- `~/GitHub/pdfcraft.back/deny.toml` and `rustfmt.toml`, verbatim where applicable.
- Convention wording from `~/GitHub/pdfcraft.back/CLAUDE.md` ("Never panic", "Prove a test fails", "Scope builds").

## Tests

- `apps/pump-cli`: one integration test (`tests/version.rs`, using `std::process::Command` on `env!("CARGO_BIN_EXE_pump-cli")`) asserting that `--version` prints the package version and exits 0.

## Acceptance

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check        # install with `cargo install cargo-deny --locked` if missing
```

## Definition of done

- All acceptance commands pass.
- Every crate's `Cargo.toml` has `[lints] workspace = true`.
- `Cargo.lock` is committed.
- `CHANGELOG.md` has the bullet.
- The tracker is ticked.

## Assumes

- The repo is at `~/GitHub/inspection-suite` on branch `feat/001-suite-bootstrap`, containing only `README.md`, `.gitignore` and `docs/`. A Rust stable toolchain is installed.
- The GitHub repo `eu-cristofer/inspection-suite` is **not** required for this sprint (local only).
