# Acceptance Source

## Purpose

Map the source-bound acceptance checker modules.

## Table of Contents

- [evidence.rs](evidence.rs): attachment content and campaign checks.
- [experiments.rs](experiments.rs): concrete cell and scenario validation.
- [git.rs](git.rs): source, plan, tracking, and evidence path checks.
- [history.rs](history.rs): reachable Git object and index scanning.
- [markers.rs](markers.rs): evidence marker extraction.
- [plans.rs](plans.rs): tracked plan orchestration.
- [secret.rs](secret.rs): high-confidence secret pattern checks.
- [source.rs](source.rs): frozen-source validation and contract derivation.
- [source-facts.rs](source_facts.rs): exact implementation and test predicates.
- [table.rs](table.rs): bounded TSV parsing.
- [workgraph.rs](workgraph.rs): dependency and final-ancestry validation.
