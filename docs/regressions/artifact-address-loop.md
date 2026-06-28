# Artifact Address Loop

## Purpose

This page records the SF novel failure where artifact tools confused Markdown
leaf paths with artifact roots.

## Input Signature

The owner asked for a SF novel with structured settings. The model selected a
Markdown leaf as an artifact root, then targeted a file under that leaf:

```text
artifact.apply root=stories/sf-novel-with-structured-settings/02-characters.md
artifact.next root=stories/sf-novel-with-structured-settings/02-characters.md/topics/background.md
artifact.audit root=stories/sf-novel-with-structured-settings/02-characters.md/topics/background.md
```

## Forbidden Behavior

- artifact.apply creates a directory ending in `.md`.
- artifact.next reports `missing=0` for a file root.
- artifact.next renders artifact.audit for a Markdown file path.
- doc.audit or artifact.audit surfaces `Not a directory` for known file-root misuse.
- batch-write JSON recovery causes partial writes or repeated parse loops.

## Expected Guard

The address reducer classifies file roots before dispatch. It reports
`root_is_file` or `root_ends_with_markdown_suffix`, names the owning root when
known, keeps the weak path relative to that root, and renders one parseable
next action that does not put a Markdown file in a root-only slot.

## Evidence

- `cargo test -p lkjagent-tools --test artifact_address`
- `cargo test -p lkjagent-tools --test doc_root_hygiene`
- `cargo test -p lkjagent-tools --test batch_write_formats`
- `cargo test -p lkjagent-runtime --test artifact_address_recovery`
- `cargo run -p lkjagent-xtask -- benchmark check-corpus`

## Status

partially implemented
