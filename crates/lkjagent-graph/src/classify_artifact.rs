use crate::case_document::DocumentState;
use crate::classify_profile::intent_facts;
use crate::classify_signals::content_artifact_request;
use crate::classify_title::{owner_title, owner_title_alias};
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
        let profile = intent_facts(lower, objective);
        let kind = profile.artifact_kind.as_deref().unwrap_or("artifact");
        return Some(
            DocumentState::planned(artifact_root(kind, objective), "content-artifact")
                .with_identity(
                    owner_title(objective),
                    Some(kind.to_string()),
                    profile.requested_scale,
                ),
        );
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

fn artifact_root(kind: &str, objective: &str) -> String {
    let parent = match kind {
        "dictionary" => "dictionaries",
        "cookbook" => "cookbooks",
        "story" => "stories",
        _ => "artifacts",
    };
    format!("{parent}/{}", artifact_alias(kind, objective))
}

fn artifact_alias(kind: &str, objective: &str) -> String {
    if let Some(alias) = owner_title_alias(objective) {
        return alias;
    }
    let base = base_alias(kind, objective);
    let Some(qualifier) = first_qualifier(base, objective) else {
        return base.to_string();
    };
    if matches!(kind, "cookbook" | "dictionary") {
        return format!("{qualifier}-{base}");
    }
    format!("{base}-{qualifier}")
}

fn base_alias<'a>(kind: &str, objective: &'a str) -> &'a str {
    let lower = objective.to_ascii_lowercase();
    match kind {
        "dictionary" => "dictionary",
        "cookbook" => "cookbook",
        "story" if lower.contains("novel") => "novel",
        "story" => "story",
        _ => "artifact",
    }
}

fn first_qualifier(base: &str, objective: &str) -> Option<String> {
    objective
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .find(|word| qualifies(word, base))
}

fn qualifies(word: &str, base: &str) -> bool {
    !word.is_empty() && word != base && !STOP_WORDS.contains(&word)
}

const STOP_WORDS: &[&str] = &[
    "a",
    "an",
    "the",
    "create",
    "write",
    "make",
    "generate",
    "produce",
    "big",
    "large",
    "very",
    "long",
    "structured",
    "settings",
    "setting",
    "with",
    "detailed",
    "sf",
    "story",
    "named",
    "called",
    "titled",
];
