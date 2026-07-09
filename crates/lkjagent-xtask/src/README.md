# lkjagent-xtask Source

## Purpose

This directory holds the repository gate dispatcher, checks, and command runner.

## Table of Contents

- [benchmark.rs](benchmark.rs): anchored scenario corpus command.
- [doc-catalog.rs](doc_catalog.rs): documentation catalog checks.
- [doc-common.rs](doc_common.rs): shared Markdown shape checks.
- [doc-crate-readmes.rs](doc_crate_readmes.rs): crate README coverage checks.
- [doc-links.rs](doc_links.rs): Markdown link checks.
- [doc-reachability.rs](doc_reachability.rs): docs reachability checks.
- [doc-special.rs](doc_special.rs): repository-specific documentation checks.
- [doc-topology.rs](doc_topology.rs): README topology checks.
- [docs.rs](docs.rs): check-docs composition.
- [docs-authority-contract.rs](docs_authority_contract.rs): native contract,
  baseline truth, schema, and retired-page assertions.
- [docs-authority-gate.rs](docs_authority_gate.rs): docs node composition and
  product source no-diff check.
- [docs-authority-product.rs](docs_authority_product.rs): Git-free product tree
  fingerprint used by the Docker node gate.
- [evaluation-harness/](evaluation_harness/README.md): deterministic scenarios,
  raw capture, PTY replay, and false-positive checks.
- [facts.rs](facts.rs): repository fact collection.
- [file-counts.rs](file_counts.rs): workspace file budget gate.
- [lib.rs](lib.rs): public gate entrypoint.
- [lines.rs](lines.rs): check-lines implementation.
- [main.rs](main.rs): binary entrypoint.
- [model.rs](model.rs): shared check data.
- [node-gate.rs](node_gate.rs): workgraph node gate router and baseline checks.
- [node-gate-evidence.rs](node_gate_evidence.rs): baseline evidence readers.
- [runner.rs](runner.rs): quiet command execution.
- [structure/](structure/README.md): structure audit commands.
- [style.rs](style.rs): check-style implementation.
