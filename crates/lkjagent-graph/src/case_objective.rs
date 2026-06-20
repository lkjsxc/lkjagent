use crate::case_fields::{
    ConstraintRecord, FieldStatus, IntentHypothesis, PreferenceRecord, QuestionRecord, RiskRecord,
};
use crate::state_track::{StateTrack, StateTrackId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveState {
    pub raw_owner_message: String,
    pub normalized: String,
    pub version: u32,
    pub non_goals: Vec<String>,
    pub owner_constraints: Vec<String>,
    pub envelope: ObjectiveEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveEnvelope {
    pub raw_owner_text: String,
    pub normalized_objective: String,
    pub inferred_intents: Vec<IntentHypothesis>,
    pub non_goals: Vec<String>,
    pub constraints: Vec<ConstraintRecord>,
    pub preferences: Vec<PreferenceRecord>,
    pub risks: Vec<RiskRecord>,
    pub open_questions: Vec<QuestionRecord>,
    pub selected_primary_track: Option<StateTrackId>,
    pub candidate_tracks: Vec<StateTrack>,
}

impl ObjectiveState {
    pub fn new(raw: &str) -> Self {
        let normalized = normalize_objective(raw);
        let non_goals = extract_lines(raw, &["do not", "no ", "out of scope"]);
        let owner_constraints = extract_lines(raw, &["must", "keep", "avoid", "required"]);
        let constraints = owner_constraints
            .iter()
            .map(|summary| ConstraintRecord::hard(summary.clone(), "owner"))
            .collect::<Vec<_>>();
        Self {
            raw_owner_message: raw.to_string(),
            normalized: normalized.clone(),
            version: 1,
            non_goals: non_goals.clone(),
            owner_constraints,
            envelope: ObjectiveEnvelope {
                raw_owner_text: raw.to_string(),
                normalized_objective: normalized,
                inferred_intents: inferred_intents(raw),
                non_goals,
                constraints,
                preferences: preferences(raw),
                risks: initial_risks(),
                open_questions: Vec::new(),
                selected_primary_track: None,
                candidate_tracks: Vec::new(),
            },
        }
    }

    pub fn attach_tracks(&mut self, tracks: &[StateTrack]) {
        self.envelope.candidate_tracks = tracks.to_vec();
        self.envelope.selected_primary_track = tracks.first().map(|track| track.id.clone());
    }
}

fn normalize_objective(raw: &str) -> String {
    let compact = raw
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let lower = compact.to_ascii_lowercase();
    let mut clauses = Vec::new();
    if lower.contains("doc") || lower.contains("readme") || lower.contains("scaffold") {
        clauses.push("semantic recursive documentation");
    }
    if lower.contains("action") || lower.contains("param") || lower.contains("path") {
        clauses.push("robust action parameter recovery");
    }
    if lower.contains("state") || lower.contains("progress") || lower.contains("track") {
        clauses.push("neutral multi-state progress tracking");
    }
    if lower.contains("token") || lower.contains("context") || lower.contains("console") {
        clauses.push("compact context and token observability");
    }
    if lower.contains("test") || lower.contains("verify") || lower.contains("docker") {
        clauses.push("verified completion evidence");
    }
    if lower.contains("lkjagent") || compact.contains("lkjagent") {
        return normalized_lkjagent(&clauses);
    }
    if clauses.is_empty() {
        return format!(
            "Resolve the owner task through planning, evidence capture, execution, and verification: {}",
            compact.chars().take(120).collect::<String>()
        );
    }
    format!(
        "Deliver the owner task by improving {}.",
        clauses.join(", ")
    )
}

fn normalized_lkjagent(clauses: &[&str]) -> String {
    if clauses.is_empty() {
        return "Improve lkjagent practical task completion reliability with evidence-gated execution."
            .to_string();
    }
    format!(
        "Improve lkjagent runtime reliability by delivering {}.",
        clauses.join(", ")
    )
}

fn extract_lines(raw: &str, needles: &[&str]) -> Vec<String> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            needles.iter().any(|needle| lower.contains(needle))
        })
        .take(12)
        .map(str::to_string)
        .collect()
}

fn inferred_intents(raw: &str) -> Vec<IntentHypothesis> {
    let lower = raw.to_ascii_lowercase();
    let mut intents = Vec::new();
    if lower.contains("doc") || lower.contains("readme") {
        intents.push(intent("document-structure", 85));
    }
    if lower.contains("fault") || lower.contains("param") || lower.contains("path") {
        intents.push(intent("action-recovery", 82));
    }
    if lower.contains("state") || lower.contains("progress") {
        intents.push(intent("state-tracking", 76));
    }
    if intents.is_empty() {
        intents.push(intent("implementation", 55));
    }
    intents
}

fn preferences(raw: &str) -> Vec<PreferenceRecord> {
    extract_lines(raw, &["prefer", "use ", "avoid", "do not"])
        .into_iter()
        .map(|summary| PreferenceRecord {
            summary,
            source: "owner".to_string(),
        })
        .collect()
}

fn initial_risks() -> Vec<RiskRecord> {
    vec![RiskRecord {
        summary: "raw owner message may be broader than the executable task".to_string(),
        mitigation: "normalize into tracks before selecting actions".to_string(),
        status: FieldStatus::Open,
    }]
}

fn intent(label: &str, confidence: u8) -> IntentHypothesis {
    IntentHypothesis {
        label: label.to_string(),
        confidence,
    }
}
