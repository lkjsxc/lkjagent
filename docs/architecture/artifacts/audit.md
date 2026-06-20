# Audit

## Purpose

Define semantic artifact audit checks.

## Checks

Audit verifies:

- root exists.
- README exists and links semantic children.
- manifest exists.
- every directory has one README.
- required nodes exist.
- duplicate semantic roles are absent.
- sequence-only part files are absent unless requested.
- content-bearing files are present.
- scaffold-only phrases are absent.
- artifact kind matches the owner request.

## Refusals

Audit rejects empty roots, generic project-doc trees for story requests,
cookbooks without recipe sections, stories without chapter content, and
scaffold-only leaves.

## Evidence

A passing audit creates verification evidence and document-structure evidence.
A failed audit names exact missing files or roles.

Content readiness is specified in [content-readiness.md](content-readiness.md).

## Status

partially implemented through `doc.audit`, `artifact.audit`, kind mismatch
checks, and content audit checks.
