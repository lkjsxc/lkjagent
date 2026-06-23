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

## Address Rules

When document or artifact tools are available, prompt text states that `root` is
a directory path and `path` is a Markdown file path under a root. A `.md` file
must not be passed as root to doc.audit, artifact.audit, doc.scaffold, or
artifact.apply. Markdown content goes through fs.write or fs.batch_write.

## Batch Write Grammar

The default example for fs.batch_write is line protocol inside `<files>`. The
prompt tells the model not to add a `<path>` parameter to fs.batch_write and not
to wrap files in JSON unless a recovery example explicitly does so.

## Payload Budget

Content-writing prompts name the 1,800 byte per-file limit, the 6,000 byte
batch limit, and the requirement to split large content into multiple semantic
files before acting.

## Audit Output

Audit prompts return failures, evidence, and a repair plan. They never close the
case and never mutate files.

## Status

partially implemented
