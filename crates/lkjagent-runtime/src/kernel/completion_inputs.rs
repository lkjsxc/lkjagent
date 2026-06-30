use crate::kernel::completion::CompletionGateDecision;

pub(crate) fn completion_inputs(completion: &CompletionGateDecision) -> Vec<String> {
    let input = &completion.input;
    vec![
        format!("objective_present={}", input.objective_present),
        format!("artifact_required={}", input.artifact_required),
        format!("artifact_ready={}", input.artifact_ready),
        format!("weak_paths={}", input.weak_paths.join("|")),
        format!("manuscript_active={}", input.manuscript_active),
        format!(
            "manuscript_words_written={}",
            input.manuscript_words_written
        ),
        format!("manuscript_word_floor={}", input.manuscript_word_floor),
        format!(
            "manuscript_target_words={}",
            input
                .manuscript_target_words
                .map(|words| words.to_string())
                .unwrap_or_else(|| "none".to_string())
        ),
        format!(
            "manuscript_chapter_count={}",
            input
                .manuscript_chapter_count
                .map(|count| count.to_string())
                .unwrap_or_else(|| "none".to_string())
        ),
        format!(
            "missing_manuscript_paths={}",
            input.missing_manuscript_paths.join("|")
        ),
        format!(
            "next_manuscript_path={}",
            input.next_manuscript_path.as_deref().unwrap_or("none")
        ),
        format!("content_atom_active={}", input.content_atom_active),
        format!(
            "content_atom_missing_count={}",
            input.content_atom_missing_count
        ),
        format!(
            "next_content_atom={}",
            input.next_content_atom.as_deref().unwrap_or("none")
        ),
        format!("fingerprint={}", input.decision_fingerprint),
    ]
}
