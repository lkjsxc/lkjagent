# Authority Boundaries

## Owner Content

Workspace Markdown owns durable, owner-readable source content: records, notes,
project material, artifacts, and activity history.

## Runtime Control

SQLite owns runtime events, state cells, edges, decisions, operation leases,
effect intents, admissions, observations, checks, context provenance, usage,
and wake conditions.

## Derived Projections

Generated navigation indexes, search indexes, workspace manifests, summaries,
TUI snapshots, and proof reports are visible but rebuildable projections. Edits
to a generated index never overwrite source records. Projections may accelerate
or explain behavior but cannot override source content or runtime events.

## Model

The model authors bounded content, chooses among explicitly admitted exploratory
operations, and proposes progress. It does not choose unrestricted paths,
invent tool availability, mutate state directly, or declare work complete.

## Harness

Rust owns intent obligations, paths, state reduction, selection, tool views,
budgets, admission, effects, recovery, checks, and final state.

## Configuration

data/lkjagent.json owns flat non-secret owner settings. Secrets come from named
environment variables and never enter prompts or evidence bodies.
