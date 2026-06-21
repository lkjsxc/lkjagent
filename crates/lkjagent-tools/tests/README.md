# lkjagent-tools Tests

## Purpose

This directory holds integration tests for dispatch and every tool adapter.

## Table of Contents

- [benchmark_seed.rs](benchmark_seed.rs): benchmark and counted scaffold fixtures.
- [artifact_dictionary.rs](artifact_dictionary.rs): dictionary content-readiness fixtures.
- [artifact_next.rs](artifact_next.rs): next bounded artifact batch fixtures.
- [artifact_tools.rs](artifact_tools.rs): artifact plan, apply, and audit fixtures.
- [control_count_mode_scope.rs](control_count_mode_scope.rs): scoped count-mode wording fixtures.
- [control_count_hyphen_words.rs](control_count_hyphen_words.rs): hyphenated number-word fixtures.
- [control_count_words.rs](control_count_words.rs): count guard number-word fixtures.
- [control_dispatch.rs](control_dispatch.rs): control tool close, wait, and count guard fixtures.
- [control_guard.rs](control_guard.rs): resumed owner guidance completion guards.
- [control_owner_question.rs](control_owner_question.rs): owner-question admission guard fixtures.
- [count_seed_allocation.rs](count_seed_allocation.rs): counted scaffold allocation fixtures.
- [count_seed_allocation_en.rs](count_seed_allocation_en.rs): English allocation fixtures.
- [count_seed_allocation_jp.rs](count_seed_allocation_jp.rs): Japanese allocation fixtures.
- [count_seed_allocation_sentence_split.rs](count_seed_allocation_sentence_split.rs): sentence split fixtures.
- [count_seed_allocation_split.rs](count_seed_allocation_split.rs): explicit split allocation fixtures.
- [count_seed_allocation_support_units.rs](count_seed_allocation_support_units.rs): support unit fixtures.
- [count_seed_anchor.rs](count_seed_anchor.rs): counted scaffold objective-anchor fixtures.
- [count_seed_cross_index.rs](count_seed_cross_index.rs): counted scaffold cross-index fixtures.
- [count_seed_anchor_parenthetical.rs](count_seed_anchor_parenthetical.rs): parenthetical anchor fixtures.
- [count_seed_kind_profile.rs](count_seed_kind_profile.rs): counted scaffold kind-profile fixtures.
- [count_seed_root_contract.rs](count_seed_root_contract.rs): counted scaffold root contract fixtures.
- [doc_content_audit.rs](doc_content_audit.rs): content-artifact scaffold-only audit fixtures.
- [doc_tools.rs](doc_tools.rs): document scaffold and audit fixtures.
- [dispatch_normalize.rs](dispatch_normalize.rs): safe parameter repair and schema example fixtures.
- [effective_policy_repair.rs](effective_policy_repair.rs): runtime repair admission over graph completion fixtures.
- [fs_shell.rs](fs_shell.rs): filesystem and shell tool fixtures.
- [knowledge_path_guard.rs](knowledge_path_guard.rs): recursive knowledge write fences.
- [graph_control_dispatch.rs](graph_control_dispatch.rs): graph, control, and dispatcher notice fixtures.
- [graph_policy.rs](graph_policy.rs): graph dispatch policy refusal fixtures.
- [memory_tools.rs](memory_tools.rs): memory dedupe and punctuation search fixtures.
- [memory_prune.rs](memory_prune.rs): memory prune merge-output fixtures.
- [native_tools.rs](native_tools.rs): native multi-read, patch, tree, and index fixtures.
- [semantic_examples.rs](semantic_examples.rs): semantic action example dispatch fixtures.
- [structure_seed.rs](structure_seed.rs): deterministic recursive docs scaffold fixture.
- [store_tools.rs](store_tools.rs): queue and memory tool fixtures.
- [support/](support/README.md): shared temp workspace and store helpers.
- [typed_tools.rs](typed_tools.rs): fs list/search/stat/batch, workspace, verify, and doc tool fixtures.
