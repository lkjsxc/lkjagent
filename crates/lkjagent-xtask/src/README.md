# lkjagent-xtask Source

## Purpose

This directory holds the repository gate dispatcher, checks, and command
runner.

## Table of Contents

- [doc-common.rs](doc_common.rs): shared Markdown shape checks.
- [doc-special.rs](doc_special.rs): task and crate README checks.
- [doc-topology.rs](doc_topology.rs): docs README topology and All Files checks.
- [benchmark/](benchmark/README.md): benchmark commands and Docker runner.
- [docs.rs](docs.rs): check-docs composition.
- [facts.rs](facts.rs): repository fact collection.
- [lib.rs](lib.rs): public gate entrypoint.
- [lines.rs](lines.rs): check-lines implementation.
- [main.rs](main.rs): binary entrypoint.
- [model.rs](model.rs): shared check data.
- [runner.rs](runner.rs): quiet command execution.
- [structure/](structure/README.md): structure audit and plan commands.
- [style.rs](style.rs): check-style implementation.
