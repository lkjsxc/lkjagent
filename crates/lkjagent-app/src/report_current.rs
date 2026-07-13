use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_effects::workspace_edit::ObservedTarget;
use lkjagent_store::error::StoreResult;
use rusqlite::{params, Transaction};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(crate) fn all_match(
    tx: &Transaction<'_>,
    workspace: &OpenedWorkspace,
    parameters: &[u8],
) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(parameters) else {
        return false;
    };
    value["paths"].as_array().is_some_and(|paths| {
        paths.iter().all(|path| {
            path.as_str()
                .is_some_and(|path| path_matches(tx, workspace, path))
        })
    })
}

pub(crate) fn reopen_stale(
    tx: &Transaction<'_>,
    workspace: &OpenedWorkspace,
    matter: &str,
    event: &str,
    slug: &str,
    children: &[&str],
) -> StoreResult<(bool, BTreeSet<String>)> {
    let map = format!("artifacts/documents/{slug}/README.md");
    let map_stale = !path_matches(tx, workspace, &map);
    if map_stale {
        reopen(tx, matter, event, &format!("report-map/{slug}"))?;
    }
    let mut stale = BTreeSet::new();
    for unit in children {
        let path = format!("artifacts/documents/{slug}/{unit}.md");
        if !path_matches(tx, workspace, &path) {
            reopen(tx, matter, event, &format!("report-member/{slug}/{unit}"))?;
            stale.insert((*unit).to_string());
        }
    }
    Ok((map_stale, stale))
}

fn reopen(tx: &Transaction<'_>, matter: &str, event: &str, suffix: &str) -> StoreResult<()> {
    let obligation = format!("{matter}/{suffix}");
    tx.execute(
        "UPDATE checks SET current=0 WHERE obligation_id=?1 AND current=1",
        [&obligation],
    )?;
    tx.execute(
        "UPDATE obligations SET status='open',current_check_id=NULL,invalidating_event_id=?1
         WHERE id=?2 AND matter_id=?3 AND (status='passed' OR current_check_id IS NOT NULL)",
        params![event, obligation, matter],
    )?;
    Ok(())
}

pub(crate) fn path_matches(tx: &Transaction<'_>, workspace: &OpenedWorkspace, path: &str) -> bool {
    let row = tx.query_row(
        "SELECT r.content,t.intended_mode,r.sha256 FROM workspace_documents d
         JOIN workspace_revisions r ON r.id=d.current_revision_id
         JOIN effect_targets t ON t.journal_id=r.effect_id AND t.ordinal=0
         WHERE d.current_path=?1 AND d.managed=1 AND d.status='active'",
        [path.as_bytes()],
        |row| {
            Ok((
                row.get::<_, Vec<u8>>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        },
    );
    let Ok((content, mode, revision)) = row else {
        return false;
    };
    let Ok(observed) = workspace.observe_edit_target(path) else {
        return false;
    };
    match observed {
        ObservedTarget::Present(target) => {
            target.bytes == content
                && i64::from(target.mode) == mode
                && Sha256::digest(&target.bytes).as_slice() == revision
        }
        ObservedTarget::Absent => false,
    }
}
