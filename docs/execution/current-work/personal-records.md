# Personal Records Work

## Purpose

Track the implementation work for diary, schedule, and TODO records.

## Current State

The contract is documented under
[../../architecture/personal/](../../architecture/personal/README.md) and
[../../architecture/tools/personal-tools.md](../../architecture/tools/personal-tools.md).
Implementation is open until store migrations, tool registry entries, focused
tests, and quiet gates pass.

## Next Slice

1. Add store schema and typed APIs in `lkjagent-store`.
2. Add validation for diary, schedule, TODO, timestamps, statuses, tags, and
   recurrence strings.
3. Add fixed registry tools and dispatcher routes.
4. Add bounded search and list observations.
5. Add optional Markdown projections from store state.
6. Add CLI inspection only if it stays thin and store-backed.

## Evidence Required

- Store migration and CRUD tests for each kind.
- FTS search tests with punctuation and non-ASCII text where feasible.
- Registry, parser, dispatch, and prompt-example tests.
- Projection tests proving generated files include record IDs and stay bounded.
- Quiet verify and Docker Compose verification before final claims.
