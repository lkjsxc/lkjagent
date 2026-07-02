# Task Brief

## Purpose

Define the rolling summary used by prompts, status, and responses.

## Contract

The task brief is an engine-maintained summary stored on the task row. It is
rewritten after meaningful state changes and capped by
`context.system.brief-tokens=450`.

The brief contains:

- the objective, referenced without rewriting its meaning;
- completed step facts that future steps need;
- named artifact facts such as characters, terms, roots, and paths;
- at most `context.memory.fact-tokens=100` of memory facts with provenance;
- current blocker diagnoses when the retry ladder needs task review.

## Exclusions

The brief does not include failed model bodies, full transcripts, raw tool logs,
or unverified claims. It may carry a bounded continuity fact after a successful
write, but prose continuity normally comes from file tails.

## Update Rule

The engine updates the brief from measured state and step outcomes. A respond
step may propose wording, but the store row is not model-owned.

## Failure This Prevents

The model cannot learn to repeat a rejected output from the brief, and the owner
cannot see a summary that smooths over failed checks.
