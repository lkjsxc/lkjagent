# Personal Records Work

## Purpose

Track the implementation work for diary, schedule, and TODO records.

## Current State

The contract is documented under
[../../architecture/personal/](../../architecture/personal/README.md) and
[../../architecture/tools/personal-tools.md](../../architecture/tools/personal-tools.md).
Store migrations, typed store APIs, registry entries, create/list/search tools,
update routes, bounded Markdown projections, and thin CLI inspection now have
focused tests.

## Next Slice

No personal-record implementation slice remains open.

## Evidence Required

- Store migration and CRUD tests for each kind.
- FTS search tests with punctuation and non-ASCII text where feasible.
- Registry, parser, dispatch, and prompt-example tests.
- Projection tests proving generated files include record IDs and stay bounded.
- Quiet verify and Docker Compose verification before final claims.

## Status

implemented.
