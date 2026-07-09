use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::tui_snapshot::TuiSnapshot;
use crate::tui_types::{
    pane_label, run_state_label, source_label, TranscriptEntry, TranscriptSource, TuiModel,
    TuiRunState,
};

pub fn save(data_dir: &Path, model: &TuiModel, snapshot: &TuiSnapshot) -> Result<PathBuf, String> {
    let dir = crate::config::workspace_root(data_dir)?.join("tui-transcripts");
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

pub fn display_lines(model: &TuiModel, snapshot: &TuiSnapshot) -> Vec<String> {
    merged_entries(model, snapshot)
        .into_iter()
        .filter(conversation_entry)
        .map(|entry| format!("{}: {}", source_label(entry.source), entry.text.trim()))
        .collect()
}

pub fn merged_entries(model: &TuiModel, snapshot: &TuiSnapshot) -> Vec<TranscriptEntry> {
    let mut seen = BTreeSet::new();
    let mut shadows = BTreeMap::new();
    let mut entries = Vec::new();
    for entry in &snapshot.transcript_entries {
        if seen.insert(entry.id.clone()) {
            *shadows.entry(shadow_key(entry)).or_insert(0) += 1;
            entries.push(entry.clone());
        }
    }
    for entry in &model.transcript {
        if session_local(entry) && consume_shadow(&mut shadows, entry) {
            continue;
        }
        if seen.insert(entry.id.clone()) {
            entries.push(entry.clone());
        }
    }
    if let Some(draft) = &model.agent_draft {
        if session_local(draft) && consume_shadow(&mut shadows, draft) {
            return entries;
        }
        if seen.insert(draft.id.clone()) {
            entries.push(draft.clone());
        }
    }
    entries
}

fn conversation_entry(entry: &TranscriptEntry) -> bool {
    matches!(
        entry.source,
        TranscriptSource::Owner | TranscriptSource::Agent
    )
}

fn consume_shadow(shadows: &mut BTreeMap<String, usize>, entry: &TranscriptEntry) -> bool {
    let Some(count) = shadows.get_mut(&shadow_key(entry)) else {
        return false;
    };
    if *count == 0 {
        return false;
    }
    *count -= 1;
    true
}

fn session_local(entry: &TranscriptEntry) -> bool {
    entry.id.contains(":session:") || entry.id == "draft-agent"
}

fn shadow_key(entry: &TranscriptEntry) -> String {
    format!("{}:{}", source_label(entry.source), entry.text.trim())
}

fn transcript_entries(model: &TuiModel, snapshot: &TuiSnapshot) -> String {
    let lines = merged_entries(model, snapshot)
        .into_iter()
        .map(|entry| saved_line(&entry))
        .collect::<Vec<_>>();
    if lines.is_empty() {
        "[no transcript entries]".to_string()
    } else {
        lines.join("\n")
    }
}

fn saved_line(entry: &TranscriptEntry) -> String {
    let path = entry.path.as_deref().unwrap_or("session");
    format!(
        "[id={} path={}] {}: {}",
        entry.id,
        path,
        source_label(entry.source),
        entry.text.trim()
    )
}

pub(crate) fn append_agent_draft(model: &mut TuiModel, text: &str) {
    match &mut model.agent_draft {
        Some(draft) => draft.text.push_str(text),
        None => {
            model.agent_draft = Some(TranscriptEntry {
                id: "draft-agent".to_string(),
                source: TranscriptSource::Agent,
                text: text.to_string(),
                path: None,
            });
        }
    }
    if model.follow {
        model.scroll = 0;
    }
}

pub(crate) fn complete_agent_draft(model: &mut TuiModel) {
    model.run_state = TuiRunState::Idle;
    if let Some(mut draft) = model.agent_draft.take() {
        draft.id = next_id(model, "agent");
        model.transcript.push(draft);
    }
}

pub(crate) fn push_entry(model: &mut TuiModel, source: TranscriptSource, text: impl Into<String>) {
    let prefix = source_label(source);
    let entry = TranscriptEntry {
        id: next_id(model, prefix),
        source,
        text: text.into(),
        path: None,
    };
    model.transcript.push(entry);
    if model.follow {
        model.scroll = 0;
    }
}

fn next_id(model: &mut TuiModel, prefix: &str) -> String {
    let id = format!("session:{}", model.next_entry_seq);
    model.next_entry_seq = model.next_entry_seq.saturating_add(1);
    format!("{prefix}:{id}")
}

fn stamp() -> String {
    crate::clock::utc_now().replace(':', "").replace('.', "-")
}
