use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use lkjagent_core::parse::{parse_expected_for_decision, Action, ParseFault, ParsedOutput};
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};

use crate::experiment_cases::{cases, ExperimentCase};

pub struct Options {
    pub out: PathBuf,
    pub out_dir: PathBuf,
    pub profile: String,
    pub all: bool,
}

pub fn run(options: Options) -> Result<PathBuf, String> {
    if options.all {
        return run_all(options.out_dir);
    }
    write_profile(&options.out, &profile(&options.profile))
}

fn run_all(out_dir: PathBuf) -> Result<PathBuf, String> {
    fs::create_dir_all(&out_dir).map_err(|error| error.to_string())?;
    for spec in profiles() {
        write_profile(&out_dir.join(format!("{}.md", spec.name)), &spec)?;
    }
    let adoption = out_dir.join("adoption.md");
    fs::write(&adoption, adoption_summary()).map_err(|error| error.to_string())?;
    Ok(adoption)
}

fn write_profile(out: &PathBuf, spec: &ProfileSpec) -> Result<PathBuf, String> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let decision = decision();
    let tool_fp = decision
        .tool_view_fingerprint()
        .map_err(|error| error.message)?;
    let mut lines = vec!["# Protocol Experiment Results".to_string(), String::new()];
    lines.push(format!(
        "profile={} features={} decision={} envelope={:?} tool_fp={} stop=</tool_call>",
        spec.name,
        spec.features.join("+"),
        decision.id,
        decision.expected_envelope,
        tool_fp
    ));
    for case in cases() {
        lines.push(run_case(&decision, &case)?);
    }
    lines.push(String::new());
    lines.push("## Decision".to_string());
    lines.push(format!("result={} next={}", spec.result, spec.next));
    lines.push(String::new());
    lines.push("## Rejected Ideas".to_string());
    lines.push("- Old action envelopes stay rejected after tool-call adoption.".to_string());
    fs::write(out, lines.join("\n")).map_err(|error| error.to_string())?;
    Ok(out.clone())
}

fn decision() -> RuntimeDecision {
    RuntimeDecision::new(
        "experiment-decision",
        "experiment-case",
        OperationKey("model.call/1".to_string()),
        ToolSetView::new(vec![
            ToolViewEntry::new("finish", "finish exploration")
                .with_params(vec!["summary"], Vec::new()),
            ToolViewEntry::new("fs.read", "read workspace file")
                .with_params(vec!["path"], vec!["count"]),
        ]),
        OutputEnvelope::Action,
    )
}

struct ProfileSpec {
    name: String,
    features: Vec<&'static str>,
    result: &'static str,
    next: &'static str,
}

#[rustfmt::skip]
fn profile(name: &str) -> ProfileSpec {
    profiles().into_iter().find(|spec| spec.name == name)
        .unwrap_or_else(|| spec_owned(name, vec!["custom-label"], "candidate", "compare"))
}

#[rustfmt::skip]
fn profiles() -> Vec<ProfileSpec> {
    vec![
        spec("baseline", vec!["current"], "candidate", "compare"),
        spec("protocol-safe", vec!["safe-filled-card"], "candidate", "live"),
        spec("context-kernel", vec!["prompt-cards"], "candidate", "live"),
        spec("personal-workspace", vec!["journal", "todo"], "deferred", "live"),
        spec("software-project", vec!["repo-evidence"], "deferred", "live"),
        spec("artifact-manifest", vec!["manifest", "nested-units"], "deferred", "live"),
        spec("protocol-stress", vec!["parse-fault"], "deferred", "live"),
    ]
}

#[rustfmt::skip]
fn adoption_summary() -> String {
    ["# Protocol Adoption Summary", "",
     "- idea=safe-filled-card status=deferred reason=compare live invalid-call rate",
     "- idea=context-lanes status=deferred reason=needs live proof",
     "- idea=personal-workspace status=deferred reason=requires endpoint profile run",
     "- idea=software-project status=deferred reason=requires endpoint profile run",
     "- idea=artifact-manifest status=deferred reason=requires artifact live profile",
     "- idea=protocol-recovery status=deferred reason=requires protocol stress run", ""]
    .join("\n")
}

fn spec(
    name: &'static str,
    features: Vec<&'static str>,
    result: &'static str,
    next: &'static str,
) -> ProfileSpec {
    spec_owned(name, features, result, next)
}

fn spec_owned(
    name: &str,
    features: Vec<&'static str>,
    result: &'static str,
    next: &'static str,
) -> ProfileSpec {
    ProfileSpec {
        name: name.to_string(),
        features,
        result,
        next,
    }
}

fn run_case(decision: &RuntimeDecision, case: &ExperimentCase) -> Result<String, String> {
    let parsed = parse_expected_for_decision(decision, case.raw);
    let admission = match &parsed {
        Ok(parsed) => action_status(decision, parsed)?,
        Err(_) => None,
    };
    let passed = case.accept == parsed.is_ok()
        && case.fault.as_ref() == parsed.as_ref().err()
        && case.admission == admission;
    Ok(format!(
        "- {} parse={} admission={} result={}",
        case.name,
        render_parse(&parsed),
        render_admission(&admission),
        if passed { "pass" } else { "fail" }
    ))
}

fn action_status(
    decision: &RuntimeDecision,
    parsed: &ParsedOutput,
) -> Result<Option<AdmissionStatus>, String> {
    match parsed {
        ParsedOutput::Action(action) => admit_action(decision, &model_action(action))
            .map(|admission| Some(admission.status))
            .map_err(|error| error.message),
        _ => Ok(None),
    }
}

fn model_action(action: &Action) -> ModelAction {
    let params = action
        .params
        .iter()
        .filter(|(name, _)| name != "tool_name")
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect::<BTreeMap<_, _>>();
    ModelAction {
        tool: action.tool.clone(),
        params,
    }
}

fn render_parse(result: &Result<ParsedOutput, ParseFault>) -> String {
    match result {
        Ok(_) => "accept".to_string(),
        Err(fault) => format!("reject:{fault:?}"),
    }
}

fn render_admission(status: &Option<AdmissionStatus>) -> String {
    match status {
        Some(AdmissionStatus::Admitted) => "admitted".to_string(),
        Some(AdmissionStatus::Rejected) => "rejected".to_string(),
        None => "n/a".to_string(),
    }
}
