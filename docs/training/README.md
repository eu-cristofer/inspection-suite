# Training: building and maintaining WitnessPoint PDF Review

Status: training material · Written 2026-11-01, at gate G1 · English

This folder teaches how the first WitnessPoint tool was planned, built and released, and how to keep it alive when upstream PdfCraft moves on. It is written from the project's own records: [`../journey.md`](../journey.md), [`../decisions.md`](../decisions.md), the fork's CHANGELOG, the GitHub PRs and runs, and the first planning session.

## Audience and goals

For engineers, AI-assisted maintainers and the owner. After working through it you should be able to:

1. explain *why* the fork is a downstream product and what that changes;
2. run a release and its smoke test, and know which checks block publishing;
3. merge a new upstream PdfCraft version without losing the fork's identity, OCR or crop review;
4. judge the project against its original idea, and know what comes next.

## Contents

| File | What it is | Time |
|---|---|---|
| [`slides.md`](slides.md) | Marp presentation (34 slides, speaker notes in comments) | 45 min talk |
| [01 — Build record](01-build-record.md) | The record: a dated timeline with every PR, run and decision | 20 min |
| [02 — Didactic notes](02-didactic-notes.md) | 11 concepts (what it is / here / takeaway) and 6 exercises | 40 min + exercises |
| [03 — Maintain and upgrade](03-maintain-and-upgrade.md) | Handbook: repo map, routine tasks, **"a new PdfCraft version"** procedure, must-keep checklist | reference |
| [04 — Retrospective](04-retrospective.md) | The first session's idea vs. the outcome, item by item, with a verdict and recommendations | 15 min |
| [`diagrams/`](diagrams/) | The Mermaid sources (`.mmd`) and the rendered SVGs used by the slides | — |

Suggested order: slides → 01 → 02 (do the exercises in a clone of the fork) → 03 → 04.

## Diagrams

Each diagram has one source, `diagrams/<name>.mmd`. The notes include the same Mermaid code inline, so GitHub renders it. Marp can't render Mermaid, so the slides use the pre-rendered SVGs. After editing a `.mmd`, re-render it and paste the new code into the notes:

```sh
cd docs/training/diagrams
for f in *.mmd; do npx -y @mermaid-js/mermaid-cli@11.4.2 -i "$f" -o "${f%.mmd}.svg" -b white -c mermaid-config.json; done
```

## Presenting the slides

```sh
cd docs/training
npx -y @marp-team/marp-cli@4.0.3 slides.md -o slides.html          # browser
npx -y @marp-team/marp-cli@4.0.3 slides.md --pdf --allow-local-files  # PDF
```

Or open `slides.md` in VS Code with the "Marp for VS Code" extension.

## Rules for this material

- No client data and no API 610 text or values, the same as the rest of this repo (`AGENTS.md`).
- No ArtCraft logos or marks; the plain-text "Based on PdfCraft" attribution is allowed.
- When the process changes, update the handbook (03) and add a journey entry. Leave 01 and 04 as records of G1.
