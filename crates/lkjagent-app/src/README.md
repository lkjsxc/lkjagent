# Source

## Purpose

Map lkjagent-app source modules.

## Table of Contents

- [main.rs](main.rs): binary entrypoint.
- [lib.rs](lib.rs): public library entry.
- [admission-bridge.rs](admission_bridge.rs): decision-specific admission rows.
- [artifact-effects.rs](artifact_effects.rs): checked unit assembly and artifact rows.
- [args.rs](args.rs): CLI parser.
- [cli.rs](cli.rs): command execution.
- [clock.rs](clock.rs): timestamp seam for runtime and deterministic tests.
- [config.rs](config.rs): file and environment loading plus direct consumers.
- [config-registry.rs](config_registry.rs): exact scalar types, bounds, and
  cross-key guards.
- [console.rs](console.rs): normal-screen owner command loop.
- [context-admin.rs](context_admin.rs): owner conflict-resolution commands.
- [context-bridge.rs](context_bridge.rs): durable context item prompt projection.
- [context-resolution-bridge.rs](context_resolution_bridge.rs): conflict lineage rows.
- [daemon.rs](daemon.rs): row-backed turn-cycle interpreter and scripted endpoint seam.
- [daemon-intake.rs](daemon_intake.rs): owner-turn intake, direct records, and waiting-answer resume.
- [daemon-lock.rs](daemon_lock.rs): heartbeat config-row daemon lease.
- [effect-error.rs](effect_error.rs): effect failure settlement.
- [exchange-bridge.rs](exchange_bridge.rs): provider exchange row persistence.
- [exchange-record.rs](exchange_record.rs): exchange log file rendering.
- [explore.rs](explore.rs): bounded explore action dispatcher.
- [inspect.rs](inspect.rs): row-backed CLI inspection renderers.
- [lease-status.rs](lease_status.rs): owner-visible lease freshness line.
- [model-call.rs](model_call.rs): endpoint call, exchange log, and usage handling.
- [model-io.rs](model_io.rs): endpoint trait, live adapter, and scripted record.
- [observation-bridge.rs](observation_bridge.rs): effect observation rows.
- [prompt-bridge.rs](prompt_bridge.rs): prompt-frame rows before model calls.
- [record-args.rs](record_args.rs): record subcommand parsing.
- [record-files.rs](record_files.rs): workspace record file commands.
- [record-identity.rs](record_identity.rs): canonical record ids and kind aliases.
- [workspace-scaffold.rs](workspace_scaffold.rs): README and directory scaffold writes.
- [recovery-bridge.rs](recovery_bridge.rs): unfinished decision reuse and
  recovery settlement.
- [runtime-bridge.rs](runtime_bridge.rs): state-ledger decision preparation.
- [runtime-cell.rs](runtime_cell.rs): operation cell projection payloads.
- [runtime-projection.rs](runtime_projection.rs): plan bridge projection events.
- [snapshot-state.rs](snapshot_state.rs): matter snapshot state-cell mirror.
- [lib.rs](lib.rs): public state module for snapshot hydration from normalized rows.
- [status.rs](status.rs): status, matter, and watch rendering helpers.
- [turn-effects.rs](turn_effects.rs): deterministic write, check, and explore effects.
- [workbench.rs](workbench.rs): refreshing normal-screen progress and input loop.
