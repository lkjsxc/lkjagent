# CLI

## Purpose

Define the owner command surface and output discipline.

## Commands

| Command | Behavior |
| --- | --- |
| `lkjagent run` | run the daemon in the foreground |
| `lkjagent send TEXT [--new]` | enqueue an owner turn and print its row id |
| `lkjagent status` | print daemon, matter, budgets, queue, and tokens |
| `lkjagent console` | read owner input and `/help` while the daemon works |
| `lkjagent workbench` | show progress while accepting owner input |
| `lkjagent log [--limit N] [--follow]` | print bounded transcript events and optionally follow |
| `lkjagent matter list` | list active matters by title, state, and dates |
| `lkjagent matter show REF` | show matter events, decisions, checks, and workspace refs |
| `lkjagent queue list` | list owner-turn delivery and routing state |
| `lkjagent queue show ID` | show one owner turn and routing evidence |
| `lkjagent context resolve MATTER_REF KEY WINNING_ITEM_ID` | record owner conflict resolution |
| `lkjagent memory QUERY` | search durable memory rows |
| `lkjagent watch` | print a refreshable terminal snapshot |
| `lkjagent doctor [--json]` | print row-backed health diagnostics without secrets |
| `lkjagent workspace [--json] [--rebuild]` | summarize workspace paths and rebuild indexes |
| `lkjagent workspace plan-rebalance [--json]` | preview moves and link edits |
| `lkjagent workspace apply-rebalance [--json]` | apply moves with aliases and audit rows |
| `lkjagent workspace validate [--json]` | verify manifest, links, indexes, and paths |
| `lkjagent record add KIND TITLE [--body TEXT]` | create a workspace record and metadata row |
| `lkjagent record list [KIND]` | list current records by row metadata |
| `lkjagent record show ID` | print one record row and Markdown body |
| `lkjagent record link ID REF` | add a document relation and refresh evidence |
| `lkjagent record archive ID` | move a record under archive and keep aliases |
| `lkjagent today, journal, todo, calendar, finance, project, dev` | friendly record-backed wrappers |
| `lkjagent help` | print usage |

## Output Rules

- Commands print line-oriented, machine-readable text by default.
- Read-only commands accept `--json` when their row-backed shape is stable.
- A successful mutating command prints the created id, path, and fingerprint
  when a workspace file changed.
- A failed command prints the command, reason, and next useful action when one
  exists.
- Quiet gates and proof collection are owned by xtask, not by the owner CLI.

## Developer Proof Commands

`cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current`
collects a bounded derived proof bundle. Final acceptance orchestration and raw
evidence validation remain xtask responsibilities.

## Log Follow Contract

`log --follow` first prints the same bounded row-backed output as `log --limit
N`, then polls the store and prints only events with ids greater than the last
printed row. It owns no state outside SQLite and exits naturally when the owner
interrupts the process.

## Data Directory

Commands accept configured data and workspace roots consistently. Status and
read-only inspection work while the daemon is stopped because they read the
store and workspace directly.

## Acceptance Checks

- Argument parsing accepts matter, record, workspace, console, workbench, and
  log-follow shapes.
- Inspect renderers keep non-follow output deterministic and follow events by
  monotonically increasing row id.
- CLI tests cover console and workbench line routing, record commands,
  row-backed log continuation, watch sections, and matter display.

## Authority Limits

Personal, matter, and proof command groups are allowed only as ledger-backed
views or record-writing helpers. They do not get private state, hidden tool
policy, or a separate completion rule. Mutating commands append events, write
workspace records or artifacts, or enqueue owner text.
