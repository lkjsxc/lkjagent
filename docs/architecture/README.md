# Architecture

## Purpose

This directory is the internal design of lkjagent: how the daemon, state
graph, context engine, action protocol, tools, memory, endpoint client, and
sandbox fit together. [overview.md](overview.md) is the map and the glossary.
Observable behavior lives in [../product/](../product/README.md); decisions
and rejected directions live in [../decisions/](../decisions/README.md).

## Table of Contents

- [overview.md](overview.md): component map, crate ownership, and glossary.
- [runtime/](runtime/README.md): turn authority, the agent loop, daemon process, queue intake, idle boundary.
- [state-graph/](state-graph/README.md): task cases, graph nodes, transitions, and evidence gates.
- [context/](context/README.md): window layout, budgets, compaction, caching, hygiene.
- [protocol/](protocol/README.md): action format, parsing, system prompt, recovery.
- [recovery/](recovery/README.md): deterministic recovery classes, retry budgets, and handoff.
- [document-structure/](document-structure/README.md): semantic document trees and catalog metadata.
- [artifacts/](artifacts/README.md): long-form content artifact profiles, manifests, audits, and completion.
- [structured-records/](structured-records/README.md): semantic records, identity, ledgers, and deduplication.
- [action-reliability/](action-reliability/README.md): parameter normalization and recovery.
- [state-model/](state-model/README.md): objective envelopes and ranked neutral tracks.
- [observability/](observability/README.md): token accounting, status, console, and model log.
- [tools/](tools/README.md): the fixed toolset and its contracts.
- [memory/](memory/README.md): SQLite store, graph evidence, transcripts, retrieval, distillation.
- [llm/](llm/README.md): endpoint contract, model target, sampling.
- [sandbox/](sandbox/README.md): container, workspace, safety model.
