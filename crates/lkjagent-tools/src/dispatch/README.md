# lkjagent-tools Dispatch

## Purpose

This directory splits dispatcher state, validation, and route helpers.

## Table of Contents

- [fs_tools.rs](fs_tools.rs): fs and shell dispatch helpers.
- [examples.rs](examples.rs): generated copyable valid action examples.
- [fs_extra_tools.rs](fs_extra_tools.rs): list, search, stat, mkdir, and batch-write routing.
- [fs_more_tools.rs](fs_more_tools.rs): multi-read, patch, and tree routing.
- [graph_inspect_tools.rs](graph_inspect_tools.rs): graph next, audit, and recovery routing.
- [graph_tools.rs](graph_tools.rs): graph plan, transition, note, context, and evidence routing.
- [guards.rs](guards.rs): task-shape write fences.
- [memory_tools.rs](memory_tools.rs): memory dispatch helpers.
- [normalize.rs](normalize.rs): safe parameter drift normalization before validation.
- [params.rs](params.rs): validated parameter access and parsing.
- [queue_tools.rs](queue_tools.rs): queue dispatch helpers.
- [refusal.rs](refusal.rs): graph policy and repeat refusal guidance.
- [routes.rs](routes.rs): tool-name routing table.
- [routes_doc.rs](routes_doc.rs): document scaffold and audit routing.
- [routes_verify.rs](routes_verify.rs): direct verification gate routing.
- [routes_workspace.rs](routes_workspace.rs): workspace summary routing.
- [state.rs](state.rs): dispatcher runtime and state structs.
- [validate.rs](validate.rs): registry validation with defaults.
