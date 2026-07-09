# State

## Purpose

Define the durable concurrent state vector reduced from runtime events.

## Table of Contents

- [cells.md](cells.md): state keys, cell fields, evidence, and unknown-key rules.
- [edges.md](edges.md): dense state-edge evidence and authority limits.
- [reducer-and-selectors.md](reducer-and-selectors.md): event reduction,
  transition guards, state patches, and deterministic candidate selection.

## Failure This Prevents

The runtime can represent many simultaneous facts without forcing them into one
coarse matter state or a closed Rust enum.
