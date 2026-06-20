# Memory Records

## Purpose

Define durable memory rows as structured records, not repeated free-form
lessons produced by idle loops.

## Kinds

Accepted kinds are `lesson`, `fact`, `task-summary`, and `incident`.
Maintenance must not invent new memory kinds or save rows that merely say
nothing useful changed.

## Identity

```text
memory_key = kind + normalized title + normalized tags + content hash prefix
```

Exact duplicates return the existing id. Same title plus high content overlap
updates or skips. Task summaries may key by case id so closed tasks do not
overwrite unrelated summaries.

## Maintenance

Maintenance checks identity before saving. Pruning must run a real update,
delete, or merge, or return a no-op outcome with cooldown.

## Status

partially implemented; accepted kinds, exact duplicate skip, same-title
overlap update, and exact duplicate prune exist. Semantic merges remain open.
