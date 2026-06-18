# Library

## Purpose

Specify where skills live and what the library contains on first start.

## Location

The runtime library is /data/skills on the data volume: flat directory,
one markdown file per skill, a git repository for provenance per
[lifecycle.md](lifecycle.md). The store keeps only index stamps (name, use
counts, refinement and outcome stamps) per
[../memory/store.md](../memory/store.md); the files are the truth.

Flat by design: the index line is the discovery mechanism, so directories
would only add path noise. If the library ever outgrows flatness, that is a
budget problem to solve in [loading.md](loading.md), not a taxonomy problem.

## Seeding

First start with an empty /data/skills copies the seed set from the image.
Seeds are written like any other skill and carry no special status; the
agent refines or retires them like the rest. The seed set stays small: the
library is meant to be grown by its owner-agent pair, not shipped.

| Seed | Trigger summary |
| --- | --- |
| workspace-survey | Opening a task in an unfamiliar workspace |
| narrow-verification | A change needs the smallest command proving it works |
| git-checkpoint | Work reached a state worth a commit with an honest message |
| web-research | The answer is outside the container and curl can fetch it |
| owner-report | A long task needs a compact, honest progress answer |

Seed bodies live in the harness repository and are validated by the same
checks as builder skills; their content is part of the implementation work
in [../../execution/tasks/skill-runtime.md](../../execution/tasks/skill-runtime.md).

## What the Library Is Not

- Not a package ecosystem: nothing installs skills from anywhere; the agent
  writes its own or the owner drops files into /data/skills.
- Not configuration: skills teach procedures; values that tune the harness
  live in lkjagent.json per [../../operations/running.md](../../operations/running.md).
- Not memory: a skill says how to do something repeatable; memory records
  what happened and what it taught ([../memory/distillation.md](../memory/distillation.md)).

## Status

implemented.
