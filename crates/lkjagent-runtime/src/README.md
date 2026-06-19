# lkjagent-runtime Source

## Purpose

This directory holds the runtime state machine and thin daemon adapters.

## Table of Contents

- [daemon.rs](daemon.rs): public daemon adapter surface.
- [daemon/](daemon/README.md): resident loop, startup, effect, and status helpers.
- [error.rs](error.rs): runtime error type.
- [graph_guard.rs](graph_guard.rs): completion guard graph-prefix guidance.
- [intake.rs](intake.rs): queue delivery helpers.
- [lib.rs](lib.rs): library root.
- [maintenance.rs](maintenance.rs): idle directive rotation, cycle budgets, and distillation prompts.
- [maintenance/](maintenance/README.md): store-facing maintenance adapters.
- [prompt.rs](prompt.rs): deterministic prefix assembly.
- [recovery.rs](recovery.rs): parse and repeat fault recovery helpers.
- [step/](step/README.md): step helper modules.
- [step.rs](step.rs): pure turn transition function.
- [task.rs](task.rs): task, pending action, and stop reason model.
