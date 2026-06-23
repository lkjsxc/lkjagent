# Readiness Reducer

## Purpose

This file owns profile-specific content readiness decisions for artifact leaf files.

## Contract

Readiness is a reducer over artifact identity, profile, path role, objective, file content, and audit context. It
returns named missing requirements and weak signals. Keyword matches can support evidence, but they cannot prove
readiness alone.

## Inputs

- semantic artifact id and profile.
- path role such as recipe, technique, entry, reference, guide, or relation page.
- owner objective and normalized topic.
- file content and unsupported verification claims.
- manifest and relation context when relevant.

## Outputs

- pass or fail decision.
- missing requirement labels.
- weak signals and semantic mismatch details.
- content signals that support readiness.
- readiness evidence only when all required checks pass.

## Invariants

- Cookbook recipes include title, category, yield, time, ingredients with quantities, method, signals, notes, and
  semantic match.
- Techniques include concept, procedure, signals, common mistakes, corrective action, and applicability.
- Dictionary entries include term, term class, non-trivial definition, and requested optional fields.
- Documentation leaves include owned topic, contract, inputs or dependencies, outputs or effects, transitions or
  relations, and verification or audit links.
- Unsupported verification claims fail readiness.

## Failure Cases

- A shallow term list satisfies a detailed dictionary request.
- A README-only artifact satisfies content readiness.
- Generic scaffold prose passes because it contains domain keywords.
- Direct `graph.evidence` satisfies audit-owned readiness.

## Verification

- profile reducer tests for cookbook, technique, dictionary, and documentation leaves.
- `cargo test -p lkjagent-tools --test doc_tools`
- benchmark fixtures for weak dictionary, scaffold docs, and cookbook drift.

## Status

partially implemented for scaffold phrase refusal and selected profile checks; ledger-backed reducer work remains open.
