use std::fs;
use std::path::Path;

pub fn graph_state_recovery(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace, &["transcript.md", "run.log"])?;
    require_all(
        &text,
        &[
            "action params refused",
            "tool=graph.state",
            "expected=no parameters",
            "<tool>graph.state</tool>\n</act>",
            "recover-params",
        ],
    )?;
    forbid(&text, "<path>.</path>\n</act>")
}

pub fn doc_scaffold_recovery(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace, &["transcript.md", "run.log"])?;
    if text.contains("action params normalized") {
        require_all(&text, &["tool=doc.scaffold", "path->root"])?;
    } else {
        require_all(
            &text,
            &[
                "action params refused",
                "tool=doc.scaffold",
                "<root>VALUE</root>",
            ],
        )?;
    }
    forbid(&text, "<path>docs</path>\n</act>")
}

pub fn status_accounting(workspace: &Path) -> Result<(), String> {
    let text = read_file(&workspace.join("status.txt"))?;
    require_all(
        &text,
        &["ctx=", "pressure=", "in=", "out=", "cache=", "total="],
    )?;
    require_any(&text, &["K", "M", "unknown"])?;
    if !text.contains('%') {
        return Err("status missing context percentage".to_string());
    }
    Ok(())
}

pub fn gpt_handoff_log(workspace: &Path) -> Result<(), String> {
    let path = workspace.join("data/logs/current-gpt-5.5-pro.md");
    let text = read_file(&path)?;
    require_all(
        &text,
        &[
            "# lkjagent GPT-5.5-Pro Run Log",
            "## Snapshot",
            "## Owner Objective",
            "## Active State Tracks",
            "## Fault Ledger",
            "## Verification",
            "token_usage:",
        ],
    )
}

fn read_any(workspace: &Path, paths: &[&str]) -> Result<String, String> {
    for path in paths {
        let candidate = workspace.join(path);
        if candidate.is_file() {
            return read_file(&candidate);
        }
    }
    Err(format!("none of {paths:?} exists"))
}

fn read_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("{} unreadable: {error}", path.display()))
}

fn require_all(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if !text.contains(needle) {
            return Err(format!("missing {needle}"));
        }
    }
    Ok(())
}

fn require_any(text: &str, needles: &[&str]) -> Result<(), String> {
    if needles.iter().any(|needle| text.contains(needle)) {
        Ok(())
    } else {
        Err(format!("missing any of {needles:?}"))
    }
}

fn forbid(text: &str, needle: &str) -> Result<(), String> {
    if text.contains(needle) {
        Err(format!("forbidden stale shape {needle}"))
    } else {
        Ok(())
    }
}
