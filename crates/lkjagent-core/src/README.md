# Source

## Purpose

Map the direct-runtime core modules.

## Table of Contents

- [lib.rs](lib.rs): public exports and canonical fingerprints.
- [prompt.rs](prompt.rs): transport prompt value.
- [parse.rs](parse.rs): decision-bound tool-call and final parsing.
- [prompt-policy.rs](prompt_policy.rs): direct output grammar text.
- [runtime-state.rs](runtime_state.rs): state cells and snapshots.
- [runtime-event.rs](runtime_event.rs): direct reducer.
- [runtime-decision.rs](runtime_decision.rs): persisted decisions and envelopes.
- [runtime-operation.rs](runtime_operation.rs): runtime state and selections.
- [runtime-selector.rs](runtime_selector.rs): deterministic direct selection.
- [runtime-prompt-kernel.rs](runtime_prompt_kernel.rs): prompt compiler.
- [runtime-tool-catalog.rs](runtime_tool_catalog.rs): five native descriptors.
- [runtime-tool-call.rs](runtime_tool_call.rs): compact XML grammar.
- [runtime-admission.rs](runtime_admission.rs): action admission.
- [runtime-context.rs](runtime_context.rs): context selection and contamination.
- [runtime-artifact.rs](runtime_artifact.rs): checked artifact units.
- [runtime-recovery.rs](runtime_recovery.rs): fault and recovery ladders.

Artifact and workspace record helpers remain exported separately; this slice did
not claim their deletion.
