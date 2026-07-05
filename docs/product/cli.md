# CLI

## Purpose

Define the owner command surface and output discipline.

## Commands

| Command | Behavior |
| --- | --- |
| `lkjagent run` | run the daemon in the foreground |
| `lkjagent send TEXT [--new]` | enqueue an owner message and print its queue id |
| `lkjagent status` | print daemon, task, step, budgets, queue, and tokens |
| `lkjagent console` | read owner input in normal scrollback while the daemon keeps working |
| `lkjagent log [--limit N] [--follow]` | print bounded transcript events, then optionally stream new rows |
| `lkjagent task list` | list tasks with state and summary |
| `lkjagent task show ID` | show plan, diagnoses, checks, and exchange refs |
| `lkjagent queue list` | list owner messages |
| `lkjagent queue show ID` | show one owner message and delivery state |
| `lkjagent context resolve CASE_ID KEY WINNING_ITEM_ID` | record the owner-selected winner for a conflict |
| `lkjagent memory QUERY` | search memory rows |
| `lkjagent watch` | print a refreshable terminal snapshot with status, trace, and proof rows |
| `lkjagent doctor [--json]` | print row-backed health diagnostics without secrets |
| `lkjagent workspace [--json]` | summarize configured workspace paths and indexes |
| `lkjagent record add KIND TITLE [--body TEXT]` | create a generic workspace record and metadata row |
| `lkjagent record list [KIND]` | list current generic records by row metadata |
| `lkjagent record show ID` | print one record row and Markdown body |
| `lkjagent record link ID REF` | add a frontmatter link and refresh fingerprint evidence |
| `lkjagent record archive ID` | move a record under `records/archive` and hide it from normal list |
| `lkjagent today, journal, todo, calendar, finance, project, dev` | friendly record-backed wrappers |
| `lkjagent proof collect [--json]` | collect a bounded proof bundle from rows and refs |
| `lkjagent proof live --minutes N` | run the bounded live proof path when endpoint evidence is available |
| `lkjagent help [group]` | print usage |

## Output Rules

- Commands print line-oriented, machine-readable text by default.
- Read-only commands accept `--json` when their row-backed shape is stable.
- A successful mutating command prints the created id or one concise success
  line.
- A failed command prints the command, reason, and next useful action when one
  exists.
- Quiet gates are owned by xtask, not by the owner CLI.

## Log Follow Contract

`log --follow` first prints the same bounded row-backed output as `log --limit
N`, then polls the store and prints only events with ids greater than the last
printed row. It owns no state outside SQLite and exits naturally when the owner
interrupts the process.

## Data Directory

Commands accept the configured data directory consistently. Status and read-only
inspection commands work while the daemon is stopped because they read the
store directly.

## Acceptance Checks

- `crates/lkjagent-app/src/args.rs` accepts `log --follow` and `log --limit N
  --follow`.
- `crates/lkjagent-app/src/inspect.rs` keeps non-follow output deterministic and
  follows events by monotonically increasing row id.
- CLI tests cover parser shape, console line routing, record commands,
  row-backed log continuation, and watch sections.

## Authority Limits

Personal and proof command groups are allowed only as ledger-backed views or
record-writing helpers. They do not get private state, a graph authority, hidden
tool policy, or a separate completion rule. Mutating commands append events,
write workspace records or artifacts, or enqueue owner text.
