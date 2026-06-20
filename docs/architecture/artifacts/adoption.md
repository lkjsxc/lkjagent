# Adoption

## Purpose

Define how the runtime handles an existing artifact root.

## Contract

Before creating a new root, artifact planning inspects known roots and
manifests for an equivalent artifact kind and title. If an equivalent root
already exists, the runtime adopts it and continues from its readiness gaps
instead of creating a duplicate tree.

## Identity

Adoption compares artifact kind, normalized title, root role, owner objective
hash when available, README title, manifest key, and semantic section roles.

## Evidence

An adoption result records the adopted root, source manifest if present,
readiness gaps, and next executable action.
