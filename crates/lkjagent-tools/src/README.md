# lkjagent-tools Source

## Purpose

This directory holds tool dispatcher, adapter, and observation code.

## Table of Contents

- [benchmark_seed.rs](benchmark_seed.rs): deterministic benchmark corpus scaffold.
- [control/](control/README.md): completion guard helper modules.
- [control.rs](control.rs): agent.done and agent.ask state transitions.
- [count_guard.rs](count_guard.rs): exact and approximate file-count guards.
- [count_guard/](count_guard/README.md): file-count scanning helpers.
- [count_profile_body.rs](count_profile_body.rs): counted scaffold body spines.
- [count_profile_design.rs](count_profile_design.rs): counted scaffold design memo bodies.
- [count_profile.rs](count_profile.rs): objective profile text for counted scaffolds.
- [count_profile_data.rs](count_profile_data.rs): counted scaffold profile labels.
- [count_seed.rs](count_seed.rs): counted document scaffold generator.
- [dispatch/](dispatch/README.md): dispatch helper modules.
- [dispatch.rs](dispatch.rs): registry validation and tool routing.
- [error.rs](error.rs): tool error type.
- [fs.rs](fs.rs): filesystem read, write, and edit adapters.
- [lib.rs](lib.rs): library root.
- [memory.rs](memory.rs): memory save and find adapters.
- [observe.rs](observe.rs): bounded frame construction helpers.
- [queue.rs](queue.rs): queue list and mutation adapters.
- [shell.rs](shell.rs): /bin/sh adapter with timeout handling.
- [skill.rs](skill.rs): source skill library load adapter.
- [structure.rs](structure.rs): recursive tree completion checks.
- [structure_network.rs](structure_network.rs): knowledge network completion checks.
- [structure_seed/](structure_seed/README.md): deterministic docs scaffold profiles.
- [structure_seed.rs](structure_seed.rs): scaffold profile entrypoint.
