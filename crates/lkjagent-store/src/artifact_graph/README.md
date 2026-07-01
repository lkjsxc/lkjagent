# Artifact Graph Store

## Purpose

This module owns SQLite adapters and plain data for durable artifact plans,
atoms, atom edges, write contracts, atom events, assembly runs, and readiness
projection rows.

## Table of Contents

- [model.rs](model.rs): plan, atom, edge, and contract row data.
- [projection.rs](projection.rs): event, assembly, and readiness data.
- [read.rs](read.rs): query adapters for plans, atoms, contracts, and readiness.
- [write.rs](write.rs): mutation adapters for plans, atoms, edges, and contracts.
- [events.rs](events.rs): event, assembly, and readiness mutation adapters.
