# lkjagent-graph Tests

## Purpose

This directory holds pure tests for source graph validation, routing, slice
rendering, completion gates, and compaction plans.

## Table of Contents

- [classify_counted_deliverables.rs](classify_counted_deliverables.rs): counted deliverable routing fixtures.
- [classify_long_content.rs](classify_long_content.rs): long content deliverable routing fixtures.
- [best_next_transition.rs](best_next_transition.rs): pure transition selector fixtures.
- [graph.rs](graph.rs): validation, routing, rendering, and completion fixtures.
- [graph_context.rs](graph_context.rs): context package pressure and compaction fixtures.
- [recovery_graph.rs](recovery_graph.rs): recovery node affordance fixtures.
- [recovery_topology.rs](recovery_topology.rs): recovery edge and node completeness fixtures.
- [state_tracks.rs](state_tracks.rs): neutral multi-state ranking and rendering fixtures.
