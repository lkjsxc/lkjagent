# Agent Manual

## Purpose

This directory is the manual for the coding agents that build lkjagent: the
work loop, the handoff format, the honesty rule, and the skill library. It
routes; it does not restate. Repository-wide policies live in
[../repository/](../repository/README.md); execution state lives in
[../execution/](../execution/README.md).

## Authority

- [work-loop.md](work-loop.md) owns the order of work in a session.
- [handoff.md](handoff.md) owns how a session ends.
- [honest-state.md](honest-state.md) owns the truth rule for the whole
  project, runtime included; every other file links to it.
- [skills/](skills/README.md) owns task-shaped procedure; skills follow the
  product format in [../architecture/skills/format.md](../architecture/skills/format.md).

## Table of Contents

- [work-loop.md](work-loop.md): the session loop from orientation to handoff.
- [handoff.md](handoff.md): the evidence-first final report and commit rules.
- [honest-state.md](honest-state.md): the no-fake-behavior rule, canonically.
- [skills/](skills/README.md): the builder skill library.
