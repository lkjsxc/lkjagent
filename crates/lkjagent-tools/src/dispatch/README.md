# lkjagent-tools Dispatch

## Purpose

This directory splits dispatcher state, validation, and route helpers.

## Table of Contents

- [authority_refusal.rs](authority_refusal.rs): immutable admission-view refusal rendering.
- [effective_refusal.rs](effective_refusal.rs): active effective-policy refusal rendering.
- [fs_tools.rs](fs_tools.rs): fs and shell dispatch helpers.
- [examples.rs](examples.rs): generated copyable valid action examples.
- [fs_extra_tools.rs](fs_extra_tools.rs): list, search, stat, mkdir, and batch-write routing.
- [fs_more_tools.rs](fs_more_tools.rs): multi-read, patch, and tree routing.
- [graph_inspect_tools.rs](graph_inspect_tools.rs): graph next, audit, and recovery routing.
- [graph_note_tools.rs](graph_note_tools.rs): graph note kind validation and alias normalization.
- [graph_tools.rs](graph_tools.rs): graph plan, transition, state, and context routing.
- [graph_evidence_tools.rs](graph_evidence_tools.rs): graph evidence and compaction routing.
- [guards.rs](guards.rs): task-shape write fences.
- [join.rs](join.rs): shared compact list renderer.
- [memory_tools.rs](memory_tools.rs): memory dispatch helpers.
- [normalize.rs](normalize.rs): safe parameter drift normalization before validation.
- [params.rs](params.rs): validated parameter access and parsing.
- [queue_tools.rs](queue_tools.rs): queue dispatch helpers.
- [refusal.rs](refusal.rs): graph policy and repeat refusal guidance.
- [routes.rs](routes.rs): tool-name routing table.
- [routes_artifact.rs](routes_artifact.rs): artifact tool routing.
- [routes_doc.rs](routes_doc.rs): document scaffold and audit routing.
- [routes_verify.rs](routes_verify.rs): direct verification gate routing.
- [routes_workspace.rs](routes_workspace.rs): workspace summary routing.
- [state.rs](state.rs): dispatcher runtime and state structs.
- [validate.rs](validate.rs): registry validation with defaults.
- [batch_write_normalize.rs](batch_write_normalize.rs): batch write normalize source module.
- [completion.rs](completion.rs): completion source module.
- [personal_tools/](personal_tools/README.md): personal tools helper modules.
- [personal_tools.rs](personal_tools.rs): personal tools source module.
