#![cfg(target_os = "linux")]

use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_effects::workspace_edit::{
    classify, DurablePhase, EditResult, FileValue, Layout, ObservedTarget, Revision,
    VerifiedOutcome,
};
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
type TestResult = Result<(), Box<dyn std::error::Error>>;
static NEXT: AtomicU64 = AtomicU64::new(1);

#[test]
fn preparation_rejects_unbound_or_unsafe_edits() -> TestResult {
    let root = fixture("prepare")?;
    fs::write(root.join("one"), "alpha beta")?;
    fs::set_permissions(root.join("one"), fs::Permissions::from_mode(0o751))?;
    fs::write(root.join("many"), "aaa")?;
    fs::write(root.join("invalid"), [0xff])?;
    symlink(root.join("one"), root.join("link"))?;
    let workspace = OpenedWorkspace::open(&root)?;
    let bound = revision(&workspace, "one")?;
    let prepared = ok(workspace.prepare_exact_edit(
        "one".into(),
        Revision::Sha256(bound.clone()),
        "alpha",
        "gamma",
        0o600,
    ))?;
    assert_eq!(prepared.prior_bytes, Some(b"alpha beta".to_vec()));
    assert_eq!(prepared.intended_bytes, b"gamma beta");
    assert_eq!(prepared.expected_mode, Some(prepared.intended_mode));
    assert_eq!(prepared.intended_mode, 0o751);
    let again = ok(workspace.prepare_exact_edit(
        "one".into(),
        Revision::Sha256(bound.clone()),
        "alpha",
        "gamma",
        0,
    ))?;
    assert_eq!(prepared.stage_identity, again.stage_identity);
    assert_eq!(bound.len(), 64);
    let many = revision(&workspace, "many")?;
    let invalid = revision(&workspace, "invalid")?;
    assert!(workspace
        .prepare_exact_edit(
            "one".into(),
            Revision::Sha256("stale".into()),
            "alpha",
            "x",
            0
        )
        .is_err());
    assert!(workspace
        .prepare_exact_edit("many".into(), Revision::Sha256(many), "aa", "x", 0)
        .is_err());
    assert!(workspace
        .prepare_exact_edit("one".into(), Revision::Sha256(bound), "alpha", "alpha", 0)
        .is_err());
    assert!(workspace
        .prepare_exact_edit("invalid".into(), Revision::Sha256(invalid), "x", "y", 0)
        .is_err());
    assert!(workspace
        .prepare_exact_edit("new".into(), Revision::Absent, "", "", 0o600)
        .is_err());
    assert!(workspace
        .prepare_exact_edit("missing/file".into(), Revision::Absent, "", "x", 0o600)
        .is_err());
    for path in ["../file", "/file", "dir//file", "link"] {
        assert!(workspace
            .prepare_exact_edit(path.into(), Revision::Absent, "", "x", 0o600)
            .is_err());
    }
    assert!(workspace
        .prepare_exact_edit(
            "large".into(),
            Revision::Absent,
            "",
            &"x".repeat(1_048_577),
            0o600
        )
        .is_err());
    let wrong_mode = value(b"gamma beta", 0o700);
    assert_eq!(
        classify(&prepared, &wrong_mode, &ObservedTarget::Absent),
        Layout::ModeMismatch
    );
    Ok(())
}

