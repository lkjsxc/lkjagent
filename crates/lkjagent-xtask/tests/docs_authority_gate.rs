use std::path::Path;

use lkjagent_xtask::docs_authority_gate::{check_changed_paths, check_contract};
use lkjagent_xtask::facts::collect_files;
use lkjagent_xtask::model::RepoFile;

fn repo_files() -> Vec<RepoFile> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    collect_files(&root).expect("repository files")
}

fn edit(files: &mut [RepoFile], path: &str, change: impl FnOnce(&mut String)) {
    let file = files
        .iter_mut()
        .find(|file| file.path == path)
        .expect("fixture path");
    change(&mut file.text);
}

#[test]
fn current_docs_satisfy_authority_contract() {
    assert_eq!(check_contract(&repo_files()), Vec::<String>::new());
}

#[test]
fn missing_contract_page_fails() {
    let mut files = repo_files();
    files.retain(|file| file.path != "docs/store/effect-journal.md");
    assert!(check_contract(&files)
        .iter()
        .any(|failure| failure.contains("effect-journal.md")));
}

#[test]
fn retired_authority_page_fails() {
    let mut files = repo_files();
    files.push(RepoFile::new(
        "docs/engine/README.md",
        "# Engine\n\n## Purpose\n\nRetired authority.\n",
    ));
    assert!(check_contract(&files)
        .iter()
        .any(|failure| failure.contains("retired page")));
}

#[test]
fn false_live_promotion_fails() {
    let mut files = repo_files();
    edit(&mut files, "docs/current-state.md", |text| {
        text.push_str("\nAll four live profiles ran and closed successfully.\n");
    });
    assert!(check_contract(&files)
        .iter()
        .any(|failure| failure.contains("false live claim")));
}

#[test]
fn incomplete_schema_fails() {
    let mut files = repo_files();
    edit(&mut files, "docs/store/schema.md", |text| {
        *text = text.replace("effect_journal", "effects");
    });
    assert!(check_contract(&files)
        .iter()
        .any(|failure| failure.contains("effect_journal")));
}

#[test]
fn broken_root_topology_fails() {
    let mut files = repo_files();
    edit(&mut files, "docs/README.md", |text| {
        *text = text.replace(
            "14. [tui/](tui/README.md): canonical transcript rendering, input, and scrolling.\n",
            "",
        );
    });
    assert!(check_contract(&files)
        .iter()
        .any(|failure| failure.contains("link child 'tui'")));
}

#[test]
fn product_source_drift_fails() {
    let changed = vec!["crates/lkjagent-app/src/daemon.rs".to_string()];
    assert!(check_changed_paths(&changed)
        .iter()
        .any(|failure| failure.contains("behavior-identical")));
}
