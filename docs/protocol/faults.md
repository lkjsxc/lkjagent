# Protocol Faults

## Purpose

Define strict parsing, bounded diagnostics, and material repair.

## Parser

The pure parser accepts exactly one complete expected root. It rejects missing,
multiple, unclosed, crossed, attributed, unknown, duplicate, oversized, and
bad-entity tags; JSON/prose action encoding; unknown tools; missing fields;
invalid primitives; unsafe paths; and values outside descriptor bounds.

It never repairs values, executes partial output, guesses a tool, or accepts
readiness prose as success.

## Bounds

Envelope, scalar, field, and total output bounds come from the persisted decision
spec. XML decoding happens after structural and byte checks. Resulting UTF-8 is
validated again before admission.

## Diagnosis

A fault records stable class/signature and one bounded diagnostic without the raw
failed body. Recovery renders the exact current descriptor and one filled valid
example. Successful responses do not carry an `ok` diagnosis into recovery.

## Premature Final

Final output outside respond records `premature_final`, remains invisible as an
agent message, and returns to the phase owning the unmet obligation. Repeated
future-tense promises cannot close or idle the matter.

## Provider Anomalies

Reasoning-only, empty-with-usage, missing content, malformed provider message,
native tool-call-only JSON, and length termination are distinct outcomes. Length
is not classified as a connection fault.
