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

Static source reading on 2026-07-03 confirmed the repository still has the Rust
workspace split described by the docs: `lkjagent-core`, `lkjagent-store`,
`lkjagent-llm`, `lkjagent-effects`, `lkjagent-app`, and `lkjagent-xtask`.
During this documentation pass, these gates passed: `check-docs`,
`check-lines`, `quiet verify`, and `docker compose run --rm verify`.

## Open Implementation Gaps

The following gaps are based on local source reading, not on failing test output:

- `crates/lkjagent-app/src/state.rs` persists `TaskSnapshot` JSON under
  `app.active-snapshot`, and `daemon.rs` loads it before row intake. Normalized
  SQLite rows are therefore not yet the only runtime truth.
- The store has normalized tables, but no row-first hydrator is wired into the
  app. Step rows do not store `actions_used`, `action_budget`, or `split_used`;
  attempts write an empty `exchange_ref`; check results are inserted with the
  task id in the `step_id` column; token usage has no observed writer.
- `parse.rs` still accepts explore `<finish>` and `<ask>` envelopes. It does
  not yet enforce the target exactness for trailing prose, attributes,
  duplicate action parameters, or unknown action parameters.
- `render.rs` still tells explore steps to return `<finish>summary</finish>` as
  an alternative to `<action>`, so prompt rendering is not aligned with the
  target grammar.
- `send --new` is parsed, but queue rows do not persist `force_new`, and enqueue
  behavior does not use the flag for answer routing.
- `status`, `task show`, and `watch` render from the config snapshot. `log`,
  `task list`, `queue list`, `queue show`, and `memory` still return placeholder
  text rather than row-backed data.
- Endpoint completion returns only content to the interpreter. Durable usage,
  cache metrics, provider anomalies, closure mode, timing, and exchange refs are
  not yet owned by structured rows plus exchange files.
- Explore effects do not yet dispatch the documented ten-tool registry. The app
  currently records a generic `observation=ok` note for explore commands.
- Turn settlement marks write content done in pure state before the effect is
  dispatched. If the effect fails, the returned error prevents the snapshot save
  for that turn, but the engine command stream is not yet a data-rich settlement
  of effect results.

## Historical Evidence

Checked-in logs under `tmp/final-20260703T061546Z/` and related `tmp/` proof
folders record previous successful gates and live proof artifacts. They are
historical evidence only. They do not prove that the current checkout passes a
gate unless that gate is rerun now.

## Next Executable Step

Add focused store hydration tests that prove the app cannot resume from
`app.active-snapshot` when normalized rows are absent or stale, then replace
config-snapshot authority with row-first hydration.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
