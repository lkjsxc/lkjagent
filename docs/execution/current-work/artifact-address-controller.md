# Artifact Address Controller

## Purpose

This task prevents file paths, document roots, artifact roots, and weak paths
from being confused by tools, prompt examples, recovery, audit, and completion.

## Contract

All doc and artifact tools resolve input through one pure address reducer before
performing filesystem or ledger effects. Graph policy may request an address,
but it does not classify roots or admit a fallback route once runtime authority
has a current decision.

## Inputs

- requested root parameter.
- optional path parameter.
- workspace filesystem kind.
- nearest catalog.toml ancestor.
- artifact ledger root records.
- graph active artifact root.
- current weak-path cursor.

## Outputs

- normalized root directory.
- optional relative weak path.
- path role and detected filesystem kind.
- address status and next valid action.
- exact example that parses, validates, and dispatches to the intended route.
- semantic refusal when input is a file where a directory is required.

## Invariants

- roots for doc.audit, artifact.audit, doc.scaffold, artifact.plan, and
  artifact.apply are directory identities.
- a root ending in `.md` is refused or repaired before write, never silently
  created as a directory or ledger identity.
- artifact.next may accept a Markdown file only as a weak path under a known
  root and never renders artifact.audit for that file.
- old `.md` directories are invalid roots and need adoption or repair output.
- OS `Not a directory` is not the primary user-facing failure for known file roots.
- completion cannot close an artifact whose only evidence is a structure-only or owner-term-only page.

## Verification

- focused reducer table tests for every path classification.
- tool route tests for plan, apply, next, audit, scaffold, and doc audit.
- dispatch-level tests proving rendered examples parse, validate, and dispatch.
- current-model-run regression fixture.
- Docker Compose verify.

## Proven Slice

Focused tests prove current refusals for artifact.plan `.md` roots before
ledger identity, artifact.next on file roots, artifact.audit on file roots,
doc.audit on file roots, artifact.apply `.md` roots, doc.scaffold `.md` roots,
existing `.md` directories, and missing directory roots. Covered refusal
examples now parse, validate, and dispatch to a route. `artifact.next` now
reports candidate actions as facts with `next_decision_required=true` instead
of executable authority. Prior address slice notes record quiet verify and
Docker Compose verify.

## Remaining Proof Gaps

No runtime-authority address proof gaps remain. Old bad `.md` directories,
ledger roots without catalog ancestors, invalid-root durable markers,
completion blocking for adopted invalid roots, and route-level examples are
covered by focused tests and final gates.

## Status

implemented
