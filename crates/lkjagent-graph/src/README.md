# lkjagent-graph Source

## Purpose

This directory holds the pure graph modules. It performs no IO and has no
workspace dependencies.

## Table of Contents

- [lib.rs](lib.rs): public module exports.
- [model.rs](model.rs): graph, case, evidence, and policy data types.
- [state.rs](state.rs): task graph state, transitions, and compaction plan data.
- [source.rs](source.rs): deterministic source graph definitions.
- [validate.rs](validate.rs): source graph validation.
- [classify.rs](classify.rs): intent routing and initial case construction.
- [classify_signals.rs](classify_signals.rs): pure lexical routing signals.
- [render.rs](render.rs): compact graph slice rendering.
- [transition.rs](transition.rs): legal transition admission.
- [completion.rs](completion.rs): evidence-gated completion checks.
- [compaction.rs](compaction.rs): structured compaction preservation plans.
- [maintenance.rs](maintenance.rs): idle maintenance directive data.
