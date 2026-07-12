use std::path::Path;

use lkjagent_xtask::docs_authority_gate::check_contract;
use lkjagent_xtask::facts::collect_files;
use lkjagent_xtask::model::RepoFile;

fn repo_files() -> Vec<RepoFile> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    collect_files(&root).unwrap_or_else(|error| vec![RepoFile::new("collection-error", error)])
}

fn edit(files: &mut [RepoFile], path: &str, change: impl FnOnce(&mut String)) {
    let file = files.iter_mut().find(|file| file.path == path);
    assert!(file.is_some(), "missing {path}");
    if let Some(file) = file {
        change(&mut file.text);
    }
}

fn remove(files: &mut [RepoFile], path: &str, token: &str) {
    edit(files, path, |text| {
        assert!(text.contains(token), "missing test token {token} in {path}");
        *text = text.replacen(token, "removed-contract-text", 1);
    });
}

fn assert_fails_with(files: &[RepoFile], expected: &str) {
    let failures = check_contract(files);
    assert!(
        failures.iter().any(|failure| failure.contains(expected)),
        "expected {expected:?} in {failures:#?}"
    );
}

#[test]
fn current_docs_satisfy_compact_authority_contract() {
    assert_eq!(check_contract(&repo_files()), Vec::<String>::new());
}

#[test]
fn every_compact_page_is_required() {
    let mut files = repo_files();
    files.retain(|file| file.path != "docs/workspace/effects.md");
    assert_fails_with(&files, "compact authority page is missing");
}

#[test]
fn removed_authority_page_is_rejected() {
    let mut files = repo_files();
    files.push(RepoFile::new(
        "docs/runtime/authority-model.md",
        "# Old Authority\n\n## Purpose\n\nCompete with the direct loop.\n",
    ));
    assert_fails_with(&files, "outside the compact 47-page authority map");
}

#[test]
fn removed_authority_name_is_rejected_in_active_contract() {
    let mut files = repo_files();
    edit(&mut files, "docs/runtime/loop.md", |text| {
        text.push_str("\nTaskSnapshot selects the next action.\n");
    });
    assert_fails_with(&files, "retired authority name remains");
}

#[test]
fn focused_runtime_and_effect_facts_are_required() {
    for (path, token) in [
        ("AGENTS.md", "persisted `RuntimeDecision` rows"),
        (
            "docs/context/assembly.md",
            "selector first persists immutable operation",
        ),
        ("docs/tools/registry.md", "`create_file`"),
        ("docs/protocol/envelopes.md", "decision IDs"),
        ("docs/product/daemon.md", "separate capabilities"),
        ("docs/workspace/effects.md", "prior/intended bytes"),
        ("docs/product/tui.md", "`conversation_messages`"),
        ("docs/runtime/completion.md", "native checks automatically"),
        ("docs/runtime/completion.md", "harness-rendered"),
    ] {
        let mut files = repo_files();
        remove(&mut files, path, token);
        assert_fails_with(&files, "missing focused contract text");
    }
}

#[test]
fn current_state_evidence_boundary_and_work_state_are_required() {
    for token in [
        "`5604ec89af3ba9dbfb287bd869971781fdcf2fad`",
        "A synthetic 901-second run",
        "| acceptance-checker | complete |",
    ] {
        let mut files = repo_files();
        remove(&mut files, "docs/current-state.md", token);
        assert_fails_with(&files, "missing focused contract text");
    }
}

#[test]
fn all_three_tracked_plans_are_required() {
    for path in [
        "evaluation/workgraph.tsv",
        "evaluation/acceptance.tsv",
        "evaluation/experiment-plan.tsv",
    ] {
        let mut files = repo_files();
        files.retain(|file| file.path != path);
        assert_fails_with(&files, "required contract input is missing");
    }
}

#[test]
fn acceptance_checker_contract_is_required() {
    let mut files = repo_files();
    remove(
        &mut files,
        "docs/evaluation/live-proof.md",
        "nine negative fixtures",
    );
    assert_fails_with(&files, "missing focused contract text");
}
