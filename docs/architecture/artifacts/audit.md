# Audit

## Purpose

Define semantic artifact audit checks.

## Checks

Audit verifies:

- root exists.
- README exists and links semantic children.
- manifest exists.
- required nodes exist.
- duplicate semantic roles are absent.
- content-bearing files are present.
- artifact kind matches the owner request.

## Refusals

Audit rejects empty roots, generic project-doc trees for story requests,
cookbooks without recipe sections, stories without chapter content, and
scaffold-only leaves.

## Evidence

A passing audit creates verification evidence and document-structure evidence.
A failed audit names exact missing files or roles.

## Status

partially implemented through `doc.audit`, `artifact.audit`, kind mismatch
checks, and content audit checks.
