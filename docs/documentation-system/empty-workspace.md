# Empty Workspace Growth

## Purpose

This file owns the first documentation tree that lkjagent creates when the workspace has no useful
artifact root yet.

## Contract

The first write is relation-first and objective-first. It creates a small map of the owner request,
known terms, evidence gates, and relations before any broad project archetype appears.

The seed contains these roles:

- root README: local navigation and completion gate summary.
- request pages: owner objective, owner terms, and source boundary.
- project pages: operating rules and evidence gates for the artifact.
- relation pages: topic map and artifact map.
- topic pages: one page per requested topic when the topic can be named without a long combined slug.

## Source Boundary

Generated topic pages preserve owner-provided words and local source paths only. They do not invent
external domain facts. A page for `minecraft`, `windows`, `japan`, or `united-states` says the term
was requested until a local source supplies facts.

## Growth Rule

Architecture, guides, operations, overview, and reference directories appear only after relation
coverage shows that the artifact needs those categories. Topology can pass before content readiness,
but topology never proves artifact readiness.

## Verification

- Write-contract tests prove the first tree has README coverage and no generic project forest.
- `doc.audit` reports topology separately from content readiness.
- Artifact completion reads audit-owned readiness, not README presence.

## Status

design-only
