# Store And Workspace Engineer

## Objective

Create one atomic, searchable, owner-visible storage path.

## Work

- native store schema and transactional APIs;
- effect journal, idempotency, conversation sequence;
- separate data and workspace roots;
- strict flat configuration registry;
- WorkspaceService with canonical paths and atomic files;
- semantic records, dates, diary merge, projects, search, indexes;
- external edit scan, validation, rebalance, fresh-store import.

## Tests

Relative roots, Japanese dates, 512-token bounds, retry duplication, crash at
each write phase, same-day diary update, body retrieval, external edits, index
predicates, archive and rebalance compensation.

## Output

Document transaction boundaries and recovery before source. Do not retain direct
filesystem bypasses or eager generic scaffold creation.
