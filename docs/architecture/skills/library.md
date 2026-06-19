# Library

## Purpose

Specify where skills live and what the library contains on first start.

## Location

The runtime library is source-owned, not data-owned. In the container image
the daemon reads flat markdown files from
`/usr/local/share/lkjagent/skills`; in local development the CLI falls back
to `crates/lkjagent-skills/seeds`. The data directory does not expose a
skill library, and the running agent does not edit skills.

The store keeps only index stamps (name, use counts, and outcome stamps)
per [../memory/store.md](../memory/store.md); the source files are the
truth.

Flat by design: the index line is the discovery mechanism, so directories
would only add path noise. If the library ever outgrows flatness, that is a
budget problem to solve in [loading.md](loading.md), not a taxonomy problem.

## Seed Set

The image ships the seed set as source content. Startup indexes that
directory directly; there is no first-start copy step. The seed set stays
small and changes through repository edits, tests, and commits.

| Seed | Trigger summary |
| --- | --- |
| workspace-survey | Opening a task in an unfamiliar workspace |
| narrow-verification | A change needs the smallest command proving it works |
| git-checkpoint | Work reached a state worth a commit with an honest message |
| web-research | The answer is outside the container and curl can fetch it |
| owner-report | A long task needs a compact, honest progress answer |
| recursive-structure | A task asks for recursive project structure creation or maintenance |

Seed bodies live in the harness repository and are validated by the same
checks as builder skills; their content is part of the implementation work
in [../../execution/tasks/skill-runtime.md](../../execution/tasks/skill-runtime.md).

## What the Library Is Not

- Not a package ecosystem: nothing installs skills from runtime data, and
  the agent does not write skill files.
- Not configuration: skills teach procedures; values that tune the harness
  live in lkjagent.json per [../../operations/running.md](../../operations/running.md).
- Not memory: a skill says how to do something repeatable; memory records
  what happened and what it taught ([../memory/distillation.md](../memory/distillation.md)).

## Status

implemented.
