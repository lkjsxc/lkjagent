# Workspace Filesystem Engineer Report

## Scope

Report-only lane for dated diary paths, daily records, lazy READMEs, indexes,
and rebalance. I inspected the required packet files, candidate source files,
related workspace docs/tests, and ran focused gates. I did not edit product
docs or source.

## Current Facts

- `crates/lkjagent-core/src/workspace_record.rs` exists and delegates canonical
  paths to `workspace_record_paths`; it also renders/parses record frontmatter,
  computes fingerprints, assigns family default states, and emits state keys.
- `crates/lkjagent-core/src/workspace_record_paths.rs` implements:
  - journal/today: `records/life/journal/YYYY/MM/DD/entry.md`;
  - todo: `records/life/todo/<state>/<id>.md`;
  - calendar: `records/life/calendar/YYYY/MM/DD/<id>.md`;
  - finance: `records/life/finance/YYYY/MM/<id>.md`;
  - project/development: slugged work paths;
  - unknown kinds: `records/knowledge/notes/<kind>/<id>.md`.
- Path helpers reject unsafe `kind` and `id` segments; invalid state falls back
  to `open`.
- `crates/lkjagent-app/src/daemon_owner_routes.rs` writes every owner turn to a
  transcript or inbox trace before direct record handling marks the queue row
  recorded.
- `crates/lkjagent-core/src/owner_record.rs` transforms daemon direct-record
  bodies by family. Journal text becomes `Summary` plus `Reflection`, TODOs
  become `Action item`, finance becomes `Finance note`, and explicit verbatim
  storage is preserved under `Verbatim`.
- CLI friendly wrappers in `crates/lkjagent-app/src/record_args.rs` write the
  wrapper text as both title and body; this is covered by existing tests and is
  different from daemon direct-record transformation.
- `crates/lkjagent-app/src/record_files.rs` writes the canonical record file,
  refreshes path READMEs, upserts `workspace_records` and history, upserts state
  cells/edges, and rebuilds indexes before reporting path/fingerprint/index.
- `crates/lkjagent-app/src/workspace_scaffold.rs` is mostly lazy for specific
  writes via `ensure_for_path` and `refresh_for_path`, but CLI startup calls
  `ensure_root`, which creates broad top-level directories and READMEs.
- `crates/lkjagent-app/src/workspace_index.rs` rebuilds six fixed indexes:
  `today`, `agenda`, `open-todos`, `active-projects`, `proof-runs`, and
  `experiments`. It records artifact rows with input record ids and
  `stale_reason: null`.
- No finance/budget index is currently generated, despite docs and packet
  acceptance requiring a finance index.
- `crates/lkjagent-app/src/workspace_rebalance.rs` plans canonical moves from
  existing `workspace_records`, validates `RebalanceMove` shape, renames files,
  refreshes READMEs, updates record rows, writes path aliases, writes rebalance
  audit rows, and rebuilds indexes.
- Rebalance does not currently repair Markdown/frontmatter links, validate old
  and new fingerprints, split large files, or implement rollback/compensation.
- Candidate files inspected are under 200 lines. `check-lines` also passed.

## Contradictions

- Packet `05-workspace-os/no-bulk-scaffold.md` says not to generate broad empty
  directory trees. `cli::run` always calls `workspace_scaffold::ensure_root`,
  which creates `inbox`, `records`, `artifacts/transcripts`, `artifacts/proof`,
  `indexes`, `system/manifests`, and READMEs before a specific record/artifact
  exists.
- `docs/workspace/indexes.md` lists `budget-month.md` and says indexes include
  input artifact fingerprints, producing decision id, and check/evidence refs.
  Source currently emits no finance/budget index and records only input record
  ids plus null stale reason.
- `docs/workspace/filesystem-layout.md` says rebalance repairs links when
  possible and rolls back or compensates failed moves. Source only renames,
  updates rows, writes aliases/audits, and rebuilds indexes.
- Packet `05-workspace-os/rebalance.md` requires old/new fingerprint validation
  and state edges. Source updates record rows and writes audit rows, but does
  not validate fingerprint continuity or create explicit rebalance state edges.
- Acceptance says a finance request writes a month-grouped finance file and
  index. Current source writes the month-grouped finance file but no
  finance-specific index.
- Acceptance says workspace files over target token budget are split or
  justified. I found no implementation in workspace rebalance/scaffold/index
  for token-budget splitting or explicit justification.
- Docs say the record body is structured unless verbatim. This is true for
  daemon direct-record owner turns, but CLI friendly wrappers store raw wrapper
  text as body.

## Exact Docs Edits

No product docs were edited in this lane. Required doc edits for consistency:

- `docs/workspace/indexes.md`: either add the implemented finance index name
  after source is changed, or replace `budget-month.md` with the actual generated
  index list. If source is fixed, specify `finance-month.md` or
  `budget-month.md` as generated from `finance` records.
