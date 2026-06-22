# Semantic Audits

## Purpose

Semantic audits own quality gates that topology cannot prove. Documentation
completion requires topology, semantic seed, relation, mock-content,
model-name, objective-match, and completion audits.

## Audit Layers

- Topology: README coverage, child links, internal links, line caps, and no
  directory ending in `.md`.
- Semantic seed: root purpose, topic contracts, concrete concept files, and no
  root-only topic blurbs.
- Relation: requested topics are connected by relation pages and backlinks.
- Mock content: repeated templates, placeholder phrases, and low project anchor
  density fail.
- Model names: durable display text uses provider-neutral terms unless a raw
  fixture or adapter page allows a name.
- Objective match: the structure satisfies the owner task and avoids unrelated
  scaffolds.

## Completion Rule

`agent.done` is illegal when any audit-owned evidence is absent, stale, or
blocked by an active guard track.

## Status

implemented
