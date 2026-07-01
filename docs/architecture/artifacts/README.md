# Artifacts

## Purpose

Define semantic artifact construction for dictionaries, long stories,
cookbooks, guides, encyclopedias, and other large content deliverables.

## Table of Contents

- [lifecycle.md](lifecycle.md): scaffold, content pass, audit pass, and completion.
- [artifact-ledger.md](artifact-ledger.md): durable artifact identity, weak paths, and audit state.
- [content-artifacts.md](content-artifacts.md): general large-content contract.
- [large-artifact/](large-artifact/README.md): durable atom graph engine for long structured works.
- [content-atoms.md](content-atoms.md): bounded semantic units for large artifacts.
- [deterministic-assembly.md](deterministic-assembly.md): daemon-owned assembly from approved atoms.
- [content-readiness.md](content-readiness.md): meaningful leaf-file checks.
- [readiness-reducer.md](readiness-reducer.md): profile-specific content readiness decisions.
- [dictionary-profile.md](dictionary-profile.md): dictionary artifact profile.
- [cookbook-profile.md](cookbook-profile.md): cookbook profile example.
- [story-profile.md](story-profile.md): story profile example.
- [manuscript-lifecycle.md](manuscript-lifecycle.md): chapter-prose lifecycle for story manuscripts.
- [manifest.md](manifest.md): artifact manifest fields and identity.
- [scaffold.md](scaffold.md): scaffold and apply behavior.
- [audit.md](audit.md): semantic artifact audit checks.
- [completion.md](completion.md): artifact completion gate.
- [completion-gates.md](completion-gates.md): deterministic completion checks.
- [adoption.md](adoption.md): existing root adoption.
- [repair.md](repair.md): bounded repair after failed audit.
- [write-batches.md](write-batches.md): bounded content write loop.
- [root-identity.md](root-identity.md): initial identity contract for missing roots.
- [root-repair.md](root-repair.md): missing-root repair route and admission.
- [semantic-identity.md](semantic-identity.md): artifact identity fields.
- [adoption-and-repair.md](adoption-and-repair.md): adoption followed by readiness repair.
- [adoption-controller.md](adoption-controller.md): runtime adoption route and audit outputs.
- [repair-controller.md](repair-controller.md): bounded repair routing after failed audit.
- [batch-cursors.md](batch-cursors.md): durable batch progress state.
- [readiness-evidence.md](readiness-evidence.md): evidence that can satisfy readiness.
- [false-completion.md](false-completion.md): scaffold-only and weak-content close refusal.
- [objective-drift.md](objective-drift.md): objective-match drift audit and guard behavior.
- [path-aliases.md](path-aliases.md): short semantic roots and artifact cards.

## Contract

A large content request is never one giant `fs.write`. It is a semantic
artifact root with README, manifest, section files, audit evidence, and
completion evidence. Scaffold, content pass, audit pass, repair, and completion
are separate states.

## Status

Implemented for scaffold profiles, artifact tool wrappers, ledger-backed
identity, durable atom graph plans, exact-path write contracts, content atom
audit, deterministic scene assembly, dictionary readiness, adoption, repair
routing, store-projected readiness, and audit-owned completion checks. Live
endpoint proof of a complete 10,000-word daemon manuscript remains operator
smoke evidence, tracked in the
[story gap](../../execution/current-work/story-manuscript-generation-gap.md).
