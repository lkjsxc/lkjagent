# Workspace Records

## Purpose

Define grounded records, sourced memory, activity projections, and reports.

## Record Tool

The normal descriptor registry exposes `write_record` with exact `family`,
`title`, and `body` fields during orient, modify, and named recovery. This slice
admits exact `family=journal`, `family=memory`, and `family=report`; review and
respond expose no record tool. The persisted descriptor and admission retain one honest
`workspace.record` effect key. Family dispatch occurs only on that admitted
path. There is no public record command, scheduler, index authority, manifest,
separate grammar, or direct writer.

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

## Memory

A memory write derives one nonempty lowercase kebab slug from the bounded title
and writes `knowledge/notes/<slug>.md`. The harness owns path and frontmatter:
`kind: memory`, semantic key, slug, and exact current owner source fingerprints.
It rejects unsafe controls, known placeholders, a title with no safe slug,
missing owner lineage, prohibited sensitive/raw-output forms, and a rendered
file above 512 conservative token units before workspace mutation.

The existing exact effect handles declared parents, managed replacement,
unmanaged collision, revisions, checks, receipts, stale owner bytes, and retry
safety. Memory adds `managed-memory`; journal retains `managed-journal` without
weaker predicates. Deterministic checks prove lineage, shape, placeholder and
size bounds. Whether model-authored prose is semantically true or merely filler
remains a configured-campaign judgment and is not claimed by scripted tests.

Later matters may receive only current active managed native revisions below
`knowledge/notes/*.md` whose producing effect settled and source matter closed.
Candidates carry exact path and current revision ID, are deduplicated by semantic
key/current revision, and are bounded to four items, 2,048 bytes, and 512
conservative token units, with a 1,024-byte/256-unit item cap. The filesystem is
not scanned. Superseded revisions, open source matters, other roots, malformed
records, and arbitrary files are absent. A row enters `context_items` only if
compilation selects it, and its body enters the prompt at most once.

A current objective line `forget <slug>:` or `correct <slug>:` suppresses the
matching memory candidate during assembly. This proves only exact textual key
correction; it is not a general semantic correction claim or a second control
plane.

## Other Families

Add one complete family at a time: TODO by open/done state, calendar by date,
finance by month, then project note below one project root. Each uses the common
effect, revision, observation, and check path.

## Activity

Canonical conversation and runtime rows may project bounded readable receipts to
`activity/conversations` and `activity/sessions`. Projections name logical IDs,
causal sequence, changed paths, and checks, but contain no raw prompt, failed
body, secret, or state JSON. They are evidence, not control authority.

## Reports

A short report derives a bounded slug from its title and writes one real file at
`artifacts/reports/<slug>.md`. Harness-owned frontmatter records kind, semantic
key, slug, and the ordered kind/fingerprint pair for every selected persisted
context item. The report requires nonempty lineage and a safe title and body;
the whole file is capped at 512 conservative token units.

The shared exact effect provides declared parents, collision and stale-byte
protection, managed replacement revisions, generic file checks, and receipts.
`managed-report` additionally checks canonical structure, exact lineage,
nonempty prose, placeholders, and size. Reports never enter memory retrieval.
Longer README maps and semantic children are not implemented yet.

## Maintenance

Only an explicit or write-derived obligation may update affected README files,
split agent-owned content, remove exact duplicate memory, or detect revision
drift. Path indexes, archive policy, and richer retrieval wait for measured need.
