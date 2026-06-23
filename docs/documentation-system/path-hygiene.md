# Path Hygiene

## Purpose

This file owns generated documentation path names so LLM agents can scan the tree without long,
combined, or serial filenames.

## Limits

Generated Markdown paths stay within these bounds unless a local contract names a stricter need:

- path segment length: 48 characters.
- Markdown stem length: 40 characters.
- full relative path under a generated root: 120 characters.
- directory depth under docs: 6 levels.

## Slug Rule

Slug generation keeps a small set of information-bearing words, removes connector words, caps the
stem, and uses a deterministic suffix only when truncation or collision requires it. Multi-topic
requests become separate topic pages instead of one combined filename.

## Audit Failures

The documentation audit names these path failures directly:

- `path_segment_too_long`.
- `markdown_stem_too_long`.
- `path_too_long`.
- `multi_topic_slug`.
- `markdown_suffix_directory`.
- `serial_filename`.

## Examples

A request for model endpoint, Minecraft, Windows, Japan, and the United States creates topic pages
such as `topics/model-endpoint.md`, `topics/minecraft.md`, `topics/windows.md`, `topics/japan.md`,
and `topics/united-states.md`.

It does not create one filename that concatenates every owner term.

## Status

design-only
