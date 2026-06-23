# Adoption Controller

## Purpose

This file owns the runtime decision path for adopting an existing artifact root.

## Contract

When an equivalent root exists, the controller attaches it to the active artifact ledger, audits structure and
readiness immediately, records weak paths, and routes to repair when gaps remain. It does not create a duplicate root
unless no equivalent root exists.

## Inputs

- current case and semantic artifact id.
- candidate roots, manifests, README titles, and catalog metadata.
- normalized owner objective and artifact profile.
- structure audit and readiness audit output.

## Outputs

- adopted-ready, adopted-needs-repair, duplicate-rejected, or no-matching-root result.
- artifact ledger update.
- weak path records and next repair action when gaps exist.
- audit evidence only when the root passes.

## Invariants

- Adoption is a ledger transition, not completion.
- Weak adopted roots remain open.
- README and catalog defects enter structure repair before readiness can pass.
- Semantic mismatch blocks adoption unless the owner objective actually matches the root.
- The next action after failed adoption is admitted by runtime authority.

## Failure Cases

- Existing weak content is abandoned and a second scaffold root is created.
- Adoption passes because root names are similar but topics differ.
- A manifest is attached without auditing current file content.
- The next repair action points at a blocked tool.

## Verification

- adoption tests for equivalent roots, mismatched roots, and weak adopted roots.
- completion tests proving adoption alone cannot close owner work.
- `cargo test -p lkjagent-tools --test artifact_next`

## Status

partially implemented.
