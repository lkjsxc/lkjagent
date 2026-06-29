# Dense Runtime State Network

## Purpose

Define the active redesign work that follows the completed runtime smoke repair.
The goal is a dense, deterministic authority network where every turn persists
why a decision exists and what can safely happen next.

## Problem

The persisted daemon path exists, but the read model is still too sparse for a
weak local model. Some facts are only present as rendered text or recent
observations, resolver plans can still depend on fallback selection, and close
paths are not all described by one typed completion packet.

## Target Contract

Every owner turn records these rows or an equivalent typed projection:

- case and artifact identity, including exact title, kind, profile, scale, and
  lifecycle;
- facts derived from snapshots, audits, observations, provider faults, and
  artifact ledgers;
- obligations created by facts and evidence requirements;
- one total resolver plan, with no mission fallback branch;
- one decision kind derived from that resolver plan;
- deterministic runtime effect command or one admitted model surface;
- progress key that proves the route advanced or is exhausted;
- completion gate inputs and refusals for every close path;
- staleness fingerprint shared by prompt rendering and admission.

## Runtime-Owned Effects

Deterministic audits, root inspections, status reads, compaction, blocked
handoffs, cooldowns, idle transitions, and close effects should run without a
provider exchange when the persisted decision already contains the needed
facts. Semantic content remains model-authored and requires an exact current
write contract.

## Artifact Scale

Artifact identity must preserve the owner title. Story and long-novel work must
record requested scale, profile requirements, weak semantic groups, and the next
bounded paths to write. A small identity seed is not sufficient for large story
completion.

## Implemented Slice

Focused tests now cover typed intent profile routing, owner-title and non-ASCII
artifact identity, counted story scale, persisted dense authority rows, total
resolver labels, deterministic audit runtime effects, progress keys, prompt and
admission fingerprints, and typed completion inputs. Workspace tests, benchmark,
quiet verify, and Docker verify prove the complete route.

## Acceptance

- `Compact Compass` is story artifact work, not compaction.
- `iwanna` preserves owner-title root identity.
- Missing roots route to write progress or blocked handoff, not same-root audit
  repetition.
- Generic roots and generic examples are absent when a current root exists.
- Deterministic audits and inspections can bypass provider calls.
- Every close path uses the same typed completion gate.
- Prompt rendering and admission cite the same decision and fingerprint.
- Docker Compose final verification passes before completion is claimed.

## Related Task

The executable task is
[../tasks/dense-runtime-state-network.md](../tasks/dense-runtime-state-network.md).
