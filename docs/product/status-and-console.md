# Status And Console

## Purpose

Define the status output, case display, event log, console, workbench, and proof
visibility that make daemon progress observable.

## Status Shape

`lkjagent status` prints stable fields derived from the store:

```text
daemon: working | idle | waiting | stopped
task: 12 Open Generic budget 47/200
step: 7/16 Write attempt 1/3
last: Notice taskclosed
question: none
queue: 0 pending
tokens: task in=61000 out=18000 cached=44000
lease: active owner=pid:123 heartbeat=unix:1780000000
state: active=4 conflicts=0
decision: case-12-decision-0007 model.call/700 status=pending ctx=9ac4 tools=aa12
admissions: 3 observations: 2 exchanges: 1 artifacts: 10
```

Every field is available with the daemon stopped. Unknown token counts are
printed as unknown rather than guessed.

## Case Display

`task show` renders one task as a bounded proof trace:

- plan-family progress: task state, step order, step kind, attempts, actions,
  and check counts;
- state summary: active state cells and conflict cells for the case;
- decision summary: latest runtime decisions with operation, status, context
  fingerprint, and tool-view fingerprint;
- proof refs: prompt-frame count, check totals, artifact fingerprints, and
  exchange refs.

Plan rows are progress evidence, not the only runtime authority.

## Event Log

`log` prints durable events: owner messages, questions, answers, decision
selection, admissions, observations, check results, context conflicts,
completion, blocks, and notices. It does not print full model requests or prose
bodies; exchange refs point to the files.

## Owner Console

`lkjagent console` is a normal-screen command loop. It never switches to an
alternate screen, so ordinary terminal scrollback, tmux copy mode, and screen
scrollback continue to work. Plain text lines enqueue owner messages. Slash
commands read rows, enqueue intent, or print local help: `/help`, `/status`,
`/watch`, `/log`, `/queue`, `/task`, `/send TEXT`, `/new TEXT`, and `/quit`.
The console flushes its banner and every non-empty reply before reading the next
line. It opens short store operations per line and owns no daemon state. Exiting
it does not stop the daemon.

## Watch Console

`watch` is a bounded one-shot terminal snapshot over the same store rows:

- status section: the same daemon, queue, token, lease, state, and decision
  lines as `status`;
- recent events section: the latest eight bounded event rows;
- task trace section: the same plan-family trace as `task show` for the active
  task, then the latest task when no task is active, or `task: none`;
- proof rows section: prompt-frame, check, artifact, and exchange row counts;
- footer hint: rerun `watch` to refresh or use `log --follow` to stream.

The renderer is line-oriented and bounded. It never owns facts that are absent
from the store.

## Workbench

`workbench` repeatedly renders the same bounded view while keeping owner input
available. Append mode is normal-screen output. Pane mode is an explicit framed
primary-screen renderer, not an alternate-screen owner. Both reuse console
slash-command handlers and exit without stopping the daemon. They do not own
decisions, completion, or private state.

## Proof Visibility

Proof bundles are produced by xtask, not by the owner CLI. A bundle summarizes
cases, state cells,
decisions, prompt frames, tool views, admissions, observations, context
conflicts, contaminated suppressions, checks, artifact fingerprints, exchanges,
token usage, workspace trees, and warnings. It does not copy SQLite files or
large model bodies.
