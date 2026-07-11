# Evaluation Harness Source

## Purpose

Implement deterministic scenario, clock, fault, snapshot, manifest, PTY, and
false-positive checks for evaluation gates.

## Table of Contents

- [mod.rs](mod.rs): composition, SHA-256, seed checks, benchmark, and smoke replay.
- [clock.rs](clock.rs): monotonic fake clock and ordered fault injector.
- [evidence.rs](evidence.rs): raw fact validation and negative fixtures.
- [pty.rs](pty.rs): raw PTY recording validation and replay binding.
- [scenario.rs](scenario.rs): anchored scenario and seed validation.
- [snapshot.rs](snapshot.rs): SQLite Online Backup and raw manifests.
