# Source

## Purpose

Map lkjagent-app source modules.

## Table of Contents

- [main.rs](main.rs): binary entrypoint.
- [lib.rs](lib.rs): public library entry.
- [admission-bridge.rs](admission_bridge.rs): decision-specific admission rows.
- [artifact-effects.rs](artifact_effects.rs): checked unit assembly and manifest rendering.
- [artifact-plan.rs](artifact_plan.rs): exact generated-artifact target and ownership plans.
- [args.rs](args.rs): CLI parser.
- [cli.rs](cli.rs): command execution.
- [clock.rs](clock.rs): timestamp seam for runtime and deterministic tests.
- [config.rs](config.rs): file and environment loading plus direct consumers.
- [config-registry.rs](config_registry.rs): exact scalar types, bounds, and
  cross-key guards.
- [console.rs](console.rs): normal-screen owner command loop.
- [context-bridge.rs](context_bridge.rs): durable context item prompt projection.
- [context-resolution-bridge.rs](context_resolution_bridge.rs): owner conflict commands and lineage rows.
- [daemon.rs](daemon.rs): row-backed turn-cycle interpreter and scripted endpoint seam.
- [daemon-intake.rs](daemon_intake.rs): owner-turn intake, direct records, and waiting-answer resume.
- [daemon-lock.rs](daemon_lock.rs): heartbeat config-row daemon lease.
- [effect-dispatch.rs](effect_dispatch.rs): ordered effect dispatch and compensation.
- [effect-files.rs](effect_files.rs): descriptor-relative atomic artifact target writes.
- [endpoint-recovery.rs](endpoint_recovery.rs): endpoint condition fingerprints and wait release.
- [exchange-bridge.rs](exchange_bridge.rs): provider exchange and prompt-frame persistence.
- [exchange-record.rs](exchange_record.rs): exchange log file rendering.
- [explore.rs](explore.rs): bounded explore action dispatcher.
- [inspect.rs](inspect.rs): row-backed CLI inspection renderers.
- [lease-status.rs](lease_status.rs): lease freshness and token usage lines.
- [model-call.rs](model_call.rs): endpoint call, exchange log, and usage handling.
- [model-io.rs](model_io.rs): endpoint trait, live adapter, and scripted record.
- [observation-bridge.rs](observation_bridge.rs): effect observation rows.
- [record-args.rs](record_args.rs): record subcommand parsing.
- [record-files.rs](record_files.rs): workspace record file commands.
- [record-archive.rs](record_archive.rs): archive settlement and compensation.
- [record-identity.rs](record_identity.rs): canonical record ids and kind aliases.
- [workspace-index.rs](workspace_index.rs): semantic navigation page rebuild.
- [workspace-rebalance.rs](workspace_rebalance.rs): rebalance planning and validation.
- [workspace-rebalance-apply.rs](workspace_rebalance_apply.rs): legacy single-move recovery.
- [workspace-rebalance-group.rs](workspace_rebalance_group.rs): durable grouped rebalance apply.
- [workspace-search.rs](workspace_search.rs): visible Markdown inventory, rebuild, and bounded retrieval.
- [workspace-scaffold.rs](workspace_scaffold.rs): README and directory scaffold writes.
- [workspace-scan.rs](workspace_scan.rs): durable inventory debounce scheduling.
- [progress-bridge.rs](progress_bridge.rs): durable progress vectors and no-progress adaptation.
- [recovery-bridge.rs](recovery_bridge.rs): unfinished decision reuse and
  recovery settlement.
- [runtime-budget.rs](runtime_budget.rs): separate durable case budget measurement and blocking.
- [runtime-bridge.rs](runtime_bridge.rs): decision preparation and effect failure settlement.
- [runtime-cell.rs](runtime_cell.rs): operation cell projection payloads.
- [runtime-projection.rs](runtime_projection.rs): plan bridge projection events.
- [snapshot-state.rs](snapshot_state.rs): matter snapshot mirror and watch view.
- [lib.rs](lib.rs): public state module for snapshot hydration from normalized rows.
- [status.rs](status.rs): status, matter, and watch rendering helpers.
- [turn-effects.rs](turn_effects.rs): deterministic check and evidence effects.
- [workbench.rs](workbench.rs): refreshing normal-screen progress and input loop.
