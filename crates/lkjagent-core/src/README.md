# Source

## Purpose

Map lkjagent-core source modules.

## Table of Contents

- [lib.rs](lib.rs): public module exports.
- [model.rs](model.rs): task, step, attempt, check, and command data.
- [parse.rs](parse.rs): envelope and plan-line parser.
- [render.rs](render.rs): prompt renderer, budgets, and fingerprints.
- [engine.rs](engine.rs): public next work and turn application seam.
- [engine-completion.rs](engine_completion.rs): task closure and event helpers.
- [engine-steps.rs](engine_steps.rs): internal step settlement helpers.
- [plan.rs](plan.rs): materialize validated plan lines into steps.
- [checks.rs](checks.rs): pure check evaluation over supplied facts.
- [words.rs](words.rs): shared word counting.
- [classify.rs](classify.rs): objective classification and starter templates.
