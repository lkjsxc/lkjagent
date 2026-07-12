use serde::{Deserialize, Serialize};

use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

#[rustfmt::skip]
pub const RECOVERY_KINDS: &[&str] = &["protocol", "hidden-tool", "premature-final", "missing-read", "stale-or-ambiguous-edit", "output-limit", "endpoint", "check", "equal-progress"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[rustfmt::skip]
pub enum FailureClass { Parse, OutputLimit, Admission, Endpoint, Effect, Check }

impl FailureClass {
    #[rustfmt::skip]
    pub fn from_fault(kind: &str, detail: &str) -> Self {
        let detail = detail.to_ascii_lowercase();
        if detail.contains("output limit") || detail.contains("maximum tokens")
            || detail.contains("length limit") { return Self::OutputLimit; }
        match kind { "parse" => Self::Parse, "admission" => Self::Admission,
            "endpoint" => Self::Endpoint, "effect" => Self::Effect,
            "check" => Self::Check, _ => Self::Effect }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[rustfmt::skip]
pub enum RecoveryStrategy {
    GrammarRepair, ConcreteExample, ConstrainedGrammar, NarrowOutput,
    ReduceUnit, ContinueBoundary, SplitSection, ReplanArtifact,
    RemoveHiddenTool, CorrectPrimitive, SelectTarget, Reinspect,
    RetryBackoff, AlternateSampling, SmallerPrompt, Reconnect, WaitExternal,
    InspectFilesystem, IdempotentReplay, Compensate, Quarantine,
    InspectCheck, RepairSource, RerunCheck, Replan, OwnerBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step {
    strategy: RecoveryStrategy,
    condition: &'static str,
}

#[rustfmt::skip]
const PARSE: &[Step] = &[
    Step { strategy: RecoveryStrategy::GrammarRepair, condition: "repair exact grammar" },
    Step { strategy: RecoveryStrategy::ConcreteExample, condition: "add one valid example" },
    Step { strategy: RecoveryStrategy::ConstrainedGrammar, condition: "constrain grammar" },
    Step { strategy: RecoveryStrategy::NarrowOutput, condition: "narrow output shape" },
];
#[rustfmt::skip]
const OUTPUT: &[Step] = &[
    Step { strategy: RecoveryStrategy::ReduceUnit, condition: "reduce semantic unit size" },
    Step { strategy: RecoveryStrategy::ContinueBoundary, condition: "continue from safe boundary" },
    Step { strategy: RecoveryStrategy::SplitSection, condition: "split semantic section" },
    Step { strategy: RecoveryStrategy::ReplanArtifact, condition: "replan artifact units" },
];
#[rustfmt::skip]
const ADMISSION: &[Step] = &[
    Step { strategy: RecoveryStrategy::RemoveHiddenTool, condition: "remove hidden tool" },
    Step { strategy: RecoveryStrategy::CorrectPrimitive, condition: "correct typed primitive" },
    Step { strategy: RecoveryStrategy::SelectTarget, condition: "select deterministic target" },
    Step { strategy: RecoveryStrategy::Reinspect, condition: "reinspect admitted state" },
];
#[rustfmt::skip]
const ENDPOINT: &[Step] = &[
    Step { strategy: RecoveryStrategy::RetryBackoff, condition: "wait for retry eligibility" },
    Step { strategy: RecoveryStrategy::AlternateSampling, condition: "change sampling limits" },
    Step { strategy: RecoveryStrategy::SmallerPrompt, condition: "reduce prompt budget" },
    Step { strategy: RecoveryStrategy::Reconnect, condition: "reconnect endpoint" },
    Step { strategy: RecoveryStrategy::WaitExternal, condition: "wait for external endpoint" },
];
#[rustfmt::skip]
const EFFECT: &[Step] = &[
    Step { strategy: RecoveryStrategy::InspectFilesystem, condition: "inspect external state" },
    Step { strategy: RecoveryStrategy::IdempotentReplay, condition: "replay exact intent" },
    Step { strategy: RecoveryStrategy::Compensate, condition: "apply verified compensation" },
    Step { strategy: RecoveryStrategy::Quarantine, condition: "quarantine conflict" },
];
#[rustfmt::skip]
const CHECK: &[Step] = &[
    Step { strategy: RecoveryStrategy::InspectCheck, condition: "inspect measured failure" },
    Step { strategy: RecoveryStrategy::RepairSource, condition: "repair checked source" },
    Step { strategy: RecoveryStrategy::RerunCheck, condition: "rerun invalidated check" },
    Step { strategy: RecoveryStrategy::Replan, condition: "replan failed obligation" },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub next_strategy: Option<RecoveryStrategy>,
    pub changed_condition: String,
    pub remaining_budget: u32,
    pub exhausted: bool,
    pub wait_external: bool,
}

#[rustfmt::skip]
pub fn plan(class: FailureClass, prior_failures: usize) -> RecoveryPlan {
    let steps = steps(class); let Some(step) = steps.get(prior_failures) else {
        return RecoveryPlan { next_strategy: None, changed_condition: "recovery ladder exhausted".to_string(),
            remaining_budget: 0, exhausted: true, wait_external: false }; };
    RecoveryPlan { next_strategy: Some(step.strategy), changed_condition: step.condition.to_string(),
        remaining_budget: u32::try_from(steps.len().saturating_sub(prior_failures + 1)).unwrap_or(0),
        exhausted: false, wait_external: step.strategy == RecoveryStrategy::WaitExternal }
}

#[rustfmt::skip]
fn steps(class: FailureClass) -> &'static [Step] {
    match class { FailureClass::Parse => PARSE, FailureClass::OutputLimit => OUTPUT,
        FailureClass::Admission => ADMISSION, FailureClass::Endpoint => ENDPOINT,
        FailureClass::Effect => EFFECT, FailureClass::Check => CHECK }
}

pub fn strategy_condition(strategy: RecoveryStrategy) -> &'static str {
    [PARSE, OUTPUT, ADMISSION, ENDPOINT, EFFECT, CHECK]
        .into_iter()
        .flatten()
        .find(|step| step.strategy == strategy)
        .map_or("owner-visible block", |step| step.condition)
}

pub fn bounded_diagnostic(detail: &str) -> String {
    detail.chars().take(512).collect()
}

#[rustfmt::skip]
pub fn normalized_signature(detail: &str) -> Result<String, FingerprintError> {
    let mut parts = detail.split_whitespace().take(64).peekable(); let mut normalized = Vec::new();
    while let Some(part) = parts.next() {
        let lower = part.to_ascii_lowercase();
        if lower.ends_with(':') { let key = identifier_key(&lower);
            if volatile_key(&key) { normalized.push(format!("{key}=#")); parts.next(); continue; } }
        normalized.push(normalize_part(&lower));
    }
    stable_fingerprint(&normalized.join(" "))
}

fn normalize_part(part: &str) -> String {
    let lower = part.to_ascii_lowercase();
    if ["req-", "request-", "trace-"]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        return "#id".to_string();
    }
    if let Some((key, _)) = lower.split_once('=').or_else(|| lower.split_once(':')) {
        let key = identifier_key(key);
        if volatile_key(&key) {
            return format!("{key}=#");
        }
    }
    let mut normalized = String::new();
    let mut digits = false;
    for character in lower.chars() {
        if character.is_ascii_digit() {
            if !digits {
                normalized.push('#');
            }
            digits = true;
        } else {
            normalized.push(character);
            digits = false;
        }
    }
    let identifier = normalized.len() >= 12
        && normalized.contains('#')
        && !normalized.contains('/')
        && normalized
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '#'));
    if identifier {
        "#id".to_string()
    } else {
        normalized
    }
}

fn identifier_key(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn volatile_key(key: &str) -> bool {
    matches!(
        key,
        "id" | "requestid" | "traceid" | "correlationid" | "timestamp" | "pid"
    )
}

pub fn tuple_fingerprint(
    operation: &str,
    prompt: &str,
    tool_view: &str,
    budget: &str,
    signature: &str,
) -> Result<String, FingerprintError> {
    stable_fingerprint(&(operation, prompt, tool_view, budget, signature))
}
