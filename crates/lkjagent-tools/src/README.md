# lkjagent-tools Source

## Purpose

This directory holds tool dispatcher, adapter, and observation code.

## Table of Contents

- [control.rs](control.rs): agent.done and agent.ask state transitions.
- [dispatch/](dispatch/README.md): dispatch helper modules.
- [dispatch.rs](dispatch.rs): registry validation and tool routing.
- [error.rs](error.rs): tool error type.
- [fs.rs](fs.rs): filesystem read, write, and edit adapters.
- [lib.rs](lib.rs): library root.
- [memory.rs](memory.rs): memory save and find adapters.
- [observe.rs](observe.rs): bounded frame construction helpers.
- [queue.rs](queue.rs): queue list and mutation adapters.
- [shell.rs](shell.rs): /bin/sh adapter with timeout handling.
- [skill.rs](skill.rs): skill library load and save adapters.
- [structure.rs](structure.rs): recursive tree completion checks.
