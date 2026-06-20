use std::fs;
use std::path::Path;

pub fn graph_plan_example(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(&text, &["<tool>graph.plan</tool>", "<checks>", "<reason>"])?;
    forbid_any(
        &text,
        &[
            "graph.plan needs checks or paths",
            "<tool>graph.next</tool>",
        ],
    )
}

pub fn memory_fts_query(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(&text, &["memory.find", "query_normalized=graph note"])?;
    require_all(&text, &["query_normalized=parameter fault"])?;
    forbid_any(&text, &["fts5: syntax error", "store error: sqlite error"])
}

pub fn maintenance_memory_duplicate(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "memory.save kind=lesson",
            "SkipDuplicate",
            "existing_id=",
            "cooldown_set=true",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "duplicate memory rows",
            "What stale memory rows",
            "What recent transcript spans",
        ],
    )
}

pub fn policy_contradiction(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "active_mode=Maintenance",
            "active_mode=Compaction",
            "hard_compaction=runtime-owned",
        ],
    )?;
    if text.matches("graph_policy=disabled").count() < 2 {
        return Err("missing disabled graph policy per mode".to_string());
    }
    forbid_any(
        &text,
        &[
            "maintenance only allows",
            "compaction only allows memory.save",
            "graph policy refused",
        ],
    )
}

pub fn graph_note_kind_recovery(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "graph.note kind=decision",
            "normalized_from=planning",
            "graph.note kind=success",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "graph.note kind=planning",
            "graph.note kind=progress",
            "graph.note kind=note",
            "graph.note kind=evidence",
            "graph.note kind=recovery",
            "graph.evidence kind=decision",
        ],
    )?;
    if text.matches("graph.next").count() > 1 {
        return Err("repeated graph.next diagnostic".to_string());
    }
    Ok(())
}

pub fn bread_cookbook_artifact(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "subroute=content-artifact",
            "root=cookbooks/bread-cookbook",
            "profile=Cookbook",
            "foundations/flour-water-salt-yeast.md",
            "recipes/sourdough-country-loaf.md",
            "content-bearing files verified",
            "doc.audit passed",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "root=docs/bread",
            "GenericProjectDocs",
            "agent.done all evidence requirements met",
            "agent.done scaffold only",
            "audit=Missing",
        ],
    )
}

fn read_any(workspace: &Path) -> Result<String, String> {
    for path in ["transcript.md", "run.log"] {
        let candidate = workspace.join(path);
        if candidate.is_file() {
            return fs::read_to_string(&candidate)
                .map_err(|error| format!("{} unreadable: {error}", candidate.display()));
        }
    }
    Err("none of transcript.md or run.log exists".to_string())
}

fn require_all(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if !text.contains(needle) {
            return Err(format!("missing {needle}"));
        }
    }
    Ok(())
}

fn forbid_any(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if text.contains(needle) {
            return Err(format!("forbidden stale shape {needle}"));
        }
    }
    Ok(())
}
