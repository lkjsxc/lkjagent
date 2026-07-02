# Status And Console

## Purpose

Define the status output, task display, event log, watch console, and proof
visibility that make daemon progress observable.

## Status Shape

`lkjagent status` prints stable fields derived from the store:

```text
daemon: working | idle | waiting | stopped
task: 12 open manuscript "Aurora Ledger..." budget 47/200
step: 7/14 write manuscript/chapter-03.md attempt 2/3
last: step_done 6/14 words=1043
question: none
queue: 0 pending
tokens: task in=61k out=18k cached=44k
```

Every field is available with the daemon stopped. Unknown token counts are
printed as unknown rather than guessed.

## Task Display

`task show` renders the plan digest exactly as prompt assembly renders it: one
line per step with state marks, attempt counts, diagnoses, check results, and
exchange-log references. The plan is the progress bar for both owner and model.

## Event Log

`log` prints transcript events: owner messages, questions, answers, step_done,
step_blocked, task_closed, task_blocked, and notices. It does not print full
model requests or prose bodies; exchange refs point to the files.

## Watch Console

`watch` is a terminal view over the same store rows:

- top deck: transcript events and the active task summary;
- bottom deck: plan digest, active step, attempts, budget, queue depth, and
  token totals;
- footer: key hints and last refresh time.

The renderer is width-aware and CJK-safe. It never owns facts that are absent
from the store.

## Proof Visibility

Proof bundles are produced by xtask. A bundle summarizes store rows, check
results, workspace trees with word counts, token usage, attempt outcomes, and
warnings. It does not copy SQLite files or large model bodies.
