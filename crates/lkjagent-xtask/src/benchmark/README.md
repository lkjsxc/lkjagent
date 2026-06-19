# Benchmark Xtask

## Purpose

This directory owns benchmark command parsing, local corpus commands, report
comparison, and real Docker benchmark orchestration.

## Table of Contents

- [args.rs](args.rs): benchmark command parser.
- [docker.rs](docker.rs): docker compose command adapter.
- [meta.rs](meta.rs): run ids, git state, paths, and report writes.
- [mod.rs](mod.rs): command dispatcher.
- [real_run.rs](real_run.rs): real agent benchmark runner.
- [wait.rs](wait.rs): daemon send, status, log, and wait loop.
