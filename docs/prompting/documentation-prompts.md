# Documentation Prompts

## Purpose

Documentation prompts own the text contract for documentation tasks. They block
broad scaffolds, disconnected blurbs, unsupported external facts, and topology-
only completion.

## Intake Output

```text
TaskContract
TopicSet
ScopeBoundary
Assumptions
Unknowns
RequiredEvidence
CompletionGates
```

## Seed Output

The semantic seed prompt may create only the root README, immediate topic
README files, one or two concept pages per topic, and at least one relation
page. Directories with `.md` suffixes are forbidden.

## Expansion Output

Expansion changes one local neighborhood. It updates the local README, root
README, and relation pages in the same batch. It does not fan out generic
`architecture`, `guides`, and `operations` directories at the start.

## Audit Output

Audit prompts return failures, evidence, and a repair plan. They never close the
case and never mutate files.

## Status

design-only
