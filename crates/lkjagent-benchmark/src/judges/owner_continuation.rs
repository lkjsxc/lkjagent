use std::fs;
use std::path::Path;

pub fn turn_budget_checkpoint(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "checkpoint_event=TurnBudgetCheckpoint",
            "continuation_decision=continue-owner-execution",
            "exact_next_action=",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "Turn budget exhausted. Send guidance to continue.",
            "Should I continue?",
            "Do you want me to proceed?",
            "daemon_state=waiting",
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
            return Err(format!("forbidden budget stop {needle}"));
        }
    }
    Ok(())
}
