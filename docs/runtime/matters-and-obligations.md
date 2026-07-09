# Matters And Obligations

## Purpose

Define owner goals and the evidence predicates that control completion.

## Matter

A matter stores an opaque ID, run and optional scenario identity, objective,
lifecycle, priority, and created and updated causal sequences. Lifecycle is one
concurrent state dimension, not a complete description of runtime work.

## Obligation

An obligation stores its matter, predicate kind and typed payload, whether it is
required, current state, passed current same-matter check reference, and the
event that invalidated it when present.

Required predicates come from owner intent and harness policy. Examples include
current file bytes at a named path, a workspace revision with a fingerprint, a
passed command, a resolved owner question, or a final response that names
verified outputs.

## Satisfaction

The reducer may mark an obligation satisfied only from a passed current check
for the same matter. Source changes invalidate the check and obligation through
a causal event. Historical, stale, or other-matter checks cannot satisfy it.

## Completion

The reducer creates a completion candidate only when every required obligation
is satisfied and no current operation is active, pending, failed, blocked, or
unsuperseded. The verify operation recomputes checks from final state. The model
may report progress but cannot create or settle a completion candidate.
