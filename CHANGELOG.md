# Changelog

This changelog follows the sprint-loop protocol: each sprint records its user-visible changes under `[Unreleased]` until a release is cut.

## [Unreleased]

- **Added** the Cargo workspace skeleton (evidence-format, pump-core, report-kit, pump-cli stubs), licences, conventions (`CLAUDE.md`/`AGENTS.md`), lint and licence gates (`001_suite-bootstrap` sprint 1).
- **Added** `evidence-format` v1: manifest envelope for `pdfcraft.snapshot_export` (v1.0/1.1, read unchanged) and `suite.pump_test` (v1.0), validation that reports every issue, path normalization, JSON Schema (`001_suite-bootstrap` sprint 2).
