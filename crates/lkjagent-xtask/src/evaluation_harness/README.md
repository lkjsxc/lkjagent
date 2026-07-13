# Evaluation Harness Source

## Purpose

Implement the confined public production evaluation runner, baseline evidence
validator, bounded captures, and mechanical false-positive checks.

## Table of Contents

- [mod.rs](mod.rs): campaign CLI, copied-binary runner, and derived output.
- [clock.rs](clock.rs): monotonic deadlines, bounded capture, and process groups.
- [evidence.rs](evidence.rs): versioned baseline and negative-fixture validation.
- [pty.rs](pty.rs): bounded campaign execution and sanitized evidence writing.
- [pty-cast.rs](pty_cast.rs): bounded temporary cast parsing into measured counts.
- [semantic/mod.rs](semantic/mod.rs): scenario-specific native fact evaluators.
- [scenario.rs](scenario.rs): five tracked aliases, hashed five-turn schedules,
  hashed seed bytes, and shell-free endpoint files. Parser tests reject changed
  owner text and seed hashes; aliases never accept arbitrary paths or owner text.
- [snapshot.rs](snapshot.rs): confined roots, manifests, diffs, and stable SQLite facts.
