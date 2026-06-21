# Readiness Evidence

## Purpose

Define what can and cannot satisfy artifact readiness.

## Contract

Readiness is semantic. Structure can prove that paths exist and links are
complete. Readiness requires content-bearing leaves that satisfy the active
profile and an audit result that names those leaves as passing.

## Cookbook Evidence

Recipe leaves require title, description, ingredients, method, timing or
yield, and notes or troubleshooting. Technique leaves require title, purpose
or concept, procedure, signals, common mistakes, corrective action, and
verification notes.

## Non-Evidence

Scaffolds, manifests, README indexes, headings without body content, status
notes, generic "replace this" prose, and unsupported verification claims are
not readiness evidence.

## Invariants

- Readiness reads actual file content.
- Scaffold-only paths stay weak even when the directory audit passes.
- Failed readiness returns exact missing requirements.
- Passing readiness contributes evidence only with passing structure audit.

## Fixture

`cookbook_weak_content_audit` proves tiny scaffold leaves remain weak.

## Verification

Run `cargo test -p lkjagent-tools doc_content_audit`.

## Status

partially implemented.
