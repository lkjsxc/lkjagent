# lkjagent-xtask Source

## Purpose

This directory holds the repository gate dispatcher, checks, and command runner.

## Table of Contents

- [benchmark.rs](benchmark.rs): anchored scenario corpus command.
- [doc-catalog.rs](doc_catalog.rs): documentation catalog checks.
- [doc-common.rs](doc_common.rs): shared Markdown shape checks.
- [doc-crate-readmes.rs](doc_crate_readmes.rs): crate README coverage checks.
- [doc-links.rs](doc_links.rs): Markdown link checks.
- [doc-special.rs](doc_special.rs): repository-specific composition and
  reachability checks.
- [doc-topology.rs](doc_topology.rs): README topology checks.
- [docs-authority-contract.rs](docs_authority_contract.rs): native contract,
  baseline truth, schema, and retired-page assertions.
- [docs-authority-gate.rs](docs_authority_gate.rs): docs node composition and
  product source no-diff check.
- [docs-authority-product.rs](docs_authority_product.rs): Git-free product tree
  fingerprint used by the Docker node gate.
- [evaluation-harness/](evaluation_harness/README.md): deterministic scenarios,
  raw capture, PTY replay, and false-positive checks.
- [facts.rs](facts.rs): repository facts, shared check data, and evidence reads.
- [gate.rs](gate.rs): command parsing, size budgets, and quiet command runner.
- [lib.rs](lib.rs): public gate entrypoint.
- [main.rs](main.rs): binary entrypoint.
- [node-gate.rs](node_gate.rs): workgraph node gate router and baseline checks.
- [repository-determinism-gate.rs](repository_determinism_gate.rs): tracked
  build inputs, exact configuration, Docker, workflow, and focused suites.
- [style.rs](style.rs): style, dependency, and structure audits.
