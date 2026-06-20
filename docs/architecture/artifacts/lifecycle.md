# Lifecycle

## Purpose

Define the ordered states for semantic content artifacts.

## States

An artifact moves through four distinct states:

- scaffold: semantic root, README indexes, and manifest exist.
- content pass: meaningful leaf files exist under semantic roles.
- audit pass: topology, links, manifest, line limits, and content readiness pass.
- completion: graph and artifact evidence prove the requested deliverable.

Scaffold is never content evidence for cookbooks, stories, guides, knowledge
bases, or long reports. `artifact.apply` may create the scaffold and a work
queue, but it cannot claim the artifact is complete.

## Runtime

The graph routes large content work to artifact planning, scaffold or
adoption, bounded content batches, audit, repair, and completion. A failed
state names exact missing paths and the next executable action.
