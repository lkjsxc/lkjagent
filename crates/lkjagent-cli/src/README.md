# lkjagent-cli Source

## Purpose

This directory holds CLI argument dispatch and one module per command.

## Table of Contents

- [args.rs](args.rs): argument parser for the documented commands.
- [accounting.rs](accounting.rs): shared context and token accounting display.
- [config.rs](config.rs): runtime config loading and first-start default writing.
- [config/json.rs](config/json.rs): JSON config parsing and rendering.
- [console.rs](console.rs): interactive owner console input loop.
- [console/display.rs](console/display.rs): display-width wrapping for terminal text.
- [console/event_view.rs](console/event_view.rs): transcript and last-output formatting.
- [console/input.rs](console/input.rs): line and terminal input adapters.
- [console/render.rs](console/render.rs): terminal console screen rendering.
- [console/style.rs](console/style.rs): ANSI terminal styling for the console.
- [console/terminal_input.rs](console/terminal_input.rs): terminal character input reader.
- [env_file.rs](env_file.rs): optional .env loading for the binary entrypoint.
- [error.rs](error.rs): CLI error type and exit-code mapping.
- [lib.rs](lib.rs): command dispatcher used by tests and main.
- [log.rs](log.rs): transcript log rendering.
- [main.rs](main.rs): binary entrypoint.
- [memory.rs](memory.rs): memory search command.
- [paths.rs](paths.rs): workspace and source path resolution.
- [run.rs](run.rs): daemon startup command.
- [send.rs](send.rs): queue append command.
- [status.rs](status.rs): daemon and store status command.
- [store.rs](store.rs): store path and connection helpers.