- `docs/workspace/indexes.md`: narrow required metadata to current fields only
  or implement missing metadata first. Current source supports generation time,
  input record ids, and stale reason; it does not support input artifact
  fingerprints, producing decision id, or check refs.
- `docs/workspace/filesystem-layout.md`: if source is not expanded, replace
  "repairs links when possible" and rollback/compensation wording with the
  current move/alias/audit behavior. Preferred source fix is listed below.
- `docs/product/cli.md`: clarify whether friendly wrappers are raw-body helpers
  or must use the same structured body transformer as daemon direct records.
- `docs/current-state.md`: after source changes, update the proven/current gap
  language for finance indexes, lazy root scaffolding, rebalance safety, and
  large-file splitting.

## Exact Source Edits

No product source was edited in this lane. Minimal source edits to satisfy the
packet contract:

- `crates/lkjagent-app/src/workspace_index.rs`: add a finance index spec, e.g.
  `("budget-month", &["finance"][..])` or `("finance-month", &["finance"][..])`;
  add tests that a finance record appears in that index and an artifact row is
  recorded.
- `crates/lkjagent-app/src/workspace_scaffold.rs` and `crates/lkjagent-app/src/cli.rs`:
  avoid unconditional broad `ensure_root` during every CLI invocation. Replace
  with root-only creation plus per-command `ensure_for_path`/`refresh_for_path`
  where real content is written, or reduce `ensure_root` to root README only.
- `crates/lkjagent-app/src/workspace_rebalance.rs`: before rename, read old text
  and fingerprint; after rename, read new text and assert same fingerprint.
  Record fingerprint validation in audit metadata if schema supports it.
- `crates/lkjagent-app/src/workspace_rebalance.rs`: add link-repair handling for
  moved record paths in record frontmatter/link fields or explicitly record an
  audit validation warning when links are not repairable.
- `crates/lkjagent-app/src/workspace_rebalance.rs`: wrap each move in a
  compensation path. If row update/audit/index rebuild fails after rename,
  attempt to move the file back or write an audit warning.
- `crates/lkjagent-core/src/owner_record.rs` or
  `crates/lkjagent-app/src/record_args.rs`: decide whether CLI wrappers should
  call the same family body transformer as daemon routes. If yes, expose a pure
  body builder and update wrapper tests.
- Large file splitting likely belongs in a new small helper or in rebalance, but
  should stay under 200 lines per file and be driven by tests first.

## Tests To Add Or Update

- Add/extend app tests for a finance owner turn and CLI finance wrapper proving:
  month path, finance index content, and finance index artifact row.
- Add a scaffold regression proving `send` or `status` does not create broad
  placeholder directories beyond required ancestors.
- Add rebalance tests for fingerprint continuity, alias resolution, audit row
  validation details, and a failed post-rename step compensation/audit warning.
- Add a rebalance/link test with a moved record referenced by another record
  link; prove link repair or explicit validation warning.
- Add a large workspace-record file test proving split behavior or a durable
  justification record.
- Add a CLI wrapper/body test once the wrapper body policy is decided.

## Commands Run

- `cargo test -p lkjagent-core --test workspace_record --test owner_turn`
  passed: 7 workspace_record tests and 3 owner_turn tests.
- `cargo test -p lkjagent-app --test owner_turn_records --test record_wrappers --test workspace_rebalance --test workspace_evidence --test diagnostics`
  passed: 2 owner_turn_records, 2 record_wrappers, 1 workspace_rebalance,
  3 workspace_evidence, and 1 diagnostics test.
- `cargo run -p lkjagent-xtask -- check-lines` passed.

## Gates To Run Before Claiming Completion

- `cargo test -p lkjagent-core --test workspace_record --test owner_turn`
- `cargo test -p lkjagent-app --test owner_turn_records --test record_wrappers --test workspace_rebalance --test workspace_evidence --test diagnostics`
- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify`

## Risks

- Root scaffolding may keep frustrating the owner by creating empty-looking
  placeholder trees despite lazy path README behavior for real writes.
- Missing finance index leaves one workspace acceptance item only partially met.
- Rebalance can leave files moved without full link repair or rollback evidence
  if a later step fails.
- Docs currently overclaim index metadata and rebalance safety compared with
  source.
- CLI wrapper raw body behavior can be mistaken for daemon direct-record body
  behavior unless docs or source unify the policy.

## Acceptance Items Affected

- Diary dated path: implemented and tested.
- Diary body not equal to command unless verbatim: implemented/tested for daemon
  direct records, not for CLI wrappers.
- TODO state-grouped path and index: implemented and tested.
- Calendar dated path and agenda index: path implemented; agenda index exists;
  add a focused content/assertion test.
- Finance month-grouped path and index: path implemented/tested; finance index
  missing.
- Artifact file/row/check before success: outside this lane, referenced by
  existing docs/tests.
- Large workspace file split/justification: not implemented in inspected files.
- Generic record rebalance with aliases/audit rows: implemented and tested for
  TODO canonical move; journal-specific and deeper safety tests still needed.
