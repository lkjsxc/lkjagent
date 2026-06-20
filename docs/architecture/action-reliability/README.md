# Action Reliability

## Purpose

This directory defines how the action protocol recovers from parameter drift
without silently guessing semantic content.

## Table of Contents

- [parameter-contract.md](parameter-contract.md): accepted parameter behavior.
- [recovery.md](recovery.md): bounded recovery after action faults.
- [normalization.md](normalization.md): deterministic alias and drop rules.
- [error-messages.md](error-messages.md): actionable refusal text.

## Local Map

- [parameter-contract.md](parameter-contract.md): owns required, optional, and no-param rules.
- [recovery.md](recovery.md): owns recovery states and repetition bounds.
- [normalization.md](normalization.md): owns safe parameter rewrite rules.
- [error-messages.md](error-messages.md): owns model-visible correction format.

## Reading Paths

- Implementation path: parameter-contract, normalization, error-messages.
- Diagnosis path: error-messages, recovery, normalization.
- Verification path: recovery, then focused dispatch tests.

## Cross-Links

- Related contract: [../protocol/action-format.md](../protocol/action-format.md).
- Owning crate or module: `crates/lkjagent-tools/src/dispatch`.
