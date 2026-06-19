# Builder Skills

## Purpose

The skill library for coding agents working on lkjagent. Every file obeys
the unified product format in
[../../architecture/skills/format.md](../../architecture/skills/format.md):
Purpose, Trigger, Context, Procedure, Checks, Must Not, optional Handoff,
at most 120 lines. Match by trigger line, load one skill, read its Context
before editing. Lifecycle (refinement and retirement) follows
[../../architecture/skills/lifecycle.md](../../architecture/skills/lifecycle.md),
executed through commits.

## Index

- [doc-contract-edit.md](doc-contract-edit.md): A contract under docs/ needs to change, with no code moving.
- [rust-crate-slice.md](rust-crate-slice.md): An execution task calls for implementing or extending a crate.
- [protocol-change.md](protocol-change.md): The action grammar, parser, or system prompt is changing.
- [context-engine.md](context-engine.md): Window layout, budgets, compaction, or caching rules are changing.
- [memory-store.md](memory-store.md): The store schema, transcripts, retrieval, or distillation is changing.
- [skill-system.md](skill-system.md): The skill format, loading, lifecycle, or library is changing.
- [verification-gate.md](verification-gate.md): A gate is being built or its checks are changing.
- [agent-maintenance.md](agent-maintenance.md): The manual, AGENTS.md, or this library needs maintenance.
- [benchmark-driven-improvement.md](benchmark-driven-improvement.md):
  A benchmark report is being used to choose or verify an agent improvement.

## Table of Contents

The index above is the table of contents: every sibling file, one trigger
clause each.
