# Question Template

## Purpose

Define direct answer matters.

## Selection

The classifier selects this template only when the owner asks for an answer and
not a record write, artifact, or system operation.

## Shape

- One model decision may emit `<message>` with the answer.
- Retrieval decisions admit source-linked workspace records and memory facts
  before answering.
- If the answer depends on files, the matter records the source refs used.

## Checks

Direct answers usually have no file checks. When the question names a workspace
path or record, the check verifies the cited source existed and was admitted to
the prompt frame.
