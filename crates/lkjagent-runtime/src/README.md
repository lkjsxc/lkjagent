# lkjagent-runtime Source

## Purpose

This directory holds the runtime state machine and thin daemon adapters.

## Table of Contents

- [daemon.rs](daemon.rs): startup, lock, signal, endpoint, and tool adapters.
- [error.rs](error.rs): runtime error type.
- [intake.rs](intake.rs): queue delivery helpers.
- [lib.rs](lib.rs): library root.
- [prompt.rs](prompt.rs): deterministic prefix assembly.
- [recovery.rs](recovery.rs): parse and repeat fault recovery helpers.
- [step/](step/README.md): step helper modules.
- [step.rs](step.rs): pure turn transition function.
- [task.rs](task.rs): task, pending action, and stop reason model.
