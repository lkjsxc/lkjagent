# State

## Purpose

Define the durable state vector that replaces fixed plan-only authority.

## Table of Contents

- [cells.md](cells.md): state keys, cell fields, evidence, and unknown-key rules.
- [edges.md](edges.md): dense state-edge evidence and authority limits.
- [reducer-and-selectors.md](reducer-and-selectors.md): event reduction,
  state patches, and deterministic candidate selection.

## Failure This Prevents

The runtime can represent many simultaneous facts without forcing them into one
coarse task state or a closed Rust enum.
