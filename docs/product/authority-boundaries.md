# Authority Boundaries

## Purpose

Assign each kind of durable information to one owner.

## Owner Content

Workspace Markdown owns durable owner-readable source content: records, notes,
project material, artifacts, and activity history.

## Runtime Control

SQLite owns runtime events, state cells and edges, decisions, operations,
leases, effect intents, admissions, observations, checks, context provenance,
usage, conversation sequence, and wake conditions.

## Derived Projections

Navigation, search indexes, workspace manifests, summaries, TUI snapshots, and
proof reports are rebuildable projections. They may accelerate or explain
behavior but cannot override source bytes or runtime events.

## Model And Harness

The model authors bounded content and proposes explicitly admitted exploratory
operations. It does not choose unrestricted paths, mutate state, invent tool
availability, or declare completion.

Rust owns intent obligations, paths, reduction, selection, tool views, budgets,
admission, effects, recovery, checks, and final state.

## Configuration

`data/lkjagent.json` owns flat non-secret owner settings. Secrets come from
named environment variables and never enter prompts or evidence bodies.
