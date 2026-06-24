# Runtime Kernel Source

## Purpose

This directory owns the pure runtime transition-kernel data model. It contains
types and reducers only. Store, filesystem, endpoint, shell, Docker, and clock
effects stay in daemon, store, tool, or CLI adapters.

## Table of Contents

- [active_mode.rs](active_mode.rs): runtime active modes.
- [snapshot.rs](snapshot.rs): durable snapshot records and newtype identifiers.
- [facts.rs](facts.rs): grouped case, graph, queue, evidence, artifact, context,
  observation, and maintenance facts.
- [event.rs](event.rs): runtime event payloads.
- [event_kind.rs](event_kind.rs): closed event catalog.
- [decision.rs](decision.rs): missions, decisions, templates, and invariants.
- [admission.rs](admission.rs): immutable dispatch admission views.
- [fault.rs](fault.rs): fault classes and retry keys.
- [effect.rs](effect.rs): deterministic runtime-owned effects.
- [render.rs](render.rs): prompt-card data.
- [reduce.rs](reduce.rs): pure mission selection and decision reduction.

## Ownership

The module does not perform I/O. It may be used by store, daemon, prompt,
dispatch, compaction, maintenance, and CLI adapters after those adapters build
or persist the required records.
