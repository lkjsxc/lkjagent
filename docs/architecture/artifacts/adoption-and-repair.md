# Adoption And Repair

## Purpose

Define the required flow when an existing artifact root is found.

## Contract

Adoption attaches the root to the active case, runs structure and readiness
audits, records gaps, and enters repair when content is missing or weak.
Adoption never grants completion by itself.

## Inputs

The flow reads known roots, manifests, README titles, semantic section roles,
owner objective, audit results, and content readiness results.

## Output

The output is adopted-ready, adopted-needs-repair, duplicate-rejected, or no
matching root. Needs-repair output names exact paths and the admitted next
repair action.

## Invariants

- Equivalent roots are repaired, not duplicated.
- Weak adopted roots remain open until audit passes.
- Failed README links enter document repair before completion.
- Repair writes are re-audited before close.

## Fixture

`cookbook_missing_readme_links` proves adoption must repair README links before
readiness can pass.

## Verification

Run `cargo test -p lkjagent-tools artifact_next`.

## Status

partially implemented.
