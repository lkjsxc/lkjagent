# Records

## Purpose

Define one generic Markdown record contract for personal, work, and development
objects.

## Shape

Each record is Markdown with YAML-like frontmatter followed by owner-readable
body sections.

```text
---
id: rec_20260705_120000_slug
kind: todo
title: Pay electricity bill
state: open
created_at: 2026-07-05T12:00:00Z
updated_at: 2026-07-05T12:00:00Z
tags: [home, finance]
links: []
state_keys: [todo:open/rec_20260705_120000_slug]
---

# Pay electricity bill

## Body

...
```

## Identity

The `id` is stable and sorts by creation time. Paths are convenient storage
locations, not identity. Pure workspace entity refs use the record id as the
stable entity id, so a rebalance can validate path moves without changing
ledger links. The canonical file implementation stores records under semantic directories
such as `workspace/records/life/journal/<id>.md`,
`workspace/records/life/todo/<id>.md`, or
`workspace/records/work/projects/<id>.md`. Archived records move under
`workspace/archive/records/...` with aliases preserved. Unknown `kind` values
are valid records and must list, show, link, archive, and round-trip without
central enum edits.

## Ledger Links

Record writes refresh `workspace_records` metadata,
`workspace_record_history` fingerprints, README path coverage, and generated
record indexes. These rows and files are evidence and indexes, not turn
authority. Frontmatter refs may point to owner messages, state keys, checks,
provider exchanges, artifacts, other records, or proof bundles.

## Staleness

Editing a record changes its fingerprint. Reducers mark dependent indexes,
checks, completion evidence, prompt frames, matter briefs, and state edges stale
or superseded before selectors can refresh them.
