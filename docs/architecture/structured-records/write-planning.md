# Write Planning

## Purpose

Define bounded semantic file planning for large documents and content
artifacts.

## Plan

An ArtifactPlan records root, title, kind, nodes, links, checks, and the
adoption or repair decision for existing files. Nodes carry path, role, title,
required flag, content policy, and artifact identity.

The planner rejects sequence-only names such as part-001.md unless the owner
asks for numbered parts. It generates semantic section names and writes bounded
sections only.

## Content Artifacts

Long stories, very long stories, cookbooks, encyclopedias, guides, corpora,
many-topic requests, structured followups, and write attempts that hit max
tokens route to content artifacts. Completion requires content-bearing files,
not scaffold-only files.

## Status

design, implementation pending
