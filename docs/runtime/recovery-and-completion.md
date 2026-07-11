# Recovery And Completion

## Purpose

Define causal recovery and check-gated matter completion.

## Failure Lineage

Every failure records class, normalized signature, decision, operation, state,
context, tool-view and budget fingerprints, attempted strategy, changed
condition, bounded diagnostic, retry count, next strategy, eligibility time,
and remaining budget.

The operation, prompt, tool view, budget, and failure signature tuple may not
repeat without a changed external condition. Each structured failure cell names
a matching immutable runtime event. Recurrence selects the next typed strategy
or records an owner-visible block with preserved evidence and an owner action.

## Recovery Ladders

Output limits shrink or split semantic units. Parse failures repair the exact
grammar before narrowing the shape. Admission failures remove hidden actions or
correct typed fields. Effect failures inspect external state before replay or
compensation. Check failures inspect measured results, repair source, and rerun
the invalidated check. Each selected strategy changes a rendered instruction,
prompt bound, output budget, eligibility instant, or operation key.

## Progress Windows

Each settled decision records a canonical vector of obligations, passed checks,
artifacts, clean source evidence, dependency edges, wake conditions, and strategy. A
configured window of equal vectors emits a typed strategy change. Even a
one-decision setting requires two equal observations; one changed vector cannot
be called stasis. A changed artifact, check, wake, evidence source, or strategy
resets the window.

## Crash Recovery

One provider call belongs to one decision. Retryable endpoint faults persist a
configured exponential eligibility instant; retry exhaustion persists an
external wait bound to a nonsecret endpoint-configuration fingerprint. The
credential contributes only a domain-separated SHA-256 token, never its bytes.
The daemon lock orders config and fingerprint updates. A later fingerprint
change durably suppresses the wait and permits one new call.

Startup reconciles prepared effects before endpoint decisions, projections, and due wakes.
A dispatching provider-exchange intent commits before network I/O. If startup
finds that sent-request boundary unfinished, it suppresses the source operation,
marks the decision interrupted, and emits a durable owner-visible blocker. It
never repeats the call or reuses an uncommitted response.

## Case Budgets

Token usage, active decision milliseconds, effect observations, and recovery
cost have independent configured limits. Durable rows compute consumption. An
exhausted dimension suppresses runnable work and records its used and limit
values in `completion.blocked`; it does not masquerade as successful completion.

## Completion

The reducer creates a completion candidate only after all required obligations
have current passed checks. Any current active, pending, failed, blocked, or
unsuperseded operation prevents it. Readiness prose, future-tense promises,
elapsed time, and empty work lists are not evidence.

The close transaction commits final checks, completion event, lifecycle change,
and final conversation message together. A later source change invalidates
dependent checks and completion evidence through a new event.
