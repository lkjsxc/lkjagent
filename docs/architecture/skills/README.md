# Skills

## Purpose

This directory specifies the skill system: markdown capability files in one
unified format, indexed cheaply in the prefix, loaded whole on demand, and
refined through source changes. Skills are how lkjagent grows capability
without exposing mutable runtime data. Decision:
[../../decisions/unified-skills.md](../../decisions/unified-skills.md).
Owned by the lkjagent-skills crate.

## Table of Contents

- [format.md](format.md): the canonical skill shape both audiences obey.
- [loading.md](loading.md): index in the prefix, bodies on demand.
- [lifecycle.md](lifecycle.md): creation, refinement, and retirement.
- [library.md](library.md): where skills live and what ships as seed.
