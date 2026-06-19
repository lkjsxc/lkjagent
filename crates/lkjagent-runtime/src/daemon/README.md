# Daemon Helpers

## Purpose

This directory holds adapter helpers for the foreground daemon.

## Table of Contents

- [runner.rs](runner.rs): resident poll loop and effect interpretation.
- [effects.rs](effects.rs): step effect persistence and tool dispatch.
- [endpoint.rs](endpoint.rs): endpoint completion and oversize handling.
- [idle.rs](idle.rs): automatic idle maintenance cycle opening.
- [skills.rs](skills.rs): guarded task skill auto-loading and scaffolding.
- [status.rs](status.rs): daemon state fields written to the store.
- [startup.rs](startup.rs): seed copying and prefix input loading.
