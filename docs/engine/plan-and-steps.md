# Plan And Steps

## Purpose

Define ordered artifact work as a state-ledger plan family.

## Plan State

The current checkout stores bridge plans as ordered row bodies. The target
keeps ordered artifact work but represents it as `plan:*` state cells plus
events and context items. Exactly one runnable plan item may be selected by a
runtime decision, but other state cells can remain active at the same time.

## Plan Item Payload

A plan item payload records identity, ordinal, title, operation key, instruction,
inputs, output path when applicable, attached check predicates, attempts used,
action budget, lineage, and evidence refs.

## Attempts

One model call creates provider exchange rows, token usage rows, prompt-frame
refs, and events tied to the persisted decision. Consecutive failed decisions for
the same plan item must differ in fingerprint or recovery policy.

## Mutation

Plan mutation is a reducer output, not model authority. Model-authored plan text
is parsed into events. The reducer may split a divisible item, narrow an
exploration item, extend content after a measured shortfall, block an item, or
request owner input when evidence shows the need.

## Failure This Prevents

Path drift is impossible for scripted work because paths and artifact refs sit in
state before content is written, while the active state vector can still expose
other constraints to the decision.
