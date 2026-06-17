# Skill Operations

## Purpose

The contracts for skill.use and skill.save: loading a skill into the
window and writing one into the library. Skill structure is owned by
[../skills/format.md](../skills/format.md), loading mechanics by
[../skills/loading.md](../skills/loading.md), and the improvement cycle by
[../skills/lifecycle.md](../skills/lifecycle.md). Canonical parameter
table: [registry.md](registry.md).

## skill.use

| Parameter | Rule |
| --- | --- |
| name | required |

Appends the skill body as an immutable skill frame per
[../skills/loading.md](../skills/loading.md).

| Error | Response |
| --- | --- |
| unknown name | tool error |
| already loaded | notice pointing at the earlier skill frame |
| concurrent skill budget exceeded | refused per [../context/budgets.md](../context/budgets.md) |

## skill.save

| Parameter | Rule |
| --- | --- |
| name | required |
| content | required |

Validates content against the unified skill format
([../skills/format.md](../skills/format.md)) and writes the markdown file
into the library. The observation states the path and the validation
verdict. A skill that fails validation is not written.

## Deferred Index Visibility

A saved skill does not appear in the prefix skill index until the next
compaction rebuilds the prefix; mid-task prefix edits would poison the
endpoint cache per [../context/caching.md](../context/caching.md). The
skill file is on disk the moment skill.save returns; only index visibility
waits for compaction.

## Status

implemented.
