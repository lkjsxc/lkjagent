# Personal Records Work

## Purpose

Track the implementation work for diary, schedule, and TODO records.

## Current State

The contract is documented under
[../../architecture/personal/](../../architecture/personal/README.md) and
[../../architecture/tools/personal-tools.md](../../architecture/tools/personal-tools.md).
Store migrations and typed store APIs now have focused tests. Tool registry
entries, dispatcher routes, projections, CLI inspection, and final gates remain
open.

## Next Slice

1. Add fixed registry tools and dispatcher routes.
2. Add bounded search and list observations.
3. Add optional Markdown projections from store state.
4. Add CLI inspection only if it stays thin and store-backed.

## Evidence Required

- Store migration and CRUD tests for each kind.
- FTS search tests with punctuation and non-ASCII text where feasible.
- Registry, parser, dispatch, and prompt-example tests.
- Projection tests proving generated files include record IDs and stay bounded.
- Quiet verify and Docker Compose verification before final claims.
