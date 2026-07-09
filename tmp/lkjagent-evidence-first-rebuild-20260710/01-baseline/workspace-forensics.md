# Workspace Forensics

## Current Visible Data

The supplied data/workspace contains generic navigation files and three owner
transcripts, but no semantic records, indexes with user content, or workspace
record rows.

## Split Persistence

Direct records, template journal writes, exploratory filesystem writes, and
native workspace text effects use different code paths. They do not share one
atomicity, idempotency, indexing, artifact, or recovery contract.

## Diary Failure

A short Japanese diary command is classified as a deterministic record. The
body becomes canned English text saying details were not provided. Older
evidence shows command text stored under Unix-derived names. Current date paths
improve naming but do not provide autonomous diary composition.

## Retrieval Failure

Workspace context loads a handful of recent metadata rows and index metadata.
It does not search relevant record bodies, external edits, older project
records, or repository evidence before ranking.

## Filesystem Tool Failure

Relative workspace roots are canonicalized for access but later stripped using
the original relative root. The live software profile produced prefix-not-found
errors through dedicated tree and list tools.

## Scaffold Failure

CLI startup eagerly creates generic directories and READMEs. This is the exact
empty-shell behavior the owner rejects. Navigation must be created only beside
real content.
