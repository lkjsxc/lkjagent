# Source

## Purpose

Map the modules compiled into the direct public application.

## Table of Contents

- [main.rs](main.rs): binary entrypoint.
- [lib.rs](lib.rs): direct application exports.
- [args.rs](args.rs): closed public CLI parser.
- [automatic-checks.rs](automatic_checks.rs): native post-edit check reduction.
- [cli.rs](cli.rs): public command execution.
- [clock.rs](clock.rs): UTC timestamps.
- [config.rs](config.rs): direct configuration consumers.
- [config-registry.rs](config_registry.rs): scalar configuration bounds.
- [model-io.rs](model_io.rs): prompt transport and endpoint adapters.
- [public-loop.rs](public_loop.rs): native send, run, status, and doctor loop.
- [workspace-root.rs](workspace_root.rs): lazy separate workspace capability.
