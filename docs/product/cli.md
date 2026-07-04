# CLI

## Purpose

Define the owner command surface and output discipline.

## Commands

| Command | Behavior |
| --- | --- |
| `lkjagent run` | run the daemon in the foreground |
| `lkjagent send TEXT [--new]` | enqueue an owner message and print its queue id |
| `lkjagent status` | print daemon, task, step, budgets, queue, and tokens |
| `lkjagent log [--limit N] [--follow]` | print bounded transcript events, then optionally stream new rows |
| `lkjagent task list` | list tasks with state and summary |
| `lkjagent task show ID` | show plan, diagnoses, checks, and exchange refs |
| `lkjagent queue list` | list owner messages |
| `lkjagent queue show ID` | show one owner message and delivery state |
| `lkjagent context resolve CASE_ID KEY WINNING_ITEM_ID` | record the owner-selected winner for a conflict |
| `lkjagent memory QUERY` | search memory rows |
| `lkjagent watch` | open the terminal console |
| `lkjagent help [group]` | print usage |

## Output Rules

- Commands print line-oriented, machine-readable text by default.
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
- CLI tests cover parser shape and row-backed log continuation.

## Removed Surfaces

There are no graph, personal-records, model-log, audit, or verification command
groups in the owner CLI. The task, queue, memory, log, status, and proof-bundle
surfaces expose the same facts without a second control plane.
