# lkjagent Documentation

## Purpose

This tree is the implementation contract for lkjagent. Code follows docs.
Start with [current-state.md](current-state.md), then follow the README for the
area being changed. Navigation uses README, catalog, relation, and fan-out
contracts. Machine-readable coverage lives in
[_meta/catalog/](./_meta/catalog/README.md); README files are for human and LLM
navigation.

## Table of Contents

- [current-state.md](current-state.md): honest ledger of implemented behavior, partial behavior, and open evidence.
- [vision/](vision/README.md): north star, principles, and scope boundaries.
- [product/](product/README.md): observable behavior of the daemon, CLI, and queue.
- [state/](state/README.md): hard state, weighted tracks, transitions, and guards.
- [prompting/](prompting/README.md): state-derived prompt frames and prompt modes.
- [documentation-system/](documentation-system/README.md): documentation contracts, growth, and audits.
- [model-interface/](model-interface/README.md): provider-neutral model boundary and terms.
- [implementation/](implementation/README.md): Rust substrate and functional core boundary.
- [relations/](relations/README.md): cross-topic relation pages.
- [maintenance/](maintenance/README.md): structural health, bias, commonality, and no-op policy.
- [verification/](verification/README.md): semantic gates and regression fixture contracts.
- [regressions/](regressions/README.md): owner-reported failure fixtures.
- [architecture/](architecture/README.md): runtime, graph, context, protocol, tools, memory, LLM, and sandbox contracts.
- [evaluation/](evaluation/README.md): mechanical benchmark tasks, judges, reports, and improvement loop.
- [decisions/](decisions/README.md): durable decision records with rejected directions.
- [repository/](repository/README.md): layout, line limits, doc standards, style, commits, workflow.
- [operations/](operations/README.md): verification gates, Compose design, and running the harness.
- [agent/](agent/README.md): manual for the coding agents that build lkjagent.
- [execution/](execution/README.md): operating rules, blocker queue, and executable tasks.
- [_meta/](_meta/README.md): catalog, graph rules, and observed documentation audit notes.

## Checks

- `cargo run -p lkjagent-xtask -- check-docs`
