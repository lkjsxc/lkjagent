# Current State

## Purpose

Keep an honest ledger that separates the product contract, behavior proven in
this checkout, and open implementation gaps.

## Contract Target

lkjagent is a single-owner, single-daemon, local-LLM plan engine. Owner messages
become durable queue rows, tasks, ordered steps, bounded model asks,
deterministic effects, measured checks, exchange logs, memory rows, token usage,
and events. The plan ledger is the only control plane, and completion is
computed by engine checks rather than by model claims.

The chosen protocol target has five model envelopes. Explore returns only
`<action>...</action>`; explore completion is the `finish` action with a
`summary` parameter. Ask steps return `<message>...</message>` and park the task
as `waiting`. Explore output never asks the owner directly.

## Proven In This Session

The app no longer reads `app.active-snapshot` as runtime authority. Focused
resume tests prove that config snapshots are ignored when rows are absent and
that normalized task and step rows win over stale config. The daemon hydrates
open and waiting tasks from normalized rows, commits turn state through rows,
and records waiting answers as rows before continuing.

The parser now rejects explore `<finish>` and `<ask>` envelopes, leading or
trailing prose, duplicate action parameters, unknown action parameters, and
unknown tools. Prompt rendering tells explore steps to finish with the `finish`
action inside `<action>`.

Queue rows persist `force_new`, send uses it, and daemon intake can select a
forced-new row without treating it as an answer. Status, task, queue, log,
memory, and watch surfaces read rows instead of a config snapshot. The bounded
explore dispatcher runs the documented filesystem, shell, memory, plan-note,
and finish actions, stores latest observations in step inputs, and persists
`memory.save` rows.

Endpoint calls produce exchange files and structured completion records with
usage, cache metrics, provider anomalies, closure mode, timing, generated
exchange refs, and nullable token usage rows. Check results use the active step
id, store check parameters, and keep numeric measurements as scalar values.
README coverage requires links to children, and link checks normalize `./`,
`../`, anchors, and directory README inference. Deterministic effect failures
commit an `effect_error` attempt and notice without marking the step done.

During this implementation pass, `cargo test -q`, `check-docs`, `check-lines`,
`quiet verify`, and `docker compose run --rm verify` passed after the row-first,
parser, CLI, explore, exchange-log, token-usage, check-measurement, memory
persistence, docs-link, and effect-error settlement changes.

## Implemented Code

`lkjagent-core` owns the pure plan engine, parser, renderer, checks, word
counting, classifier, templates, docs-link helpers, and recovery helpers.
`lkjagent-store` owns the plan-store schema, row hydration, queue access, and
atomic turn state commits. `lkjagent-effects` owns filesystem, shell, check
gathering, observations, and exchange log file helpers. `lkjagent-app` owns the
daemon interpreter, row-backed CLI renderers, endpoint adapter, waiting answer
routing, effect-error settlement, and bounded explore dispatcher. `lkjagent-llm`
owns the endpoint wire client. `lkjagent-xtask` owns repository gates, structure
audit, deterministic replay, benchmark commands, and proof bundle collection.

## Open Implementation Gaps

No executable implementation gap is known in the current checkout. Future retry
policies may choose richer typed effect-result and judged-check records, but the
current behavior has deterministic tests and passing gates.

## Historical Evidence

Checked-in logs under `tmp/final-20260703T061546Z/` and related `tmp/` proof
folders record previous successful gates and live proof artifacts. They are
historical evidence only. They do not prove that the current checkout passes a
gate unless that gate is rerun now.

## Next Executable Step

No executable implementation task is known. If new owner requirements arrive,
start by updating this ledger with the observed gap and add a focused test.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
