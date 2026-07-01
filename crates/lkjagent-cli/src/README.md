# lkjagent-cli Source

## Purpose

This directory holds CLI argument dispatch and one module per command.

## Table of Contents

- [args.rs](args.rs): argument parser for the documented commands.
- [args_help.rs](args_help.rs): usage text for the binary entrypoint.
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
- [graph.rs](graph.rs): active graph case rendering.
- [model_log.rs](model_log.rs): current model handoff log command.
- [lib.rs](lib.rs): command dispatcher used by tests and main.
- [log.rs](log.rs): transcript log rendering.
- [main.rs](main.rs): binary entrypoint.
- [memory.rs](memory.rs): memory search command.
- [paths.rs](paths.rs): workspace and source path resolution.
- [run.rs](run.rs): daemon startup command.
- [send.rs](send.rs): queue append command.
- [status.rs](status.rs): daemon and store status command.
- [store.rs](store.rs): store path and connection helpers.
- [args_catalog.rs](args_catalog.rs): args catalog source module.
- [args_log.rs](args_log.rs): args log source module.
- [args_model_log.rs](args_model_log.rs): args model log source module.
- [args_personal.rs](args_personal.rs): args personal source module.
- [args_queue.rs](args_queue.rs): args queue source module.
- [args_task.rs](args_task.rs): args task source module.
- [config/](config/README.md): config helper modules.
- [console/](console/README.md): console helper modules.
- [model_log_export.rs](model_log_export.rs): model log export source module.
- [personal.rs](personal.rs): personal source module.
- [personal_projection.rs](personal_projection.rs): personal projection source module.
- [personal_projection_text.rs](personal_projection_text.rs): personal projection text source module.
- [queue.rs](queue.rs): queue source module.
- [status_artifact.rs](status_artifact.rs): artifact status projection source module.
- [status_context.rs](status_context.rs): status context source module.
- [status_deck.rs](status_deck.rs): status deck source module.
- [status_facts.rs](status_facts.rs): status facts source module.
- [task.rs](task.rs): task source module.
