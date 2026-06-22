# Implementation

## Purpose

This directory owns the implementation substrate contract for lkjagent. Rust is
used for typed state, pure reducers, schema validation, deterministic audits,
and effect adapters.

## Table of Contents

- [rust.md](rust.md): Rust ownership in the workspace.
- [functional-core.md](functional-core.md): pure core and effects boundary.

## Local Map

- The model boundary is [../model-interface/contract.md](../model-interface/contract.md).
- State ownership is [../state/state-vector.md](../state/state-vector.md).
- Verification is [../operations/verification.md](../operations/verification.md).

## Status

implemented
