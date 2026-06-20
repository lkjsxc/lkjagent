# Identity

## Purpose

Define stable keys so retries repair or update semantic equivalents instead of
creating duplicate files, records, or memory rows.

## Keys

```text
topic_key = normalized topic path
artifact_key = normalized owner objective + artifact root + artifact kind
section_key = artifact_key + semantic role + normalized title
memory_key = kind + normalized title + normalized tags + content hash prefix
failure_key = fault kind + tool + normalized message + active node
```

## Normalization

Normalization lowercases, trims, collapses punctuation to one separator,
removes empty tokens, and keeps owner-visible names separate from internal
keys. The rendered objective must not show visible counter prefixes.

## Adoption

Before creating a root or section, the runtime scans known manifests, current
roots, README titles, section roles, and owner objective hashes. If an
equivalent exists, the write plan adopts or repairs it.

## Status

partially implemented for memory and document graph manifests. Artifact,
section, and failure-key adoption remain open.
