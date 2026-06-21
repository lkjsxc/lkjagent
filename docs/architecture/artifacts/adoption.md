# Adoption

## Purpose

Define how the runtime handles an existing artifact root.

## Contract

Before creating a new root, artifact planning inspects known roots and
manifests for an equivalent semantic artifact. If an equivalent root already
exists, the runtime adopts it and continues from its readiness gaps instead of
creating a duplicate tree. A root path alone is not identity.

## Artifact Record

```text
ArtifactRecord
- artifact_id
- root
- owner_case_id
- semantic_title
- artifact_kind
- topic
- domain
- requested_scale
- required_sections
- expected_leaf_profiles
- manifest_path
- readiness_state
- batch_cursor
```

## Identity

Adoption compares artifact kind, normalized title, topic, domain, root role,
owner objective hash when available, README title, manifest key, semantic
section roles, and expected leaf profiles.

For a Japanese cookbook, the semantic id must reject unrelated generic bread
scaffolds even when a directory name looks plausible. Adoption may keep an
incomplete equivalent root, but it must enter repair instead of completion.

## Evidence

An adoption result records the adopted root, source manifest if present,
semantic id, readiness gaps, and next executable action.

## Invariants

- Adoption must attach the root to the active case before completion.
- Adoption does not bypass content readiness.
- Adopted artifacts enter repair when profile-specific fields are missing.
- Completion consumes readiness only from the active artifact id.
- Compaction snapshots preserve artifact id and root together.

## Failure Cases

- A shallow existing dictionary is adopted and immediately completed.
- A duplicate root is created instead of repairing the equivalent root.
- A Japanese cookbook adopts bread leaves as ready content.
- The runtime loses artifact identity after compaction.

## Verification

Adoption tests attach an existing root to the active case, preserve artifact
identity across compaction, route incomplete adopted roots to repair, and fail
wrong-domain cookbook leaves.

## Related Files

- [repair.md](repair.md)
- [content-readiness.md](content-readiness.md)
- [manifest.md](manifest.md)
