# Source

## Purpose

Map lkjagent-core source modules.

## Table of Contents

- [lib.rs](lib.rs): public module exports.
- [runtime-state.rs](runtime_state.rs): state keys, cells, snapshots, and
  state-vector fingerprints.
- [runtime-tool-catalog.rs](runtime_tool_catalog.rs): canonical tool
  descriptors and views.
- [runtime-event.rs](runtime_event.rs): events, patches, reducer, and patch
  application.
- [runtime-decision.rs](runtime_decision.rs): runtime decisions, envelopes, and
  tool-set views.
- [runtime-operation.rs](runtime_operation.rs): selected runtime operation
  payload.
- [runtime-prompt-kernel.rs](runtime_prompt_kernel.rs): structured prompt card
  plan and fingerprints.
- [runtime-candidate.rs](runtime_candidate.rs): selector candidate generation,
  scoring, and edge blocking.
- [runtime-selector.rs](runtime_selector.rs): pure state-vector decision
  selection and fresh-evidence closure checks.
- [runtime-admission.rs](runtime_admission.rs): action admission and workspace
  path policy.
- [runtime-artifact.rs](runtime_artifact.rs): checked artifact units,
  deterministic assembly, fingerprints, and word counts.
- [runtime-recovery.rs](runtime_recovery.rs): typed fault classes and strategy ladders.
- [runtime-context.rs](runtime_context.rs): context items, contamination, and
  contradiction detection.
- [runtime-fingerprint.rs](runtime_fingerprint.rs): stable FNV-1a fingerprints
  over canonical JSON.
- [model.rs](model.rs): current task, step, attempt, check, and command data.
- [parse.rs](parse.rs): envelope and plan-line parser.
- [prompt-policy.rs](prompt_policy.rs): prompt envelopes, policies, and budgets.
- [render.rs](render.rs): prompt renderer and fingerprints.
- [engine.rs](engine.rs): public next work and turn application seam.
- [engine-completion.rs](engine_completion.rs): task closure and event helpers.
- [engine-steps.rs](engine_steps.rs): internal step settlement helpers.
- [plan.rs](plan.rs): materialize validated plan lines into steps.
- [checks.rs](checks.rs): pure check evaluation over supplied facts.
- [workspace-record.rs](workspace_record.rs): generic Markdown record format.
- [classify.rs](classify.rs): objective classification and starter templates.
