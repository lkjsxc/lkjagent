# State

## Purpose

This directory owns the runtime state contract for lkjagent cases: one hard
state controls legality, and weighted tracks express concurrent pressure that
can block tools, prompts, and completion.

## Table of Contents

- [hard-state.md](hard-state.md): hard lifecycle nodes and transition authority.
- [weighted-state.md](weighted-state.md): concurrent track shape, weights, and posture.
- [state-vector.md](state-vector.md): storage and reducer rules for active tracks.
- [transition-handbook.md](transition-handbook.md): documentation growth transitions.
- [track-update-policy.md](track-update-policy.md): event-to-track updates and decay.
- [track-guards.md](track-guards.md): guard thresholds that affect tool admission.

## Local Map

- The hard state names what may happen now.
- The weighted vector names why the next safe action may differ from the plan.
- Guard tracks feed [../prompting/prompt-frame.md](../prompting/prompt-frame.md)
  and [../documentation-system/growth-stages.md](../documentation-system/growth-stages.md).

## Status

implemented
