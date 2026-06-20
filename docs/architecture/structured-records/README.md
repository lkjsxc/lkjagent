# Structured Records

## Purpose

Define how arbitrary topics, content artifacts, and durable memory are
represented without duplicate Markdown clutter. The agent records topic
knowledge as durable records with stable identity, then renders files from
those records.

## Table of Contents

- [record-model.md](record-model.md): record kinds, identities, and ownership.
- [identity.md](identity.md): stable semantic keys for records, artifacts, sections, memory, and failures.
- [topic-map.md](topic-map.md): topic and subtopic graph shape.
- [artifact-ledger.md](artifact-ledger.md): artifact roots, manifests, and roles.
- [deduplication.md](deduplication.md): duplicate detection and write decisions.
- [write-planning.md](write-planning.md): bounded semantic file planning.
- [memory-records.md](memory-records.md): idempotent memory entries and maintenance writes.
- [completion-evidence.md](completion-evidence.md): evidence required to close work.

## Contract

The runtime treats records as the source of truth for broad topics. Files and
memory rows are effects created from records after identity and duplicate
checks pass. A model may propose content, but it may not invent duplicate
paths, titles, or memory rows without runtime admission.

## Status

partially implemented; memory exact dedupe, content classification, and
semantic scaffold profiles exist. Stable artifact adoption, section repair,
and semantic memory merge remain open.
