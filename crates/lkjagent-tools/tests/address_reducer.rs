mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::address::{
    resolve_artifact_address, AddressNextAction, ArtifactAddress, ArtifactAddressKind, PathKind,
    ResolveInput, RootPathProblem,
};
use support::{temp_workspace, TestResult};

#[test]
fn classifies_directory_roots_and_missing_roots() -> TestResult<()> {
    let workspace = temp_workspace("address-reducer-roots")?;
    fs::create_dir_all(workspace.join("with-catalog"))?;
    fs::write(
        workspace.join("with-catalog/catalog.toml"),
        "kind = \"story\"\n",
    )?;
    fs::create_dir_all(workspace.join("plain-root"))?;

    assert_address(
        resolve(&workspace, "with-catalog", None)?,
        ArtifactAddressKind::RootDirectory,
        None,
        PathKind::Directory,
    );
    assert_address(
        resolve(&workspace, "plain-root", None)?,
        ArtifactAddressKind::RootDirectory,
        None,
        PathKind::Directory,
    );
    assert_address(
        resolve(&workspace, "missing-root", None)?,
        ArtifactAddressKind::MissingRoot,
        None,
        PathKind::Missing,
    );
    assert_address(
        resolve(&workspace, "missing.md", None)?,
        ArtifactAddressKind::InvalidRoot,
        Some(RootPathProblem::RootEndsWithMarkdownSuffix),
        PathKind::Missing,
    );
    Ok(())
}

#[test]
fn classifies_files_markdown_directories_and_paths() -> TestResult<()> {
    let workspace = temp_workspace("address-reducer-files")?;
    fs::create_dir_all(workspace.join("stories/root/topics"))?;
    fs::write(
        workspace.join("stories/root/catalog.toml"),
        "kind = \"story\"\n",
    )?;
    fs::write(workspace.join("stories/root/topics/a.md"), "# A\n")?;
    fs::create_dir_all(workspace.join("characters.md"))?;
    fs::write(workspace.join("orphan.md"), "# Orphan\n")?;

    let known_file = resolve(&workspace, "stories/root/topics/a.md", None)?;
    assert_eq!(known_file.kind, ArtifactAddressKind::FileUnderKnownRoot);
    assert_eq!(known_file.problem, Some(RootPathProblem::RootIsFile));
    assert_eq!(known_file.root.as_deref(), Some("stories/root"));
    assert_eq!(known_file.weak_path.as_deref(), Some("topics/a.md"));

    let orphan = resolve(&workspace, "orphan.md", None)?;
    assert_eq!(orphan.kind, ArtifactAddressKind::InvalidRoot);
    assert_eq!(orphan.problem, Some(RootPathProblem::RootIsFile));
    assert_eq!(orphan.root, None);

    let bad_dir = resolve(&workspace, "characters.md", None)?;
    assert_eq!(
        bad_dir.problem,
        Some(RootPathProblem::RootEndsWithMarkdownSuffix)
    );
    assert_eq!(bad_dir.detected, PathKind::Directory);

    let weak = resolve(&workspace, "stories/root", Some("topics/a.md"))?;
    assert_eq!(weak.weak_path.as_deref(), Some("topics/a.md"));
    assert!(matches!(
        weak.next_action,
        AddressNextAction::RepairPath { .. }
    ));
    Ok(())
}

#[test]
fn rejects_empty_outside_and_escaping_paths() -> TestResult<()> {
    let workspace = temp_workspace("address-reducer-outside")?;
    assert_eq!(
        resolve(&workspace, "", None)?.problem,
        Some(RootPathProblem::RootMissing)
    );
    assert_eq!(
        resolve(&workspace, "../outside", None)?.problem,
        Some(RootPathProblem::RootOutsideWorkspace)
    );
    let result = resolve_artifact_address(ResolveInput {
        workspace: &workspace,
        requested_root: "root",
        requested_path: Some("../outside.md"),
        kind: "story",
    });
    assert!(result.is_err());
    Ok(())
}

#[cfg(unix)]
#[test]
fn classifies_existing_non_directory_objects() -> TestResult<()> {
    use std::os::unix::net::UnixListener;

    let workspace = temp_workspace("address-reducer-other")?;
    let listener = UnixListener::bind(workspace.join("socket"))?;
    let address = resolve(&workspace, "socket", None)?;
    assert_eq!(address.kind, ArtifactAddressKind::InvalidRoot);
    assert_eq!(address.problem, Some(RootPathProblem::RootNotDirectory));
    assert_eq!(address.detected, PathKind::Other);
    drop(listener);
    Ok(())
}

fn resolve(workspace: &Path, root: &str, path: Option<&str>) -> TestResult<ArtifactAddress> {
    Ok(resolve_artifact_address(ResolveInput {
        workspace,
        requested_root: root,
        requested_path: path,
        kind: "story",
    })?)
}

fn assert_address(
    address: ArtifactAddress,
    kind: ArtifactAddressKind,
    problem: Option<RootPathProblem>,
    detected: PathKind,
) {
    assert_eq!(address.kind, kind);
    assert_eq!(address.problem, problem);
    assert_eq!(address.detected, detected);
}