#[test]
fn replacement_phases_are_durable_classified_and_cleanup_verified() -> TestResult {
    let root = fixture("replace")?;
    fs::write(root.join("target"), "prior")?;
    fs::set_permissions(root.join("target"), fs::Permissions::from_mode(0o640))?;
    let workspace = OpenedWorkspace::open(&root)?;
    let prepared = ok(workspace.prepare_exact_edit(
        "target".into(),
        Revision::Sha256(revision(&workspace, "target")?),
        "prior",
        "intended",
        0,
    ))?;
    ok(workspace.advance_exact_edit(&prepared, DurablePhase::Staged))?;
    let stage = root.join(&prepared.stage_identity);
    assert_eq!(fs::read(&stage)?, b"intended");
    assert_eq!(fs::read(root.join("target"))?, b"prior");
    assert_eq!(fs::metadata(&stage)?.permissions().mode() & 0o7777, 0o640);
    assert!(workspace
        .advance_exact_edit(&prepared, DurablePhase::Settled)
        .is_err());
    fs::set_permissions(&stage, fs::Permissions::from_mode(0o600))?;
    assert!(workspace
        .advance_exact_edit(&prepared, DurablePhase::Exchanged)
        .is_err());
    assert_eq!(fs::read(root.join("target"))?, b"prior");
    fs::set_permissions(&stage, fs::Permissions::from_mode(0o640))?;
    ok(workspace.advance_exact_edit(&prepared, DurablePhase::Exchanged))?;
    assert_eq!(fs::read(root.join("target"))?, b"intended");
    assert_eq!(fs::read(&stage)?, b"prior");
    fs::write(root.join("target"), "third-owner-value")?;
    assert!(workspace
        .advance_exact_edit(&prepared, DurablePhase::Compensated)
        .is_err());
    assert_eq!(fs::read(root.join("target"))?, b"third-owner-value");
    assert_eq!(fs::read(&stage)?, b"prior");
    assert!(workspace
        .cleanup_exact_edit(&prepared, VerifiedOutcome::Settled)
        .is_err());
    fs::write(root.join("target"), "intended")?;
    ok(workspace.advance_exact_edit(&prepared, DurablePhase::Settled))?;
    ok(workspace.cleanup_exact_edit(&prepared, VerifiedOutcome::Settled))?;
    assert!(!stage.exists());
    let mode = fs::metadata(root.join("target"))?.permissions().mode();
    assert_eq!(mode & 0o7777, 0o640);
    Ok(())
}

#[test]
fn creation_races_compensate_without_overwriting_owner_values() -> TestResult {
    let root = fixture("create")?;
    let workspace = OpenedWorkspace::open(&root)?;
    let prepared =
        ok(workspace.prepare_exact_edit("new".into(), Revision::Absent, "", "created", 0o750))?;
    ok(workspace.advance_exact_edit(&prepared, DurablePhase::Staged))?;
    fs::write(root.join("new"), "owner")?;
    assert!(workspace
        .advance_exact_edit(&prepared, DurablePhase::Exchanged)
        .is_err());
    assert_eq!(fs::read(root.join("new"))?, b"owner");
    fs::remove_file(root.join("new"))?;
    ok(workspace.advance_exact_edit(&prepared, DurablePhase::Exchanged))?;
    let mode = fs::metadata(root.join("new"))?.permissions().mode();
    assert_eq!(mode & 0o7777, 0o750);
    ok(workspace.advance_exact_edit(&prepared, DurablePhase::Compensated))?;
    assert!(!root.join("new").exists());
    assert_eq!(fs::read(root.join(&prepared.stage_identity))?, b"created");
    ok(workspace.cleanup_exact_edit(&prepared, VerifiedOutcome::Compensated))?;
    assert!(!root.join(&prepared.stage_identity).exists());
    Ok(())
}

fn revision(workspace: &OpenedWorkspace, path: &str) -> Result<String, std::io::Error> {
    match ok(workspace.observe_edit_target(path))? {
        ObservedTarget::Present(value) => Ok(value.revision),
        ObservedTarget::Absent => Err(std::io::Error::other("target absent")),
    }
}
fn value(bytes: &[u8], mode: u32) -> ObservedTarget {
    ObservedTarget::Present(FileValue {
        bytes: bytes.to_vec(),
        revision: String::new(),
        mode,
    })
}
fn ok<T>(result: EditResult<T>) -> Result<T, std::io::Error> {
    result.map_err(|error| std::io::Error::other(format!("{error:?}")))
}
fn fixture(name: &str) -> Result<PathBuf, std::io::Error> {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("lkjagent-edit-{name}-{}-{id}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
