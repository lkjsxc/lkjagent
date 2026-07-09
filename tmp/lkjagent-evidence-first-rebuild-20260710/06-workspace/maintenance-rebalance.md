# Maintenance And Rebalance

## Preview

Rebalance produces a plan containing affected documents, semantic reason,
source fingerprints, target paths, link edits, index changes, and rollback
strategy. Preview is owner-readable under system/operations.

## Apply

Use operation groups and atomic renames. Move related semantic files together,
repair structured links, update projections, validate postconditions, and remove
only empty generated directories.

## Safety

- refuse changed source fingerprints;
- never use raw global string replacement for links;
- preserve owner-authored prose;
- retain aliases until link validation passes;
- resume or compensate after injected crash points;
- permit partial progress only with an explicit recoverable group state.

## Scheduling

Trigger from file-count, page-size, stale-link, or index-debt thresholds. Routine
validation is deterministic. Model-assisted taxonomy experiments require a
preview and adoption evidence.

## Evidence

Record before and after trees, fingerprints, link check, index check, operation
events, and any compensation.
