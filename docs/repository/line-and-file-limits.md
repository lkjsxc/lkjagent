# Line And File Limits

## Purpose

Define size budgets for authored files and product surfaces.

## Line Cap

Every authored file is capped by `repository.file.line-cap=200` lines. This
includes Markdown, Rust, scripts, config, Compose files, and workflows.
Generated logs under `data/logs/` are diagnostic evidence and are not authored
source.

## Token Target

Project-authored files target about 512 tokens when practical. This is an
authoring target, not a hard gate. Owner-requested final artifacts may be
larger when generated from small checked units and deterministic assembly.

## File Budgets

| Surface | Budget |
| --- | ---: |
| documentation under `docs/` | `repository.docs-file-budget=100` |
| product Rust source files | `repository.product-source-file-budget=170` |

The warning threshold is `repository.file-budget.warn-percent=90`. Warnings are
for planning; hard failures happen at the budget.

## Split Rule

Split by ownership. A doc approaching the cap becomes a directory only when it
has at least two real child topics. A Rust module splits along model, pure
transition, adapter, and error ownership.

## Gate

`check-lines` enforces line caps. `check-files` enforces file-count budgets.
