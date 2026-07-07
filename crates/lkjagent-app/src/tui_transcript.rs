use std::path::{Path, PathBuf};

use crate::tui_snapshot::TuiSnapshot;
use crate::tui_types::{pane_label, run_state_label, source_label, TuiModel};

pub fn save(data_dir: &Path, model: &TuiModel, snapshot: &TuiSnapshot) -> Result<PathBuf, String> {
    let dir = data_dir.join("workspace/tui-transcripts");
    std::fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let path = dir.join(format!("{}.txt", stamp()));
    std::fs::write(&path, text(model, snapshot)).map_err(|error| error.to_string())?;
    Ok(path)
}

pub fn text(model: &TuiModel, snapshot: &TuiSnapshot) -> String {
    let entries = transcript_entries(model, snapshot);
    [
        "# lkjagent tui transcript".to_string(),
        format!("state: {}", run_state_label(model.run_state)),
        format!("pane: {}", pane_label(model.active_pane)),
        format!(
            "search: {}",
            if model.search.is_empty() {
                "none"
            } else {
                &model.search
            }
        ),
        "## status".to_string(),
        snapshot.status.clone(),
        "## queue".to_string(),
        snapshot.queue.clone(),
        "## transcript".to_string(),
        entries,
    ]
    .join("\n")
}

fn transcript_entries(model: &TuiModel, snapshot: &TuiSnapshot) -> String {
    let mut lines = snapshot
        .transcript
        .lines()
        .map(str::to_string)
        .collect::<Vec<_>>();
    for entry in &model.transcript {
        let line = format!("{}: {}", source_label(entry.source), entry.text.trim());
        if !lines.iter().any(|existing| existing == &line) {
            lines.push(line);
        }
    }
    if lines.is_empty() {
        "[no transcript entries]".to_string()
    } else {
        lines.join("\n")
    }
}

fn stamp() -> String {
    crate::clock::utc_now().replace(':', "").replace('.', "-")
}
