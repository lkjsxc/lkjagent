# Context Packages

## Purpose

Define the graph-selected replacement for loading whole procedure files into
the endpoint window.

## Package Shape

A context package is a source-owned, bounded instruction fragment selected by
the active graph node and phase. Each package has:

- name and token budget.
- node kinds and task families where it applies.
- stable instructions or checklist items.
- related docs or memory queries.
- compression rules for pressure states.

The prefix carries a graph slice listing selected package names and compact
instructions. Runtime log frames may contain package refresh notices when the
active phase changes.

## Selection

Selection considers task family, active node, missing evidence, touched paths,
context pressure, and recent failures. Green pressure allows normal packages.
Yellow pressure narrows package text. Orange schedules compaction at the next
safe boundary. Red compacts before endpoint or owner delivery. Black-invalid
pauses with a diagnostic.

The document-construction package is selected for documentation,
knowledge-base, and counted structured content tasks so large standalone
deliverables receive structure guidance before endpoint execution.

## Status

implemented.
