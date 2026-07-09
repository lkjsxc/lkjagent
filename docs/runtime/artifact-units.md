# Artifact Units

## Purpose

Define how long owner artifacts are generated through small checked units and
assembled into the requested final shape.

## Manifest Contract

An artifact manifest records id, kind, title, root, schema number, audience,
objectives, source records, checks, status, layout rules, and units. The generic
manifest can describe reports, meeting packs, investigations, project design
docs, travel plans, transcripts, and exports.

## Unit Contract

An artifact root contains one or more files. Each file contains ordered units.
A unit stores stable id, optional parent id, target path, ordinal, target tokens,
target words when relevant, source refs, previous-tail refs, check requirements,
assembly policy, and the decision id that authorized generation.

Ordinary model-authored units target about 512 output tokens. A decision may use
a different cap only when its row records the reason and budget. The 200-line
repository file cap still applies to authored project files.

## Nested Paths

Unit output paths may be nested, for example `reports/q1/sections/intro.md`.
Nested paths are validated as workspace-relative paths before effects can write
or rebalance them.

## Assembly Rule

The harness assembles final owner files only after required unit checks pass.
The assembled file gets its own artifact fingerprint. Case closure requires
fresh checks that match current artifact fingerprints; model prose alone is not
completion evidence.

When generated artifact content spans multiple units, the requested artifact path
stores a short owner-readable manifest with size justification, full-body
fingerprint, and `.parts/` references. The full generated body is written to
checked `part-NNN.md` files beside the requested path. Parent artifact rows point
to the manifest path, and unit artifact rows point to the part paths.

## Long Artifact Rule

Large reports or document packs are generated as bounded units with source refs,
previous-tail continuity where relevant, deterministic assembly, and fresh checks
before completion. Shortfalls create new row-backed continuation work from the
current artifact state.

## Acceptance Checks

- Artifact manifests describe nested units, source refs, checks, assembly, and
  fingerprints.
- `lkjagent-app` write effects assemble checked units and persist file plus unit
  artifact fingerprints.
- Large generated artifacts leave a compact manifest plus checked part files
  instead of one oversized owner-facing file.
- `lkjagent-core` completion tests require fresh artifact fingerprints for
  closing evidence.

## Failure This Prevents

A long artifact does not depend on one large completion, and stale checks cannot
close work after generated units or assembled files change.
