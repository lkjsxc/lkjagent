# Status And Console

## Purpose

Define status output, matter display, event log, console, workbench, and proof
visibility that make daemon progress observable.

## Status Shape

`lkjagent status` prints stable fields derived from the store:

```text
daemon: working | idle | waiting | stopped
matter: daily-capture open title="July notes" budget 47/200
decision: matter-daily-capture-decision-0007 model.call status=pending ctx=9ac4 tools=aa12
last: record_written path=workspace/records/life/journal/rec_20260707_note.md
question: none
queue: 0 pending
tokens: input_uncached=804 input_cached=1200 input_total=2004 output=196 cache=known
lease: active owner=pid:123 heartbeat=unix:1780000000
state: active=4 conflicts=0
admissions: 3 observations: 2 exchanges: 1 artifacts: 10 records: 4
```

Every field is available with the daemon stopped. Unknown token counts are
printed as unknown rather than guessed.

## Matter Display

`matter show` renders one matter as a bounded proof trace:

- lifecycle: title, state, owner turns, waiting question, and close report;
- state summary: active state cells, blockers, conflicts, and relation edges;
- decision summary: latest runtime decisions with operation, status, context
  fingerprint, and tool-view fingerprint;
- workspace refs: records, artifacts, indexes, aliases, and fingerprints;
- proof refs: prompt-frame count, check totals, artifact fingerprints, and
  exchange refs.

Plan-family rows may appear as bridge evidence, never as the only runtime
authority.

## Event Log

`log` prints durable events: owner turns, questions, answers, matter routing,
record writes, decision selection, admissions, observations, check results,
context conflicts, completion, blocks, and notices. It does not print full model
requests or long bodies; exchange and workspace refs point to files.

## Owner Console

`lkjagent console` is a normal-screen command loop. It never switches to an
alternate screen, so ordinary terminal scrollback, tmux copy mode, and screen
scrollback continue to work. Plain text lines enqueue owner turns. Slash
commands read rows, enqueue intent, or print local help: `/help`, `/status`,
`/watch`, `/log`, `/queue`, `/matter`, `/record`, `/send TEXT`, `/new TEXT`,
and `/quit`. The console flushes its banner and every non-empty reply before
reading the next line. It opens short store operations per line and owns no
daemon state. Exiting it does not stop the daemon.

## Watch Console

`watch` is a bounded one-shot terminal snapshot over the same store rows:

- status section: daemon, queue, token, lease, state, decision, and record lines;
- recent events section: the latest eight bounded event rows;
- matter trace section: the active matter, latest matter, or `matter: none`;
- proof rows section: prompt-frame, check, artifact, exchange, and record counts;
- footer hint: rerun `watch` to refresh or use `log --follow` to stream.

The renderer is line-oriented and bounded. It never owns facts that are absent
from the store or workspace.

## Workbench

`workbench` opens a terminal operator console when stdin and stdout are TTYs.
The console uses durable transcript rows, a grapheme-aware composer, and panes
for transcript, matter, tool, graph, workspace, proof, queue, and log evidence.
It renders both owner and lkjagent messages from durable rows. Non-TTY runs keep
the line-oriented append or pane fallback.

## Proof Visibility

Proof bundles are produced by xtask. A bundle summarizes matters, state cells,
edges, decisions, prompt frames, tool views, admissions, observations, context
conflicts, contaminated suppressions, checks, artifact fingerprints, exchanges,
token usage, workspace trees, and warnings. It does not copy SQLite files or
large model bodies.
