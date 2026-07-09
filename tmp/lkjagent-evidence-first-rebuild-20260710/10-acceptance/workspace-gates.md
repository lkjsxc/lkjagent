# Workspace Gates

## Root

- Data and workspace are separate mounts.
- Relative and absolute configured roots pass list, tree, search, read, write,
  and validation.
- Empty startup creates no placeholder hierarchy.
- Descriptor-relative traversal resists an ancestor symlink swap between check
  and use.

## Records

- Japanese diary composition writes life/journal/local-date/entry.md.
- The diary body contains neither the command nor canned missing-detail text.
- Same-day update preserves existing and new content.
- TODO, calendar, finance, note, and project use semantic dates and states.
- Activity history preserves raw owner turns separately.
- UTC/local midnight, daylight-saving ambiguity, and retry across a date
  boundary preserve the original semantic date.
- Every managed file has a unique path-independent document ID and valid header.
- Every managed Markdown and generated navigation page is at most 512 measured
  tokens with a recorded tokenizer identity.

## Transactions

- Crash injection at every write phase yields exactly one result after restart.
- Queue, document, history, effect journal, state, and file fingerprints agree.
- Immutable revision bytes recover both prior and intended diary content.
- Multi-file artifacts resume and verify all semantic units.

## Retrieval

- Relevant old bodies beat unrelated recent metadata.
- External edits are discovered and fingerprinted.
- Stable-read debounce, periodic reconciliation, move detection, duplicate IDs,
  external deletion tombstones, and valid large source files behave as
  documented.
- Material stale or conflicting sources do not enter normal prompts.

## Maintenance

- Index predicates are correct.
- Navigation pages remain bounded.
- Required navigation debt prevents record success until settled.
- Rebalance preview, apply, link repair, and compensation pass.

## Store

- Foreign-key and integrity checks pass.
- Idempotency, conversation sequence, path, document ID, and effect-outcome
  uniqueness constraints hold.
- Online backup and workspace manifest describe one consistent evidence
  boundary.
- workspace_gate.py recomputes managed headers, journal path/date, fingerprints,
  manifest membership, token ceiling, and zero unsettled navigation debt.
- Fresh-store import activates atomically or rolls back without changing the
  active workspace.
