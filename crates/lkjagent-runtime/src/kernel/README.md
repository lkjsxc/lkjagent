# Runtime Kernel Source

## Purpose

This directory owns the pure runtime transition-kernel data model. It contains
types, facts, reducers, resolver rules, completion gates, and write contracts
only. Store, filesystem, endpoint, shell, Docker, and clock effects stay in
daemon, store, tool, or CLI adapters.

## Table of Contents

- [active_mode.rs](active_mode.rs): runtime active modes.
- [adapter.rs](adapter.rs): snapshot adapter input and normalization.
- [adapter_facts.rs](adapter_facts.rs): adapter fact parsing and projection helpers.
- [adapter_fingerprint.rs](adapter_fingerprint.rs): authority and staleness fingerprints.
- [admission.rs](admission.rs): immutable dispatch admission views.
- [admission_decide.rs](admission_decide.rs): admitted-tool decision helpers.
- [authority_ledger.rs](authority_ledger.rs): decision ledger projection helpers.
- [completion.rs](completion.rs): central completion gate reducer.
- [completion_inputs.rs](completion_inputs.rs): prompt-visible completion gate facts.
- [content_atom.rs](content_atom.rs): content atom progress facts.
- [decision.rs](decision.rs): missions, decisions, templates, and invariants.
- [decision_apply.rs](decision_apply.rs): reducer output application helpers.
- [effect.rs](effect.rs): deterministic runtime-owned effects.
- [event.rs](event.rs): runtime event payloads.
- [event_kind.rs](event_kind.rs): closed event catalog.
- [facts.rs](facts.rs): grouped case, graph, queue, evidence, artifact, context,
  observation, provider, and maintenance facts.
- [fault.rs](fault.rs): fault classes and retry keys.
- [manuscript.rs](manuscript.rs): manuscript plan, progress, and shrink facts.
- [manuscript_path.rs](manuscript_path.rs): manuscript path parsing and scene write selection.
- [mission.rs](mission.rs): runtime mission enum and labels.
- [mission_select.rs](mission_select.rs): deterministic mission selection.
- [mod.rs](mod.rs): module exports.
- [next_action_simple.rs](next_action_simple.rs): simple fs.write action helpers.
- [obligation.rs](obligation.rs): obligation enum and priority derivation.
- [obligation_contract.rs](obligation_contract.rs): content write contract selection.
- [obligation_facts.rs](obligation_facts.rs): runtime fact aggregation.
- [obligation_parse.rs](obligation_parse.rs): observation parsing helpers.
- [obligation_paths.rs](obligation_paths.rs): artifact contract path helpers.
- [progress.rs](progress.rs): progress keys and repeated-route guards.
- [provider.rs](provider.rs): provider anomaly event mapping.
- [reduce.rs](reduce.rs): pure decision reducer.
- [render.rs](render.rs): prompt-card data.
- [repeat_guard.rs](repeat_guard.rs): repeat-key helpers.
- [resolver.rs](resolver.rs): resolver plan selection.
- [resolver_label.rs](resolver_label.rs): resolver labels and rule identifiers.
- [resolver_rules.rs](resolver_rules.rs): total mission-rule table.
- [snapshot.rs](snapshot.rs): durable snapshot records and identifiers.
- [write_contract.rs](write_contract.rs): write-contract marker types.

## Ownership

The module does not perform I/O. It may be used by store, daemon, prompt,
dispatch, compaction, maintenance, and CLI adapters after those adapters build
or persist the required records.
