# Personal Materialized Views

## Purpose

Define generated workspace Markdown views for personal records.

## Contract

Generated Markdown is a projection from SQLite. It is not canonical record state
and direct edits are ignored unless an import contract is documented and
implemented.

## Layout

The projection writes under `personal/` in the configured data directory and
uses bounded indexes:

```text
personal/journal/YYYY/MM/YYYY-MM-DD.md
personal/schedule/events/<id>-<slug>.md
personal/schedule/months/YYYY-MM.md
personal/todos/open.md
personal/todos/projects/<project-slug>.md
```

Each generated file includes stable record IDs. Monthly and project splits keep
files under line limits as the record set grows.

## Regeneration

Projection writes are idempotent. A write may run after each mutation or through
a dedicated render route, but either path must read from the store and must obey
placeholder and line-limit checks. The CLI render route removes stale generated
files before writing the current bounded projection.
