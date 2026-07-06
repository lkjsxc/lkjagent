use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::parse::{parse_expected_for_decision, Action, ParseFault, ParsedOutput};
use lkjagent_core::runtime_admission::{admit_action, AdmissionStatus, ModelAction};
use lkjagent_core::runtime_decision::{
    OperationKey, OutputEnvelope, RuntimeDecision, ToolSetView, ToolViewEntry,
};
pub fn run(args: &[String], root: &Path) -> i32 {
    match parse(args, root).and_then(run_protocol) {
        Ok(path) => {
            println!("ok experiment protocol out={}", path.display());
            0
        }
        Err(error) => {
            eprintln!("experiment protocol failed");
            eprintln!("exit status: 1");
            eprintln!("{error}");
            1
        }
    }
}
fn parse(args: &[String], root: &Path) -> Result<Options, String> {
    let mut out = root.join("tmp/protocol-experiment-current.md");
    let mut profile = "baseline".to_string();
    let mut index = if args.first().is_some_and(|arg| arg == "protocol") {
        1
    } else {
        0
    };
    while index < args.len() {
        match args[index].as_str() {
            "--out" => {
                out = path_arg(args, index + 1, root)?;
                index += 2;
            }
            "--profile" => {
                profile = args.get(index + 1).ok_or("--profile needs a name")?.clone();
                index += 2;
            }
            other => return Err(format!("unknown experiment argument: {other}")),
        }
    }
    Ok(Options { out, profile })
}
fn run_protocol(options: Options) -> Result<PathBuf, String> {
    if let Some(parent) = options.out.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let decision = decision();
    let tool_fp = decision
        .tool_view_fingerprint()
        .map_err(|error| error.message)?;
    let mut lines = vec!["# Protocol Experiment Results".to_string(), String::new()];
    lines.push(format!(
        "profile={} decision={} envelope={:?} tool_fp={} stop=</tool_call>",
        options.profile, decision.id, decision.expected_envelope, tool_fp
    ));
    for case in cases() {
        lines.push(run_case(&decision, &case)?);
    }
    lines.push(String::new());
    lines.push("## Rejected Ideas".to_string());
    lines.push("- Old action envelopes stay rejected after tool-call adoption.".to_string());
    fs::write(&options.out, lines.join("\n")).map_err(|error| error.to_string())?;
    Ok(options.out)
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
                .with_params(vec!["path"], Vec::new()),
        ]),
        OutputEnvelope::Action,
    )
}

struct ExperimentCase {
    name: &'static str,
    raw: &'static str,
    accept: bool,
    fault: Option<ParseFault>,
    admission: Option<AdmissionStatus>,
}

fn cases() -> Vec<ExperimentCase> {
    vec![
        case("valid-tool-call", "<tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>", true, None, Some(AdmissionStatus::Admitted)),
        case("old-action-envelope", "<action><tool_name>finish</tool_name><summary>done</summary></action>", false, Some(ParseFault::WrongBlock), None),
        case("missing-tool-name", "<tool_call><summary>done</summary></tool_call>", false, Some(ParseFault::BadParams), None),
        case("unknown-tool", "<tool_call><tool_name>shell.run</tool_name><command>pwd</command></tool_call>", false, Some(ParseFault::UnknownTool), None),
        case("duplicate-field", "<tool_call><tool_name>finish</tool_name><summary>a</summary><summary>b</summary></tool_call>", false, Some(ParseFault::BadParams), None),
        case("tool-name-second", "<tool_call><summary>done</summary><tool_name>finish</tool_name></tool_call>", false, Some(ParseFault::BadParams), None),
        case("missing-required", "<tool_call><tool_name>finish</tool_name></tool_call>", false, Some(ParseFault::BadParams), None),
        case("unknown-field", "<tool_call><tool_name>finish</tool_name><summary>done</summary><extra>x</extra></tool_call>", false, Some(ParseFault::BadParams), None),
        case("placeholder-path", "<tool_call><tool_name>fs.read</tool_name><path>FIELD_VALUE</path></tool_call>", true, None, Some(AdmissionStatus::Rejected)),
        case("prose-outside", "note <tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>", false, Some(ParseFault::WrongBlock), None),
        case("unclosed", "<tool_call><tool_name>finish</tool_name>", false, Some(ParseFault::Unclosed), None),
        case("empty", "<tool_call></tool_call>", false, Some(ParseFault::Empty), None),
        case("workspace-escape", "<tool_call><tool_name>fs.read</tool_name><path>../secret</path></tool_call>", true, None, Some(AdmissionStatus::Rejected)),
    ]
}

fn case(
    name: &'static str,
    raw: &'static str,
    accept: bool,
    fault: Option<ParseFault>,
    admission: Option<AdmissionStatus>,
) -> ExperimentCase {
    ExperimentCase {
        name,
        raw,
        accept,
        fault,
        admission,
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

fn path_arg(args: &[String], index: usize, root: &Path) -> Result<PathBuf, String> {
    let value = args
        .get(index)
        .ok_or_else(|| "--out needs a path".to_string())?;
    let path = PathBuf::from(value);
    Ok(if path.is_absolute() {
        path
    } else {
        root.join(path)
    })
}

struct Options {
    out: PathBuf,
    profile: String,
}
