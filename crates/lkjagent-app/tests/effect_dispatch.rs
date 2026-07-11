use std::fs;

use lkjagent_app::effect_dispatch::dispatch_effects;
use lkjagent_core::classify::instantiate;
use lkjagent_core::engine::Command;
use lkjagent_core::parse::Action;
use lkjagent_core::runtime_fingerprint::stable_fingerprint;
use lkjagent_store::admission_rows::{EffectTargetRevision, PreparedEffect};
use rusqlite::Connection;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn prior_failure_reports_completed_effects() -> TestResult<()> {
    let workspace = std::env::temp_dir().join(format!("lkjagent-prior-{}", std::process::id()));
    fs::create_dir_all(&workspace)?;
    fs::write(workspace.join("note.md"), "changed")?;
    let mut snapshot = instantiate(1, "write note");
    let conn = Connection::open_in_memory()?;
    let first = PreparedEffect {
        admission_id: "first-admission".to_string(),
        journal_id: "first-journal".to_string(),
        command_ordinal: 1,
        target_path: None,
        prior_fingerprint: String::new(),
        intended_fingerprint: String::new(),
        effect_name: "plan.note".to_string(),
        targets: Vec::new(),
    };
    let effect = PreparedEffect {
        admission_id: "admission".to_string(),
        journal_id: "journal".to_string(),
        command_ordinal: 2,
        target_path: Some("note.md".to_string()),
        prior_fingerprint: stable_fingerprint(&Some(b"before".to_vec()))
            .map_err(|error| error.message)?,
        intended_fingerprint: String::new(),
        effect_name: "native.write_file".to_string(),
        targets: Vec::new(),
    };
    let commands = [
        Command::RunExplore(Action {
            tool: "plan.note".to_string(),
            params: vec![("note".to_string(), "done".to_string())],
        }),
        Command::WriteFile {
            path: "note.md".to_string(),
            content: "after".to_string(),
        },
    ];
    let failure = dispatch_effects(
        &conn,
        &workspace,
        &mut snapshot,
        &commands,
        &[first, effect],
    )
    .err()
    .ok_or("changed prior was dispatched")?;
    assert_eq!(failure.completed, 1);
    assert!(failure.failed_current);
    assert_eq!(fs::read_to_string(workspace.join("note.md"))?, "changed");
    Ok(())
}

#[test]
fn partial_bundle_failure_restores_prior_targets() -> TestResult<()> {
    let workspace = std::env::temp_dir().join(format!("lkjagent-rollback-{}", std::process::id()));
    if workspace.exists() {
        fs::remove_dir_all(&workspace)?;
    }
    fs::create_dir_all(&workspace)?;
    fs::write(workspace.join("blocked"), "owner")?;
    let targets = vec![
        revision(1, "first.md", None, Some(b"first"))?,
        revision(2, "blocked/second.md", None, Some(b"second"))?,
    ];
    let effect = PreparedEffect {
        admission_id: "admission".to_string(),
        journal_id: "journal".to_string(),
        command_ordinal: 1,
        target_path: Some("first.md".to_string()),
        prior_fingerprint: targets[0].prior_fingerprint.clone(),
        intended_fingerprint: targets[0].intended_fingerprint.clone(),
        effect_name: "native.write_file".to_string(),
        targets,
    };
    let mut snapshot = instantiate(1, "write bundle");
    let conn = Connection::open_in_memory()?;
    let command = Command::WriteFile {
        path: "first.md".to_string(),
        content: "ignored".to_string(),
    };
    assert!(dispatch_effects(&conn, &workspace, &mut snapshot, &[command], &[effect]).is_err());
    assert!(!workspace.join("first.md").exists());
    assert_eq!(fs::read_to_string(workspace.join("blocked"))?, "owner");
    Ok(())
}

fn revision(
    target_ordinal: i64,
    path: &str,
    prior: Option<&[u8]>,
    intended: Option<&[u8]>,
) -> TestResult<EffectTargetRevision> {
    let prior_bytes = prior.map(<[u8]>::to_vec);
    let intended_bytes = intended.map(<[u8]>::to_vec);
    Ok(EffectTargetRevision {
        target_ordinal,
        role: "file".to_string(),
        path: path.to_string(),
        prior_fingerprint: stable_fingerprint(&prior_bytes).map_err(|error| error.message)?,
        intended_fingerprint: stable_fingerprint(&intended_bytes).map_err(|error| error.message)?,
        prior_bytes,
        intended_bytes,
        artifacts: Vec::new(),
    })
}
