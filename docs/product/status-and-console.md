# Status And Console

## Purpose

Define the status output, case display, event log, watch console, and proof
visibility that make daemon progress observable.

## Status Shape

`lkjagent status` prints stable fields derived from the store:

```text
daemon: working | idle | waiting | stopped
case: 12 open "Aurora Ledger..." budget 47/200
decision: d-20260704-0007 model.call ctx=9ac4... tools=fs.read,finish
state: active=case:objective,plan:item-7,completion:check-pending
conflicts: none
queue: 0 pending
tokens: case in=61k out=18k cached=44k
```

Every field is available with the daemon stopped. Unknown token counts are
printed as unknown rather than guessed.

## Case Display

`task show` or the case display renders active state cells, plan-family progress,
current decision fingerprints, context conflicts, suppressed contaminated items,
check results, artifact fingerprints, admissions, observations, and exchange
refs. Plan rows are progress evidence, not the only runtime authority.

## Event Log

`log` prints durable events: owner messages, questions, answers, decision
selection, admissions, observations, check results, context conflicts,
completion, blocks, and notices. It does not print full model requests or prose
bodies; exchange refs point to the files.

## Watch Console

`watch` is a terminal view over the same store rows:

- top deck: owner-visible events and the active case summary;
- middle deck: state vector, current decision, tool view, and conflicts;
- bottom deck: plan-family progress, attempts, budget, queue depth, and token
  totals;
- footer: key hints and last refresh time.

The renderer is width-aware and CJK-safe. It never owns facts that are absent
from the store.

## Proof Visibility

Proof bundles are produced by xtask. A bundle summarizes cases, state cells,
decisions, prompt frames, tool views, admissions, observations, context
conflicts, contaminated suppressions, checks, artifact fingerprints, exchanges,
token usage, workspace trees, and warnings. It does not copy SQLite files or
large model bodies.
