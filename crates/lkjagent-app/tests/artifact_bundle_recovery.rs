use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::runtime_admission::{AdmissionStatus, ToolAdmission};
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{
    insert_admission_and_prepare, mark_journal, EffectPreparation, EffectTargetRevision,
};
use lkjagent_store::artifact_rows::ArtifactRow;
use lkjagent_store::effect_recovery::recover_unsettled_effects;
use lkjagent_store::plan_schema::setup;
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn complete_bundle_recovery_settles_artifacts_and_refs() -> TestResult<()> {
    let data = fixture_root("complete")?;
    let workspace = data.join("workspace");
    fs::create_dir_all(workspace.join("out.parts"))?;
    fs::write(workspace.join("out.parts/part-999.md"), "stale")?;
    let mut targets = bundle_targets()?;
    targets.push(target(
        5,
        "stale-part",
        "out.parts/part-999.md",
        Some(b"stale"),
        None,
        Vec::new(),
    )?);
    let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
    prepare(&conn, "complete", &targets)?;
    fs::remove_file(workspace.join("out.parts/part-999.md"))?;
    write_intended(&workspace, &targets)?;

    assert_eq!(
        recover_unsettled_effects(&mut conn, &workspace, "restart")?,
        1
    );
    let state: String = conn.query_row(
        "SELECT state FROM effect_journal WHERE id = 'complete-journal'",
        [],
        |row| row.get(0),
    )?;
    let artifacts: i64 = conn.query_row("SELECT COUNT(*) FROM artifacts", [], |row| row.get(0))?;
    let refs: String = conn.query_row(
        "SELECT artifact_refs_json FROM observations WHERE id = 'complete-journal-recovery-observation'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(state, "recovered");
    assert_eq!(artifacts, 3);
    assert_eq!(refs, "[\"artifact-parent\"]");
    assert!(!workspace.join("out.parts/part-999.md").exists());
    Ok(())
}

#[test]
fn partial_and_conflicting_bundles_fail_without_artifacts() -> TestResult<()> {
    for (name, count, conflict) in [
        ("first-part", 2, false),
        ("parts-only", 3, false),
        ("conflict", 4, true),
        ("extra-part", 4, false),
    ] {
        let data = fixture_root(name)?;
        let workspace = data.join("workspace");
        let targets = bundle_targets()?;
        let mut conn = Connection::open(data.join("lkjagent.sqlite3"))?;
        prepare(&conn, name, &targets)?;
        write_intended(&workspace, &targets[..count])?;
        if conflict {
            fs::write(workspace.join("out.parts/part-002.md"), "owner bytes")?;
        }
        if name == "extra-part" {
            fs::write(workspace.join("out.parts/part-999.md"), "unexpected")?;
        }
        assert!(recover_unsettled_effects(&mut conn, &workspace, "restart").is_err());
        let state: String = conn.query_row(
            "SELECT state FROM effect_journal WHERE id = ?1",
            [format!("{name}-journal")],
            |row| row.get(0),
        )?;
        let artifacts: i64 =
            conn.query_row("SELECT COUNT(*) FROM artifacts", [], |row| row.get(0))?;
        assert_eq!(state, "applying");
        assert_eq!(artifacts, 0);
        if conflict {
            assert_eq!(
                fs::read_to_string(workspace.join("out.parts/part-002.md"))?,
                "owner bytes"
            );
        }
    }
    Ok(())
}

#[rustfmt::skip]
fn bundle_targets() -> TestResult<Vec<EffectTargetRevision>> {
    Ok(vec![
        target(1, "parts-membership", "out.parts", Some(b""),
            Some(b"out.parts/part-001.md\nout.parts/part-002.md"), Vec::new())?,
        target(2, "part", "out.parts/part-001.md", None, Some(b"part one"),
            vec![artifact("artifact-unit-1", "out.parts/part-001.md", Some("artifact-parent"))])?,
        target(3, "part", "out.parts/part-002.md", None, Some(b"part two"),
            vec![artifact("artifact-unit-2", "out.parts/part-002.md", Some("artifact-parent"))])?,
        target(4, "main", "out.md", None, Some(b"manifest"), vec![artifact("artifact-parent", "out.md", None)])?,
    ])
}

fn target(
    target_ordinal: i64,
    role: &str,
    path: &str,
    prior: Option<&[u8]>,
    intended: Option<&[u8]>,
    artifacts: Vec<ArtifactRow>,
) -> TestResult<EffectTargetRevision> {
    let prior_bytes = prior.map(<[u8]>::to_vec);
    let intended_bytes = intended.map(<[u8]>::to_vec);
    Ok(EffectTargetRevision {
        target_ordinal,
        role: role.to_string(),
        path: path.to_string(),
        prior_fingerprint: stable_fingerprint(&prior_bytes).map_err(|error| error.message)?,
        intended_fingerprint: stable_fingerprint(&intended_bytes).map_err(|error| error.message)?,
        prior_bytes,
        intended_bytes,
        artifacts,
    })
}

fn artifact(id: &str, path: &str, parent: Option<&str>) -> ArtifactRow {
    ArtifactRow {
        id: id.to_string(),
        case_id: "1".to_string(),
        kind: if parent.is_some() { "unit" } else { "file" }.to_string(),
        path: path.to_string(),
        fingerprint: format!("fingerprint-{id}"),
        parent_artifact_id: parent.map(str::to_string),
        metadata_json: "{}".to_string(),
        created_at: "now".to_string(),
    }
}

fn prepare(conn: &Connection, id: &str, targets: &[EffectTargetRevision]) -> TestResult<()> {
    let admission = ToolAdmission {
        decision_id: format!("{id}-decision"),
        tool_view_fingerprint: "view".to_string(),
        action_tool: "native.write_file".to_string(),
        status: AdmissionStatus::Admitted,
        reason: "harness admitted".to_string(),
    };
    insert_admission_and_prepare(
        conn,
        &EffectPreparation {
            id: &format!("{id}-admission"),
            case_id: "1",
            admission: &admission,
            parsed_action_json: "{}",
            journal_id: &format!("{id}-journal"),
            idempotency_key: &format!("{id}-key"),
            command_ordinal: 1,
            target_path: Some("out.md"),
            prior_fingerprint: "legacy-prior",
            intended_fingerprint: "legacy-intended",
            targets,
            created_at: "now",
        },
    )?;
    mark_journal(conn, &format!("{id}-journal"), "applying", "now")?;
    Ok(())
}

fn write_intended(workspace: &Path, targets: &[EffectTargetRevision]) -> TestResult<()> {
    for target in targets {
        if target.role == "parts-membership" {
            continue;
        }
        if let Some(bytes) = &target.intended_bytes {
            let path = workspace.join(&target.path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, bytes)?;
        }
    }
    Ok(())
}

#[rustfmt::skip]
fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-artifact-bundle-{name}-{}", std::process::id()));
    if path.exists() { fs::remove_dir_all(&path)?; }
    fs::create_dir_all(path.join("workspace"))?;
    let conn = Connection::open(path.join("lkjagent.sqlite3"))?;
    setup(&conn)?;
    Ok(path)
}
