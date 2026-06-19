use crate::count_profile::DeliverableKind;

pub(crate) fn detect_kind(objective: &str) -> DeliverableKind {
    let lower = objective.to_lowercase();
    if contains_any(
        &lower,
        &[
            "story",
            "novel",
            "fiction",
            "narrative",
            "manuscript",
            "screenplay",
            "script",
            "saga",
            "tale",
            "物語",
            "小説",
            "脚本",
        ],
    ) {
        DeliverableKind::Narrative
    } else if contains_any(
        &lower,
        &[
            "guide",
            "manual",
            "tutorial",
            "procedure",
            "playbook",
            "runbook",
            "handbook",
            "sop",
            "手順",
            "説明書",
            "運用書",
        ],
    ) {
        DeliverableKind::Guide
    } else if contains_any(
        &lower,
        &[
            "report",
            "analysis",
            "research",
            "dossier",
            "whitepaper",
            "study",
            "briefing",
            "調査",
            "分析",
            "報告",
        ],
    ) {
        DeliverableKind::Report
    } else {
        DeliverableKind::General
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}
