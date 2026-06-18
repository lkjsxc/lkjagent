# lkjagent-runtime Tests

## Purpose

This directory holds pure runtime-step and thin daemon adapter tests.

## Table of Contents

- [maintenance.rs](maintenance.rs): idle cycle rotation, preemption, and early-close fixtures.
- [maintenance_authority.rs](maintenance_authority.rs): maintenance tool authority and local remote fixtures.
- [daemon_loop.rs](daemon_loop.rs): resident queue, endpoint, tool, ask, and error fixtures.
- [prompt_daemon.rs](prompt_daemon.rs): prompt, startup, lock, and shutdown fixtures.
- [step.rs](step.rs): task lifecycle, recovery, and compaction fixtures.
- [support/](support/README.md): shared state and store helpers.
