# Project Model Implementation

## Purpose

This relation page connects lkjagent as the central project, the provider-
neutral model interface, and Rust as the implementation substrate.

## Relations

- lkjagent depends on Rust for typed cases, reducers, schemas, and gates.
- lkjagent constrains the model endpoint through prompt frames and action
  validation.
- The model interface proposes actions; Rust runtime code authorizes and
  executes effects.
- Documentation audits observe this relation before completion.

## Backlinks

- Project contract: [../current-state.md](../current-state.md).
- Model interface: [../model-interface/contract.md](../model-interface/contract.md).
- Rust substrate: [../implementation/rust.md](../implementation/rust.md).

## Status

implemented
