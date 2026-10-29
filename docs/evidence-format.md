# Evidence format

Status: v1 implemented in `crates/evidence-format` · Last reviewed: 2026-10-29

The manifest (`manifest.json`) is the canonical description of an evidence package. Every other file in a package (reports, images, data) is derived from it or listed by it.

## Envelope

Every manifest is a JSON object with:

- `package_kind` (string): `pdfcraft.snapshot_export` or `suite.pump_test`.
- `manifest_version` (string): the version of that kind's contract, e.g. `"1.1"`.
- `items` (array): evidence items. Each item has:
  - an `id`: `item_1`, `item_2`, … in order, with no gaps or duplicates;
  - a `kind`;
  - kind-specific fields.
- Any other keys, at the top level or inside items, are preserved verbatim, so a read-then-write round trip loses no fields.

## Package kinds

| Kind | Supported versions | Allowed item kinds |
|---|---|---|
| `pdfcraft.snapshot_export` | 1.0, 1.1 | `snapshot` |
| `suite.pump_test` | 1.0 | `pump_acceptance`, `chart`, `data`, `input` |

The normative specification of the snapshot kind is `docs/snapshot-export-spec.md` in `eu-cristofer/pdfcraft`. Snapshot manifests are read unchanged.

## Path rules

These values must be normalized package-relative paths:
- any string under an object key named `path` or ending in `_path`;
- every string in an `evidence` array.

A normalized path:
- is not empty;
- uses `/` separators only (no backslashes);
- is not absolute;
- has no `..` segments;
- has no `.` segments and no duplicate separators. For example, `a//b.png` and `./a.png` are rejected as not normalized.

## The `tool` key

`tool` is an optional top-level object with `name`, `version` and an optional `commit`. Read and write it through `Manifest::tool` and `Manifest::set_tool`; a malformed value gives `FormatError::Tool`.

## Versioning policy

- Additive fields bump the minor version.
- Renames or removals bump the major version and need a decision entry in `docs/decisions.md`.

## JSON Schema snapshot

`json_schema(kind)` generates the schema. The committed snapshot is `crates/evidence-format/tests/snapshots/pump_test_schema.json`. To update it after an intended change:

```sh
UPDATE_SNAPSHOTS=1 cargo test -p evidence-format --test schema_snapshot
```
