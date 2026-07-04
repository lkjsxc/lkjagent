# Artifact Units

## Purpose

Define how long owner artifacts are generated through small checked units and
assembled into the requested final shape.

## Unit Contract

An artifact root contains one or more files. Each file contains ordered units.
A unit stores target path, ordinal, target tokens, target words when relevant,
source context keys, previous-tail refs, check requirements, assembly policy,
and the decision id that authorized generation.

Ordinary model-authored units target about 512 output tokens. A decision may use
a different cap only when its row records the reason and budget. The 200-line
repository file cap still applies to authored project files.

## Assembly Rule

The harness assembles final owner files only after required unit checks pass.
The assembled file gets its own artifact fingerprint. Case closure requires
fresh checks that match current artifact fingerprints; model prose alone is not
completion evidence.

## Longform Rule

Long manuscripts generate settings first, compact them into clean context items,
generate chapter units with previous-tail continuity, assemble chapter files,
and run per-chapter and aggregate word checks before completion.

## Failure This Prevents

A long artifact does not depend on one large completion, and stale checks cannot
close work after generated units or assembled files change.
