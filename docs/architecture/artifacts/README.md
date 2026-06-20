# Artifacts

## Purpose

Define semantic artifact construction for long stories, cookbooks, guides,
encyclopedias, and other large content deliverables.

## Table of Contents

- [lifecycle.md](lifecycle.md): scaffold, content pass, audit pass, and completion.
- [content-artifacts.md](content-artifacts.md): general large-content contract.
- [content-readiness.md](content-readiness.md): meaningful leaf-file checks.
- [cookbook-profile.md](cookbook-profile.md): cookbook profile example.
- [story-profile.md](story-profile.md): story profile example.
- [manifest.md](manifest.md): artifact manifest fields and identity.
- [scaffold.md](scaffold.md): scaffold and apply behavior.
- [audit.md](audit.md): semantic artifact audit checks.
- [completion.md](completion.md): artifact completion gate.
- [completion-gates.md](completion-gates.md): deterministic completion checks.
- [adoption.md](adoption.md): existing root adoption.
- [repair.md](repair.md): bounded repair after failed audit.
- [write-batches.md](write-batches.md): bounded content write loop.

## Contract

A large content request is never one giant `fs.write`. It is a semantic
artifact root with README, manifest, section files, audit evidence, and
completion evidence. Scaffold, content pass, audit pass, and completion are
separate states.

## Status

partially implemented through document scaffold profiles, artifact tool
wrappers, and audit checks. Manifest adoption and repair remain open.
