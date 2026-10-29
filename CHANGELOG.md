# Changelog

This changelog follows the sprint-loop protocol: each sprint records its user-visible changes under `[Unreleased]` until a release is cut.

## [Unreleased]

- **Added** the Cargo workspace skeleton (evidence-format, pump-core, report-kit, pump-cli stubs), licences, conventions (`CLAUDE.md`/`AGENTS.md`), lint and licence gates (`001_suite-bootstrap` sprint 1).
- **Added** `evidence-format` v1: manifest envelope for `pdfcraft.snapshot_export` (v1.0/1.1, read unchanged) and `suite.pump_test` (v1.0), validation that reports every issue, path normalization, JSON Schema (`001_suite-bootstrap` sprint 2).
- **Added** the frozen interface contract (D-008): `pump-core` and `report-kit` public types with not-implemented stubs, a 3-OS CI workflow, and the private data repo layout (`001_suite-bootstrap` sprint 3).
