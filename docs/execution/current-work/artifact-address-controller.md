# Artifact Address Controller

## Purpose

This task prevents file paths, document roots, artifact roots, and weak paths
from being confused by tools, prompt examples, recovery, audit, and completion.

## Contract

All doc and artifact tools resolve input through one pure address reducer before
performing filesystem or ledger effects.

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
- path role.
- next valid action.
- exact example that parses and dispatches.
- semantic refusal when input is a file where a directory is required.

## Invariants

- roots for doc.audit, artifact.audit, doc.scaffold, and artifact.apply are directories.
- a root ending in `.md` is refused or migrated before write, never silently created as a directory.
- artifact.next never renders artifact.audit for a Markdown file.
- OS `Not a directory` is not the primary user-facing failure for known file roots.
- completion cannot close an artifact whose only evidence is a structure-only or owner-term-only page.

## Verification

- focused reducer tests.
- tool route tests.
- dispatch-level tests proving rendered examples parse, validate, and dispatch.
- current-model-run regression fixture.
- Docker Compose verify.

## Status

open
