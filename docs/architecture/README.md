# Architecture

## Purpose

This directory is the internal design of lkjagent: how the daemon, context
engine, action protocol, tools, memory, skills, endpoint client, and sandbox
fit together. [overview.md](overview.md) is the map and the glossary; each
subdirectory owns one subsystem. Observable behavior lives in
[../product/](../product/README.md); decisions and their rejected directions
live in [../decisions/](../decisions/README.md).

## Table of Contents

- [overview.md](overview.md): component map, crate ownership, and glossary.
- [runtime/](runtime/README.md): the agent loop, daemon process, queue intake, idle boundary.
- [context/](context/README.md): window layout, budgets, compaction, caching, hygiene.
- [protocol/](protocol/README.md): action format, parsing, system prompt, recovery.
- [tools/](tools/README.md): the fixed toolset and its contracts.
- [memory/](memory/README.md): SQLite store, transcripts, retrieval, distillation.
- [skills/](skills/README.md): skill format, loading, lifecycle, library.
- [llm/](llm/README.md): endpoint contract, model target, sampling.
- [sandbox/](sandbox/README.md): container, workspace, safety model.
