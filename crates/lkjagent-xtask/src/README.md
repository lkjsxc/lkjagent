# lkjagent-xtask Source

## Purpose

Map repository checks, acceptance, evaluation helpers, and command dispatch.

## Table of Contents

- [acceptance.rs](acceptance.rs): acceptance command and report orchestration.
- [acceptance/](acceptance/README.md): plans, evidence, Git, and secret checks.
- [doc_common.rs](doc_common.rs): shared Markdown shape checks.
- [doc_links.rs](doc_links.rs): Markdown link checks.
- [doc_special.rs](doc_special.rs): docs composition and reachability.
- [doc_topology.rs](doc_topology.rs): README topology checks.
- [docs_authority_contract.rs](docs_authority_contract.rs): compact authority facts.
- [docs_authority_gate.rs](docs_authority_gate.rs): docs authority command.
- [evaluation_harness/](evaluation_harness/README.md): scenario and evidence helpers.
- [facts.rs](facts.rs): repository file facts and shared evidence reads.
- [gate.rs](gate.rs): command parser, file budgets, and quiet runner.
- [lib.rs](lib.rs): public command entrypoint.
- [main.rs](main.rs): binary entrypoint.
- [node_gate.rs](node_gate.rs): focused docs, evaluation, and repository gate router.
- [style.rs](style.rs): source, dependency, and structure audits.
