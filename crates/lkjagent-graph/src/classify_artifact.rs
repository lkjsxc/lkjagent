use crate::case_document::DocumentState;
use crate::classify_signals::content_artifact_request;
use crate::model::TaskFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RouteSpec {
    pub subroute: String,
    pub route_reason: String,
    pub document: Option<DocumentState>,
}

pub(crate) fn route_spec(family: TaskFamily, objective: &str) -> RouteSpec {
    let lower = objective.to_ascii_lowercase();
    let content_artifact = content_artifact_request(&lower, objective);
    RouteSpec {
        subroute: subroute_for(family, content_artifact).to_string(),
        route_reason: route_reason_for(family, content_artifact).to_string(),
        document: document_state_for(family, objective, &lower, content_artifact),
    }
}

fn document_state_for(
    family: TaskFamily,
    objective: &str,
    lower: &str,
    content_artifact: bool,
) -> Option<DocumentState> {
    if !matches!(
        family,
        TaskFamily::Documentation | TaskFamily::KnowledgeBase
    ) {
        return None;
    }
    if content_artifact {
        let kind = artifact_kind(lower);
        return Some(DocumentState::planned(
            artifact_root(kind, objective),
            "content-artifact",
        ));
    }
    Some(DocumentState::planned("structured-output", "document"))
}

fn subroute_for(family: TaskFamily, content_artifact: bool) -> &'static str {
    match family {
        TaskFamily::Documentation | TaskFamily::KnowledgeBase if content_artifact => {
            "content-artifact"
        }
        TaskFamily::Documentation | TaskFamily::KnowledgeBase => "document-construction",
        TaskFamily::CodeChange | TaskFamily::BugFix => "code-change",
        TaskFamily::Architecture => "architecture-change",
        TaskFamily::Verification => "verification",
        TaskFamily::Compaction => "compaction",
        TaskFamily::Recovery => "recovery",
        TaskFamily::Benchmark => "benchmark",
        TaskFamily::IdleMaintenance => "idle-maintenance",
        TaskFamily::Maintenance => "maintenance",
    }
}

fn route_reason_for(family: TaskFamily, content_artifact: bool) -> &'static str {
    match family {
        TaskFamily::Documentation | TaskFamily::KnowledgeBase if content_artifact => {
            "long content deliverable requires semantic artifact construction"
        }
        TaskFamily::Documentation | TaskFamily::KnowledgeBase => {
            "counted or document-shaped deliverable"
        }
        TaskFamily::CodeChange | TaskFamily::BugFix => "implementation/fix wording preempts docs",
        TaskFamily::Architecture => "architecture wording with code/doc drift risk",
        TaskFamily::Verification => "verification or test wording",
        TaskFamily::Compaction => "context pressure or compaction wording",
        TaskFamily::Recovery => "recovery/failure wording",
        TaskFamily::Benchmark => "benchmark wording",
        TaskFamily::IdleMaintenance => "empty-queue maintenance",
        TaskFamily::Maintenance => "maintenance or cleanup wording",
    }
}

fn artifact_kind(lower: &str) -> &'static str {
    if lower.contains("dictionary") || lower.contains("glossary") || lower.contains("lexicon") {
        "dictionary"
    } else if lower.contains("cookbook") || lower.contains("recipe") || lower.contains("bread") {
        "cookbook"
    } else if lower.contains("story")
        || lower.contains("novel")
        || lower.contains("manuscript")
        || lower.contains("narrative")
    {
        "story"
    } else {
        "artifact"
    }
}

fn artifact_root(kind: &str, objective: &str) -> String {
    let parent = match kind {
        "dictionary" => "dictionaries",
        "cookbook" => "cookbooks",
        "story" => "stories",
        _ => "artifacts",
    };
    format!("{parent}/{}", artifact_slug(objective))
}

fn artifact_slug(objective: &str) -> String {
    let words = objective
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .filter(|word| !word.is_empty() && !STOP_WORDS.contains(&word.as_str()))
        .take(6)
        .collect::<Vec<_>>();
    if words.is_empty() {
        "content-artifact".to_string()
    } else {
        words.join("-")
    }
}

const STOP_WORDS: &[&str] = &[
    "a", "an", "the", "create", "write", "make", "generate", "produce", "big", "large", "very",
];
