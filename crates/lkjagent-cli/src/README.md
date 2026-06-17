# lkjagent-cli Source

## Purpose

This directory holds CLI argument dispatch and one module per command.

## Table of Contents

- [args.rs](args.rs): argument parser for the six documented commands.
- [config.rs](config.rs): runtime config loading and first-start default writing.
- [env_file.rs](env_file.rs): optional .env loading for the binary entrypoint.
- [error.rs](error.rs): CLI error type and exit-code mapping.
- [lib.rs](lib.rs): command dispatcher used by tests and main.
- [log.rs](log.rs): transcript log rendering.
- [main.rs](main.rs): binary entrypoint.
- [memory.rs](memory.rs): memory search command.
- [run.rs](run.rs): daemon startup command.
- [send.rs](send.rs): queue append command.
- [skills.rs](skills.rs): skill library listing.
- [status.rs](status.rs): daemon and store status command.
- [store.rs](store.rs): store path and connection helpers.
