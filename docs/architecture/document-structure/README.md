# Document Structure

## Purpose

This directory defines documentation as a navigable knowledge graph projected
into a recursive filesystem tree.

## Table of Contents

- [tree-contract.md](tree-contract.md): recursive README-indexed tree rules.
- [network-contract.md](network-contract.md): graph manifest and cross-link rules.
- [naming.md](naming.md): semantic filename and directory rules.
- [scaffold-profiles.md](scaffold-profiles.md): deterministic profile shapes.
- [audit.md](audit.md): topology audit requirements.
- [completion-gates.md](completion-gates.md): evidence needed before closure.

## Local Map

- [tree-contract.md](tree-contract.md): owns directory and README shape.
- [network-contract.md](network-contract.md): owns catalog-derived graph data.
- [naming.md](naming.md): owns forbidden sequence-only names.
- [scaffold-profiles.md](scaffold-profiles.md): owns profile selection.
- [audit.md](audit.md): owns deterministic audit checks.
- [completion-gates.md](completion-gates.md): owns close criteria.

## Reading Paths

- Implementation path: scaffold-profiles, naming, tree-contract, network-contract.
- Diagnosis path: audit, naming, completion-gates.
- Verification path: audit, completion-gates, then `docker compose run --rm verify`.

## Cross-Links

- Related contract: [../tools/doc-tools.md](../tools/doc-tools.md).
- Owning crate or module: `crates/lkjagent-tools/src/doc.rs`.
