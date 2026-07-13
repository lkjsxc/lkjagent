# Workspace Layout

## Purpose

Define one owner-visible root without eager scaffolding.

## Path Grammar

```text
workspace/
  README.md
  inbox/
  life/
    journal/YYYY/MM/DD/entry.md
    todo/open/
    todo/done/
    calendar/YYYY/MM/DD/
    finance/YYYY/MM/
    notes/
  knowledge/
    notes/
    references/
  projects/<project-slug>/
  artifacts/
    documents/
    reports/
  activity/
    conversations/YYYY/MM/DD/
    sessions/YYYY/MM/DD/
  indexes/
  archive/
  system/
    conflicts/
    operations/
```

This is a grammar, not a startup scaffold. Editing an existing file creates no
README or projection. The current journal slice creates only its canonical
`entry.md` and missing parent directories; it creates no README. Every newly
created path is part of the admitted target set.

## Path Rules

Reject absolute paths, parent traversal, empty components, control characters,
symlink components, special files, case-collision ambiguity, and reserved
internal names. Use descriptor-relative no-follow opens for reads, checks,
writes, and recovery.

## File Size

Agent-owned Markdown targets at most 512 conservative token units, computed as
the greater of UTF-8 character count and bytes divided by four rounded up.
Existing owner files may be larger and are read in pages. A write-derived maintenance obligation may
split an oversized agent-owned file into meaningful children and a map. It never
creates empty parts or rewrites owner files automatically.

## Projects

A project root can contain its own README, notes, tasks, repository checkout, and
reports. Context candidates retain exact project path and revision. Recent data
from another project is not eligible without a named cross-project need.

## Authority

Files are owner-readable content. SQLite owns runtime identity, revisions,
decisions, effects, checks, and projection lineage. Neither file prose nor a
workspace index selects runtime work.
