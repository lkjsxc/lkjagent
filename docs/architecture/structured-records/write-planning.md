# Write Planning

## Purpose

Define bounded semantic file planning for large documents and content
artifacts.

## Plan

An ArtifactPlan records root, title, kind, nodes, links, checks, and the
adoption or repair decision for existing files. Nodes carry path, role, title,
required flag, content policy, and artifact identity.

Rules:

- Never create sequence-only files unless the owner explicitly asks for
  numbered parts.
- Never generate `part-001.md` as a generic fallback.
- Never create a second semantic equivalent file when one exists.
- Adopt existing artifact roots before creating new roots.
- Repair stale generated files rather than duplicate them.
- Use manifests to track semantic roles and artifact identity.

## Content Artifacts

Long stories, very long stories, cookbooks, encyclopedias, guides, corpora,
many-topic requests, structured followups, and write attempts that hit max
tokens route to content artifacts. Completion requires content-bearing files,
not scaffold-only files.

## Status

partially implemented; story and cookbook scaffold profiles generate semantic
paths and avoid sequence-only names. Existing-root adoption, repair planning,
bounded section writing, and content-bearing completion remain open.
