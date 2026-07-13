# Acceptance Source

## Purpose

Map the source-bound acceptance checker modules.

## Table of Contents

- [args.rs](args.rs): closed acceptance command parsing.
- [build-manifest.rs](build_manifest.rs): source and campaign binary binding.
- [campaign-evidence.rs](campaign_evidence.rs): strict scenario fact derivation.
- [campaign-predicates.rs](campaign_predicates.rs): scenario-specific measurements.
- [command-evidence.rs](command_evidence.rs): deterministic and Docker receipts.
- [evidence.rs](evidence.rs): attachment content and campaign checks.
- [experiments.rs](experiments.rs): concrete cell and scenario validation.
- [git.rs](git.rs): source, plan, tracking, and evidence path checks.
- [history.rs](history.rs): reachable Git object and index scanning.
- [markers.rs](markers.rs): evidence marker extraction.
- [plans.rs](plans.rs): tracked plan orchestration.
- [review.rs](review.rs): independent review fingerprints and coverage.
- [secret.rs](secret.rs): high-confidence secret pattern checks.
- [source.rs](source.rs): frozen-source validation and contract derivation.
- [source-facts.rs](source_facts.rs): exact implementation and test predicates.
- [source-audit.rs](source_audit.rs): uniqueness, style, and candidate removal.
- [synthetic.rs](synthetic.rs): bounded compressed database validation.
- [table.rs](table.rs): bounded TSV parsing.
- [workgraph.rs](workgraph.rs): dependency and final-ancestry validation.
