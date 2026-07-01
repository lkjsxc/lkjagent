# lkjagent-runtime Step Helpers

## Purpose

This directory holds helper modules for the pure runtime step transition.

## Table of Contents

- [compact.rs](compact.rs): compaction rebuild step helper.
- [budget.rs](budget.rs): turn budget spending and budget notice routing.
- [cycle.rs](cycle.rs): maintenance cycle start helper.
- [fault_meta.rs](fault_meta.rs): recovery fault kind counters and labels.
- [fault_wait.rs](fault_wait.rs): recovery fault recording and graph routing helper.
- [frames.rs](frames.rs): notice and result construction helpers.
- [graph_output.rs](graph_output.rs): graph evidence updates after tools.
- [graph_phase.rs](graph_phase.rs): graph phase and evidence-kind helpers.
- [input.rs](input.rs): pure step input enum.
- [output.rs](output.rs): tool-output frame and control-action helpers.
- [owner_guidance.rs](owner_guidance.rs): owner follow-up root and artifact guidance updates.
- [oversize.rs](oversize.rs): endpoint oversize recovery messages.
- [oversize_step.rs](oversize_step.rs): endpoint oversize recovery state transition.
- [provider_anomaly.rs](provider_anomaly.rs): provider anomaly notice and recovery step.
- [recovery_select.rs](recovery_select.rs): graph selector bridge for recovery routes.
- [turn.rs](turn.rs): owner and model completion step helpers.
- [action_params.rs](action_params.rs): action params source module.
- [budget_render.rs](budget_render.rs): budget render source module.
- [effects_model.rs](effects_model.rs): effects model source module.
- [fault_key.rs](fault_key.rs): fault key source module.
- [graph_output_evidence.rs](graph_output_evidence.rs): graph output evidence source module.
- [graph_output_plan.rs](graph_output_plan.rs): graph output plan source module.
- [graph_output_plan_helpers.rs](graph_output_plan_helpers.rs): graph output plan helpers source module.
