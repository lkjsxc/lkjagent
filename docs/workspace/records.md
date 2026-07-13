# Workspace Records

## Purpose

Define grounded records, sourced memory, activity projections, and reports.

## Record Tool

The normal descriptor registry exposes `write_record` with exact `family`,
`title`, and `body` fields during orient, modify, and named recovery. This slice
admits only exact `family=journal`; review and respond expose no record tool. The
persisted descriptor, admission, and `workspace.record.journal` effect key drive
dispatch. There is no public record command, scheduler, manifest, separate
grammar, or direct writer.

## Journal

A diary request writes `life/journal/YYYY/MM/DD/entry.md`. Selection binds the
effective fixed-offset workspace timezone, wall time, and local date in the
immutable decision context before provider intent. A later config edit or slow
call cannot move that path.

The harness renders UTF-8 Markdown with kind, date, and exact selected source
fingerprints, then adds model-authored title and body. The whole document must be
nonempty, free of known placeholders, and at most 512 conservative token units.
The model cannot choose path, date, or frontmatter. Sparse evidence may produce
a short honest reflection that states uncertainty. Success requires native
lineage, effect, revision, structural check, generic byte/content/collateral
checks, and a receipt that binds checked path and revision. Scripted tests prove
mechanics only; configured-model campaigns own semantic groundedness.

## Other Families

Add one complete family at a time: TODO by open/done state, calendar by date,
finance by month, ordinary note by semantic slug, then project note below one
project root. Each uses the common effect, revision, observation, and check path.

## Memory

Memory begins as sourced owner-readable Markdown below `knowledge/`. Store only
explicit owner facts/preferences, durable project facts, or verified paths needed
later. Each entry names source identity and effective date when known.

Do not store secrets, failed model claims, transient faults, or duplicated
objective prose. A later current owner correction wins. Acceptance requires one
fact to influence a fresh matter exactly once.

## Activity

Canonical conversation and runtime rows may project bounded readable receipts to
`activity/conversations` and `activity/sessions`. Projections name logical IDs,
causal sequence, changed paths, and checks, but contain no raw prompt, failed
body, secret, or state JSON. They are evidence, not control authority.

## Reports

A short report uses one real file below `artifacts/documents` or
`artifacts/reports`. Longer output uses meaningful semantic children plus a
README map, each below the token cap. No empty or placeholder part exists.
Checks cover paths, links, order, fingerprints, sources, and final receipt.

## Maintenance

Only an explicit or write-derived obligation may update affected README files,
split agent-owned content, remove exact duplicate memory, or detect revision
drift. Path indexes, archive policy, and richer retrieval wait for measured need.
