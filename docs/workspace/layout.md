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
README or projection. A record or report effect may declare the short README
needed for its first real child. Every created path is part of the admitted
target set.

## Path Rules

Reject absolute paths, parent traversal, empty components, control characters,
symlink components, special files, case-collision ambiguity, and reserved
internal names. Use descriptor-relative no-follow opens for reads, checks,
writes, and recovery.

## File Size

Agent-owned Markdown targets at most 512 model tokens. Existing owner files may
be larger and are read in pages. A write-derived maintenance obligation may
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
