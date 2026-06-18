# Unified Skills

## Purpose

Fix the relationship between the skills the harness runs on and the skills
the coding agents use to build the harness.

## Decision

One skill format serves both audiences. The canonical shape is owned by
[../architecture/skills/format.md](../architecture/skills/format.md). The
runtime loads skills from its library at
[../architecture/skills/library.md](../architecture/skills/library.md); the
builders load theirs from [../agent/skills/README.md](../agent/skills/README.md).
Both sets obey the same headings, the same line cap, and the same lifecycle.

## Consequences

- The format gets twice the testing: every friction the builders feel is a
  bug report for the runtime, and the reverse.
- The harness can maintain its own skills with the same machinery the
  builders use through explicit maintenance paths described in
  [../architecture/runtime/self-maintenance.md](../architecture/runtime/self-maintenance.md).
- A skill written for building lkjagent is loadable by lkjagent once it runs;
  the project bootstraps its own capability library.
- There is exactly one format document to keep sharp, honoring the one-rule
  one-owner principle.

## Rejected Directions

- Two formats tuned per audience: doubles the contracts and guarantees the
  runtime format rots while the builder format gets the attention.
- Generic frontmatter-based skill conventions from other ecosystems: YAML
  frontmatter wastes parser effort and reads worse to a model than plain
  headed markdown.
