use std::fs;
use std::path::{Path, PathBuf};

use lkjagent_core::parse::{parse_expected_for_decision, ParseFault};
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
            other => return Err(format!("unknown experiment argument: {other}")),
        }
    }
    Ok(Options { out })
}

fn run_protocol(options: Options) -> Result<PathBuf, String> {
    if let Some(parent) = options.out.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let cases = cases();
    let mut lines = vec!["# Protocol Experiment Results".to_string(), String::new()];
    for case in &cases {
        let actual = parse_expected_for_decision(&decision(), case.raw);
        let passed = case.accept == actual.is_ok() && case.fault.as_ref() == actual.as_ref().err();
        lines.push(format!(
            "- {} expected={} actual={} result={}",
            case.name,
            if case.accept { "accept" } else { "reject" },
            render_actual(&actual),
            if passed { "pass" } else { "fail" }
        ));
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
        ToolSetView::new(vec![ToolViewEntry::new("finish", "finish exploration")
            .with_params(vec!["summary"], Vec::new())]),
        OutputEnvelope::Action,
    )
}

struct ExperimentCase {
    name: &'static str,
    raw: &'static str,
    accept: bool,
    fault: Option<ParseFault>,
}

fn cases() -> Vec<ExperimentCase> {
    vec![
        ExperimentCase {
            name: "current-tool-call",
            raw: "<tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>",
            accept: true,
            fault: None,
        },
        ExperimentCase {
            name: "old-action-envelope",
            raw: "<action><tool_name>finish</tool_name><summary>done</summary></action>",
            accept: false,
            fault: Some(ParseFault::WrongBlock),
        },
        ExperimentCase {
            name: "old-tool-field",
            raw: "<tool_call><tool>finish</tool><summary>done</summary></tool_call>",
            accept: false,
            fault: Some(ParseFault::BadParams),
        },
    ]
}

fn render_actual(result: &Result<lkjagent_core::parse::ParsedOutput, ParseFault>) -> String {
    match result {
        Ok(_) => "accept".to_string(),
        Err(fault) => format!("reject:{fault:?}"),
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
}
