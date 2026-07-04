# Completion

## Purpose

Define how plan items and cases finish through fresh evidence.

## Completion Authority

Completion belongs to the harness. Model output can provide content or a sparse
judgment block, but terminal closure is reachable only when the active
`RuntimeDecision` names completion predicates and current check rows satisfy
them.

## State Cells

Completion is represented by state cells such as `completion:check-pending`,
`completion:check-passed`, `completion:check-failed`,
`completion:close-candidate`, `completion:blocked`, and `completion:closed`.
These cells carry evidence refs and artifact fingerprints.

## Step Checks

A plan item may attach checks from `checks.catalog.names` in
[../checks/catalog.md](../checks/catalog.md). Checks run after the selected
effect and before the item can become done. A failed check produces a bounded
diagnosis with measured values and a recovery state edge.

## Case Checks

When no runnable work remains, case checks run over the real workspace and
current artifacts. A manuscript case checks chapter file count, total words, and
objective-specific constraints. A docs-tree case checks README coverage and
relative links.

## Evidence Rows

Each check result stores check name, parameters, pass flag, measured value or
structured JSON, artifact refs, decision id, and timestamp. Status, display,
benchmarks, replay, and proof bundles read those rows rather than model prose.

## Stale Evidence

If an artifact changes after a passing check, the reducer suppresses dependent
`completion:check-passed` cells or creates a fresh check requirement. Artifact
fingerprints, not timestamps alone, drive freshness.

## Failure This Prevents

False completion is structurally blocked, and old passing evidence cannot close
a changed case.
