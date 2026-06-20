# Artifacts

## Purpose

Define semantic artifact construction for long stories, cookbooks, guides,
encyclopedias, and other large content deliverables.

## Table of Contents

- [content-artifacts.md](content-artifacts.md): general large-content contract.
- [cookbook-profile.md](cookbook-profile.md): cookbook profile example.
- [story-profile.md](story-profile.md): story profile example.
- [manifest.md](manifest.md): artifact manifest fields and identity.
- [scaffold.md](scaffold.md): scaffold and apply behavior.
- [audit.md](audit.md): semantic artifact audit checks.
- [completion.md](completion.md): artifact completion gate.

## Contract

A large content request is never one giant `fs.write`. It is a semantic
artifact root with README, manifest, section files, audit evidence, and
completion evidence.

## Status

partially implemented through document scaffold profiles and audit checks;
explicit artifact tools, manifest adoption, and repair remain open.
