# Private repo layout

Status: accepted (`001_suite-bootstrap` sprint 3) · Last reviewed: 2026-10-29

Some material never lives in this public repo:
- licensed-standard values;
- real client data;
- signed-off golden numbers.

It lives in a separate private repo, **`eu-cristofer/costa-inspection-private`** (visibility: private).

## Layout

```
README.md            rules: never push to a public remote; anonymise clients (Client-A, Job-001)
tolerances/README.md file format = docs/tolerances.md (written in 002_pump-core); values only from the licensed API 610 copy;
                     file name api610-<edition>.toml; the `source` field names the table, the person and the date
tolerances/          one file per edition
golden/README.md     case layout <case>/{job.toml,test.csv,tolerances.toml,expected.toml}; expected = signed-off numbers
golden/
templates/           client report expectations (input U4)
```

## Golden harness

The `002_pump-core` golden harness reads the environment variable `SUITE_PRIVATE_GOLDEN=<path>/golden`. When it is unset, the private golden cases are skipped. The harness never falls back to real data inside the public repo; the public repo carries synthetic cases only.

## Initial README for the private repo

```markdown
# costa-inspection-private

PRIVATE. Holds licensed-standard tolerance values, anonymised real test data and signed-off golden cases for inspection-suite.

## Rules

- Never push this repo to a public remote or mirror it.
- Anonymise clients and jobs (Client-A, Job-001): no client names, plant tags or serial numbers.
- Tolerance values come only from the owner's licensed API 610 copy.

## Layout

- `tolerances/`: one file per edition, named `api610-<edition>.toml`. The `source` field names the table, the person and the date.
- `golden/`: cases as `<case>/{job.toml,test.csv,tolerances.toml,expected.toml}`; `expected.toml` holds signed-off numbers.
- `templates/`: client report expectations.

## Use from inspection-suite

    export SUITE_PRIVATE_GOLDEN=<path-to-this-repo>/golden
    cargo test --workspace
```
