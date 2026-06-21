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

## Invariants

- Adoption must attach the root to the active case before completion.
- Adoption does not bypass content readiness.
- Adopted artifacts enter repair when profile-specific fields are missing.

## Failure Cases

- A shallow existing dictionary is adopted and immediately completed.
- A duplicate root is created instead of repairing the equivalent root.
- The runtime loses artifact identity after compaction.

## Verification

Adoption tests attach an existing root to the active case, preserve artifact
identity across compaction, and route incomplete adopted roots to repair.

## Related Files

- [repair.md](repair.md)
- [content-readiness.md](content-readiness.md)
- [manifest.md](manifest.md)
