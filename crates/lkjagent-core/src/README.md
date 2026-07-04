# Source

## Purpose

Map lkjagent-core source modules.

## Table of Contents

- [lib.rs](lib.rs): public module exports.
- [runtime-state.rs](runtime_state.rs): state keys, cells, snapshots, and
  state-vector fingerprints.
- [runtime-event.rs](runtime_event.rs): events, patches, reducer, and patch
  application.
- [runtime-decision.rs](runtime_decision.rs): runtime decisions, envelopes, and
  tool-set views.
- [runtime-admission.rs](runtime_admission.rs): action admission and workspace
  path policy.
- [runtime-context.rs](runtime_context.rs): context items, contamination, and
  contradiction detection.
- [runtime-completion.rs](runtime_completion.rs): fresh evidence rules for
  closure.
- [runtime-fingerprint.rs](runtime_fingerprint.rs): stable FNV-1a fingerprints
  over canonical JSON.
- [model.rs](model.rs): current task, step, attempt, check, and command data.
- [parse.rs](parse.rs): envelope and plan-line parser.
- [render.rs](render.rs): prompt renderer, budgets, and fingerprints.
- [engine.rs](engine.rs): public next work and turn application seam.
- [engine-completion.rs](engine_completion.rs): task closure and event helpers.
- [engine-steps.rs](engine_steps.rs): internal step settlement helpers.
- [plan.rs](plan.rs): materialize validated plan lines into steps.
- [checks.rs](checks.rs): pure check evaluation over supplied facts.
- [words.rs](words.rs): shared word counting.
- [classify.rs](classify.rs): objective classification and starter templates.
