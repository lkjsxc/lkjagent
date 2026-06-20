# Structured Records

## Purpose

Define the semantic record model that prevents duplicate Markdown trees and
duplicate memory rows. The agent records topic knowledge as durable records
with stable identity, then renders files from those records.

## Table of Contents

- [record-model.md](record-model.md): record kinds, identities, and ownership.
- [topic-map.md](topic-map.md): topic and subtopic graph shape.
- [artifact-ledger.md](artifact-ledger.md): artifact roots, manifests, and roles.
- [deduplication.md](deduplication.md): duplicate detection and write decisions.
- [write-planning.md](write-planning.md): bounded semantic file planning.
- [maintenance-records.md](maintenance-records.md): idempotent background records.
- [completion-evidence.md](completion-evidence.md): evidence required to close work.

## Contract

The runtime treats records as the source of truth for broad topics. Files and
memory rows are effects created from records after identity and duplicate
checks pass. A model may propose content, but it may not invent duplicate
paths, titles, or memory rows without runtime admission.

## Status

design, implementation pending
