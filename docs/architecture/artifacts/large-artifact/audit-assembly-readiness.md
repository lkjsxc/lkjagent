# Audit Assembly Readiness

## Purpose

Define the real-file audit loop that updates atom rows, runs deterministic
assembly, and projects completion readiness.

## Audit

`artifact.audit` measures files on disk. It excludes README, catalog, manifest,
transcript, owner-request, and other navigation files from final content counts.
For each atom it records the measured count, weak classes, status, and event.
Story-bible-only, outline-only, scaffold-only, placeholder, owner-term-only, and
generic-example content never satisfies manuscript or large-artifact floors.

## Assembly

When source atoms for an assembled target are ready, the daemon assembles the
final target deterministically. Manuscript scenes assemble into chapter files;
other profiles may assemble indexes or completion evidence. Every run records
source atom ids, target paths, measured count, status, and summary. If a target
already exists with enough measured prose, audit records the same assembled
truth instead of leaving a pending assembly blocker.

## Readiness

The readiness projection is ready only when required atoms are ready or
assembled, measured totals meet the accepted floor, no active contract remains,
and no weak blockers remain. Scene atoms with assembled targets do not keep
`assembly_pending` true after the target atom is ready or represented as
assembled. Completion gates read this projection together with graph evidence
and runtime faults.

## Status

implemented.
