# CLAUDE.md

Day-to-day working instructions. Binding rules are in `AGENTS.md` and win on any conflict.

## Read first

- `AGENTS.md`: never-panic, units, standards, data and licence rules. In short: no standard text and no client data in this repo, and acceptance limits only from data files.
- `docs/plan/official-plan.md`: scope, architecture, projects, gates.
- `docs/journey.md`: status board and session log. Update it at the end of every session.
- `docs/decisions.md`: decision log. Record new decisions there; contract changes after G0 need an entry.

## Quality gates (before every commit)

- `cargo fmt --check`
- `cargo clippy -p <crate> --all-targets -- -D warnings`
- `cargo test -p <crate>`
- `cargo deny check`
- Scope builds and tests to the crates you touched (`-p <crate>`), not the whole workspace.

## Tests

- Prove a test fails before the fix by reverting the fix and watching it fail.
- Golden tests reproduce signed-off results; never regenerate them to make a test pass.

## New crates

- Add `[lints] workspace = true` and inherit package fields from `[workspace.package]`.

## Commits and branches

- Follow the sprint-loop protocol: projects and sprints live under `docs/changes/`.
- Work on feature branches only; open a PR into `main`; never push to `main` directly.
