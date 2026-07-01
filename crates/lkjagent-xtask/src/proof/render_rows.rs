use super::model::{
    CaseRow, ContractRow, CountRow, DecisionRow, EventRow, FileRow, QueueRow, ReadinessRow,
    WordCountRow,
};

pub fn case_rows(rows: &[CaseRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.id.to_string(),
                row.family.clone(),
                row.phase.clone(),
                row.node.clone(),
                row.status.clone(),
                row.next_action.clone(),
                row.objective_chars.to_string(),
            ]
        })
        .collect()
}

pub fn count_rows(rows: &[CountRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| vec![row.name.clone(), row.count.to_string()])
        .collect()
}

pub fn queue_rows(rows: &[QueueRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.id.to_string(),
                row.status.clone(),
                row.delivered_turn.clone(),
                row.content_chars.to_string(),
                row.created_at.clone(),
            ]
        })
        .collect()
}

pub fn readiness_rows(rows: &[ReadinessRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.plan_id.to_string(),
                row.root.clone(),
                row.profile.clone(),
                row.status.clone(),
                row.atoms.clone(),
                format!("{}/{}", row.measured, row.floor),
                row.active_contract.clone(),
                row.next_path.clone(),
                row.blockers.clone(),
            ]
        })
        .collect()
}

pub fn contract_rows(rows: &[ContractRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.id.clone(),
                row.root.clone(),
                row.status.clone(),
                row.limits.clone(),
                row.paths.clone(),
            ]
        })
        .collect()
}

pub fn decision_rows(rows: &[DecisionRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.id.to_string(),
                row.mission.clone(),
                row.mode.clone(),
                row.node.clone(),
                row.next_action.clone(),
                row.completion.clone(),
                row.created_at.clone(),
            ]
        })
        .collect()
}

pub fn event_rows(rows: &[EventRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.id.to_string(),
                row.turn.clone(),
                row.kind.clone(),
                row.tokens.to_string(),
                row.created_at.clone(),
            ]
        })
        .collect()
}

pub fn word_count_rows(rows: &[WordCountRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            vec![
                row.root.clone(),
                row.files.to_string(),
                row.words.to_string(),
                row.manuscript_files.to_string(),
                row.manuscript_words.to_string(),
            ]
        })
        .collect()
}

pub fn file_rows(rows: &[FileRow]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| vec![row.path.clone(), row.bytes.to_string()])
        .collect()
}
