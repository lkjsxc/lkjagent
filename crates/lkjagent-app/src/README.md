# Source

## Purpose

Map the modules compiled into the direct public application.

## Table of Contents

- [main.rs](main.rs): binary entrypoint.
- [lib.rs](lib.rs): direct application exports.
- [args.rs](args.rs): closed public CLI parser.
- [automatic-checks.rs](automatic_checks.rs): native post-edit check reduction.
- [cli.rs](cli.rs): public command execution.
- [clock.rs](clock.rs): UTC timestamps and fixed-offset local dates.
- [config.rs](config.rs): direct configuration consumers.
- [config-registry.rs](config_registry.rs): scalar configuration bounds.
- [journal-apply.rs](journal_apply.rs): journal effect preparation, application, and revision settlement.
- [journal-checks.rs](journal_checks.rs): structural, lineage, token, and placeholder checks.
- [journal-dispatch.rs](journal_dispatch.rs): immutable date context and bounded journal rendering.
- [model-io.rs](model_io.rs): prompt transport and endpoint adapters.
- [public-loop.rs](public_loop.rs): native send, run, status, and doctor loop.
- [tui-model.rs](tui_model.rs): pure TUI state, input events, and intake effects.
- [tui-composer.rs](tui_composer.rs): grapheme-aware composer reduction.
- [tui-wrap.rs](tui_wrap.rs): grapheme and display-width wrapping.
- [tui-viewport.rs](tui_viewport.rs): follow and durable manual anchors.
- [tui-screen.rs](tui_screen.rs): canonical conversation and activity projection.
- [tui-reducer.rs](tui_reducer.rs): pure screen state transitions.
- [workspace-root.rs](workspace_root.rs): lazy separate workspace capability.
