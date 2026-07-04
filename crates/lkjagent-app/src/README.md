# Source

## Purpose

Map lkjagent-app source modules.

## Table of Contents

- [main.rs](main.rs): binary entrypoint.
- [lib.rs](lib.rs): public library entry.
- [args.rs](args.rs): CLI parser.
- [cli.rs](cli.rs): command execution.
- [clock.rs](clock.rs): timestamp seam for runtime and deterministic tests.
- [daemon.rs](daemon.rs): row-backed turn-cycle interpreter and scripted endpoint seam.
- [daemon-intake.rs](daemon_intake.rs): queue intake and waiting-answer resume.
- [daemon-lock.rs](daemon_lock.rs): heartbeat config-row daemon lease.
- [effect-error.rs](effect_error.rs): effect failure settlement.
- [endpoint.rs](endpoint.rs): endpoint adapter wrapper.
- [exchange-record.rs](exchange_record.rs): exchange log file rendering.
- [explore.rs](explore.rs): bounded explore action dispatcher.
- [inspect.rs](inspect.rs): row-backed CLI inspection renderers.
- [model-call.rs](model_call.rs): endpoint call, exchange log, and usage handling.
- [model-io.rs](model_io.rs): endpoint trait and scripted completion record.
- [runtime-bridge.rs](runtime_bridge.rs): plan-row projection into state-ledger
  decisions.
- [state.rs](state.rs): active snapshot hydration from normalized rows.
- [status.rs](status.rs): status, task, and watch rendering helpers.
- [turn-effects.rs](turn_effects.rs): deterministic write, check, and explore effects.
