# Core Integration Tests

## Purpose

Map focused integration targets for the direct runtime core.

## Table of Contents

- [contract-tables.rs](contract_tables.rs): closed direct vocabularies.
- [runtime-reducer-selector.rs](runtime_reducer_selector.rs): direct reducer and selection.
- [direct-action-grammar.rs](direct_action_grammar.rs): compact grammar rejection.
- [tool-call.rs](tool_call.rs): descriptor-bound validation.
- [tool-call-edges.rs](tool_call_edges.rs): final and phase boundaries.
- [prompt-kernel.rs](prompt_kernel.rs): decision-bound prompt compilation.
- [admission.rs](admission.rs): direct admission and effect keys.
- [journal-admission.rs](journal_admission.rs): exact record descriptor and journal-only admission.
- [default-tool-view.rs](default_tool_view.rs): phase-specific native tools.
- Other targets cover runtime state, context, recovery, artifacts, and retained
  workspace helper contracts.
