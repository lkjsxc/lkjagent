# lkjagent-graph Source

## Purpose

This directory holds the pure graph modules. It performs no IO and has no
workspace dependencies.

## Table of Contents

- [lib.rs](lib.rs): public module exports.
- [case_context.rs](case_context.rs): context package binding and pressure state.
- [case_decision.rs](case_decision.rs): recorded case decision data.
- [case_document.rs](case_document.rs): document topology state.
- [case_evidence.rs](case_evidence.rs): evidence records and requirements.
- [case_objective.rs](case_objective.rs): raw and normalized objective state.
- [case_plan.rs](case_plan.rs): structured plan step data.
- [case_recovery.rs](case_recovery.rs): typed fault and recovery ladder state.
- [classify.rs](classify.rs): intent routing and initial case construction.
- [classify_signals.rs](classify_signals.rs): pure lexical routing signals.
- [compaction.rs](compaction.rs): structured compaction preservation plans.
- [completion.rs](completion.rs): evidence-gated completion checks.
- [context_selection.rs](context_selection.rs): graph-aware package selection.
- [guards.rs](guards.rs): transition guard evaluation.
- [maintenance.rs](maintenance.rs): idle maintenance directive data.
- [model.rs](model.rs): graph node, edge, package, and policy data types.
- [node_policy.rs](node_policy.rs): node and edge affordance policy data.
- [render.rs](render.rs): compact graph slice rendering.
- [render_guidance.rs](render_guidance.rs): bounded guidance line helpers.
- [render_tools.rs](render_tools.rs): rendered allowed and blocked tool helpers.
- [state.rs](state.rs): task graph state, transitions, and compaction plan data.
- [source.rs](source.rs): deterministic source graph definitions.
- [source_code.rs](source_code.rs): code-change node definitions.
- [source_compaction.rs](source_compaction.rs): compaction node definitions.
- [source_completion.rs](source_completion.rs): completion node definitions.
- [source_context.rs](source_context.rs): context node definitions.
- [source_document.rs](source_document.rs): document node definitions.
- [source_edges.rs](source_edges.rs): source graph edge definitions.
- [source_execution.rs](source_execution.rs): execution node definitions.
- [source_intake.rs](source_intake.rs): intake and understanding node definitions.
- [source_maintenance.rs](source_maintenance.rs): maintenance node definitions.
- [source_nodes.rs](source_nodes.rs): shared source node constructors.
- [source_packages.rs](source_packages.rs): context package definitions.
- [source_planning.rs](source_planning.rs): planning node definitions.
- [source_recovery.rs](source_recovery.rs): base recovery node definitions.
- [source_recovery_extra.rs](source_recovery_extra.rs): ladder recovery node definitions.
- [source_verification.rs](source_verification.rs): verification node definitions.
- [source_docs.rs](source_docs.rs): document family helpers.
- [transition.rs](transition.rs): legal transition admission.
- [transition_select.rs](transition_select.rs): ranked next-transition selection.
- [state_track.rs](state_track.rs): neutral active-state track model and ranking.
- [state_track_seed.rs](state_track_seed.rs): initial track seeds by task family.
- [validate.rs](validate.rs): source graph validation.
- [validate_tools.rs](validate_tools.rs): validation allowlist for registry tool ids.
