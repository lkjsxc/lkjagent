# lkjagent-tools Source

## Purpose

This directory holds tool dispatcher, adapter, and observation code.

## Table of Contents

- [benchmark_seed.rs](benchmark_seed.rs): deterministic benchmark corpus scaffold.
- [control/](control/README.md): completion guard helper modules.
- [control.rs](control.rs): agent.done and agent.ask state transitions.
- [count_guard.rs](count_guard.rs): exact and approximate file-count guards.
- [count_guard/](count_guard/README.md): file-count scanning helpers.
- [count_number.rs](count_number.rs): counted wording number-span parser.
- [count_number_kanji.rs](count_number_kanji.rs): Japanese numeric kanji helpers.
- [count_number_words.rs](count_number_words.rs): English number-word parser.
- [count_profile_anchor_cleanup.rs](count_profile_anchor_cleanup.rs): content-anchor count cleanup.
- [count_profile_audit.rs](count_profile_audit.rs): counted scaffold acceptance audit text.
- [count_profile_anchor_content.rs](count_profile_anchor_content.rs): body anchor cleanup.
- [count_profile_anchor.rs](count_profile_anchor.rs): counted scaffold requirement anchors.
- [count_profile_body.rs](count_profile_body.rs): counted scaffold body spines.
- [count_profile_design.rs](count_profile_design.rs): counted scaffold design memo bodies.
- [count_profile_detail.rs](count_profile_detail.rs): counted scaffold per-part content details.
- [count_profile_index.rs](count_profile_index.rs): counted scaffold index sections.
- [count_profile_kind.rs](count_profile_kind.rs): counted scaffold kind detection.
- [count_profile_local.rs](count_profile_local.rs): counted scaffold per-file verification text.
- [count_profile_manifest.rs](count_profile_manifest.rs): counted scaffold audit manifest text.
- [count_profile_passage.rs](count_profile_passage.rs): counted scaffold draft passage blocks.
- [count_profile_reading.rs](count_profile_reading.rs): counted scaffold root reading path text.
- [count_profile_restart.rs](count_profile_restart.rs): counted scaffold restart guide text.
- [count_profile_stage.rs](count_profile_stage.rs): counted scaffold stage labels and ranges.
- [count_profile_thread.rs](count_profile_thread.rs): counted scaffold segment briefs.
- [count_profile_variation.rs](count_profile_variation.rs): counted scaffold per-part variation data.
- [count_profile_variation_en.rs](count_profile_variation_en.rs): English per-part variation data.
- [count_profile_variation_jp.rs](count_profile_variation_jp.rs): Japanese per-part variation data.
- [count_profile.rs](count_profile.rs): objective profile text for counted scaffolds.
- [count_profile_data.rs](count_profile_data.rs): counted scaffold profile labels.
- [count_seed.rs](count_seed.rs): counted document scaffold generator.
- [count_seed_allocation.rs](count_seed_allocation.rs): counted scaffold file allocation.
- [count_seed_allocation_infer.rs](count_seed_allocation_infer.rs): inferred split units.
- [count_seed_allocation_lead.rs](count_seed_allocation_lead.rs): split allocation lead words.
- [count_seed_allocation_split.rs](count_seed_allocation_split.rs): remaining-file allocation hints.
- [count_seed_allocation_signals.rs](count_seed_allocation_signals.rs): allocation signal spans.
- [count_seed_allocation_units.rs](count_seed_allocation_units.rs): split allocation unit words.
- [count_seed_verify.rs](count_seed_verify.rs): counted scaffold structural verifier.
- [count_seed_verify_manifest.rs](count_seed_verify_manifest.rs): counted scaffold audit-manifest verifier.
- [count_seed_verify_main.rs](count_seed_verify_main.rs): counted scaffold main-file verifier.
- [count_seed_verify_reading.rs](count_seed_verify_reading.rs): counted scaffold reading-path verifier.
- [count_seed_verify_root.rs](count_seed_verify_root.rs): counted scaffold root budget verifier.
- [count_seed_verify_restart.rs](count_seed_verify_restart.rs): counted scaffold restart guide verifier.
- [count_seed_verify_text.rs](count_seed_verify_text.rs): counted scaffold text-section verifier.
- [dispatch/](dispatch/README.md): dispatch helper modules.
- [dispatch.rs](dispatch.rs): registry validation and tool routing.
- [error.rs](error.rs): tool error type.
- [fs.rs](fs.rs): filesystem read, write, and edit adapters.
- [fs_list.rs](fs_list.rs): bounded deterministic file listing.
- [fs_search.rs](fs_search.rs): bounded substring search.
- [fs_stat.rs](fs_stat.rs): deterministic file metadata.
- [fs_batch.rs](fs_batch.rs): directory creation and batched file writes.
- [fs_tree.rs](fs_tree.rs): bounded deterministic workspace tree output.
- [lib.rs](lib.rs): library root.
- [memory.rs](memory.rs): memory save, find, and prune adapters.
- [observe.rs](observe.rs): bounded frame construction helpers.
- [queue.rs](queue.rs): queue list and mutation adapters.
- [shell.rs](shell.rs): /bin/sh adapter with timeout handling.
- [workspace.rs](workspace.rs): repository shape summaries and compact index.
- [verify.rs](verify.rs): direct cargo and xtask verification runners.
- [artifact.rs](artifact.rs): artifact plan, apply, audit, and next entrypoints.
- [artifact_next.rs](artifact_next.rs): bounded artifact write-batch planner.
- [doc/](doc/README.md): semantic scaffold and audit helper modules.
- [doc.rs](doc.rs): document scaffold and audit entrypoints.
- [structure.rs](structure.rs): recursive tree completion checks.
- [structure_network.rs](structure_network.rs): knowledge network completion checks.
- [structure_seed/](structure_seed/README.md): deterministic docs scaffold profiles.
- [structure_seed.rs](structure_seed.rs): scaffold profile entrypoint.
