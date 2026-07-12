# Recovery

## Purpose

Define durable recovery that changes conditions and preserves owner bytes.

## Failure Record

Each failure stores class, normalized signature, decision, operation, state,
source revision, tool/context/grammar fingerprints, strategy, bounded diagnosis,
next eligibility, and remaining budget. Raw failed model bodies do not enter
normal prompts.

## Strategies

- Protocol faults receive the exact code and one descriptor-generated example.
- Hidden tools regenerate only the current decision view.
- Premature final output returns to the phase owning the unmet obligation.
- Missing reads activate a source-revision need.
- Stale or ambiguous edits reread and rebase; they never overwrite.
- Output limits reduce scope or split one semantic unit.
- Endpoint faults use typed retry/wait policy and config-change wakes.
- Check faults expose measured differences and return to modification.
- Equal progress vectors change evidence, tools, scope, or wake.

A tuple cannot repeat without a changed fingerprinted factor or external
condition. A successful attempt with an `ok` outcome never creates a fault card.

## Crash Reconciliation

Provider intent persists before send. An ambiguous sent boundary is not replayed.

File effects persist prior/intended bytes and mode, stage identity, and each
exchange or compensation phase. Startup inspects descriptor-relative target and
stage values. Intended target bytes are successful only when captured bytes and
mode match the expected preimage. A newer owner value wins during compensation.
Unclassified states block without overwrite.

## Budgets And Fairness

Model calls, effects, recovery cost, active time, and tokens have separate durable
limits. Exhaustion creates a visible resumable block with used and limit values.
One blocked matter cannot starve an unrelated runnable matter.
