# Relation Graph

## Purpose

The relation graph owns cross-topic connectedness. It prevents separate topic
summaries from masquerading as a knowledge structure.

## Relation Types

```text
implements
depends-on
constrains
observes
audits
recovers
prompts
updates
blocks
supersedes
```

## Page Duties

Each durable topic page answers:

- What does this concept depend on?
- What depends on it?
- Which state transitions mention it?
- Which prompts include it?
- Which audits verify it?
- Which recovery path repairs it?

## Audit Rule

A multi-topic request fails if no relation page links the requested topics, if a
topic directory has no outgoing relation, or if backlinks are missing from
pages that the relation file names.

## Status

implemented
