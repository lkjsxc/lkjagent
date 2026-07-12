# Core Integration Tests

## Purpose

Map focused integration targets for the pure runtime core.

## Table of Contents

- [admission.rs](admission.rs): persisted projections, state views, and effect-key dispatch.
- [contract-tables.rs](contract_tables.rs): closed direct-runtime interface vocabularies and invariants.
- [direct-action-grammar.rs](direct_action_grammar.rs): rejected XML and legacy action forms.
- [tool-call.rs](tool_call.rs): descriptor-bound tool and input validation.
- [tool-call-edges.rs](tool_call_edges.rs): final envelopes, phase binding, and byte bounds.
- [render-tool-cards.rs](render_tool_cards.rs): parser-valid compact examples and leak checks.
- [render.rs](render.rs): decision-bound prompt protocol rendering.
- Other Rust targets cover unrelated compatibility behavior.
