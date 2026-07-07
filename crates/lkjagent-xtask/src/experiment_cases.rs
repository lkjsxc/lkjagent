use lkjagent_core::parse::ParseFault;
use lkjagent_core::runtime_admission::AdmissionStatus;
use lkjagent_core::runtime_tool_call::ToolCallError;

pub struct ExperimentCase {
    pub name: &'static str,
    pub raw: String,
    pub accept: bool,
    pub fault: Option<ParseFault>,
    pub admission: Option<AdmissionStatus>,
}

pub fn cases() -> Vec<ExperimentCase> {
    vec![
        case(
            "valid-finish",
            action("finish", &[("summary", "done")]),
            true,
            None,
            Some(AdmissionStatus::Admitted),
        ),
        case(
            "safe-fs-read",
            action("fs.read", &[("path", "README.md"), ("count", "20")]),
            true,
            None,
            Some(AdmissionStatus::Admitted),
        ),
        case(
            "invalid-count",
            action("fs.read", &[("path", "README.md"), ("count", "many")]),
            false,
            Some(schema("wrong primitive for count")),
            None,
        ),
        case(
            "old-tool-call",
            "<tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>",
            false,
            Some(ParseFault::Action(ToolCallError::NoActionFound)),
            None,
        ),
        case(
            "old-action-envelope",
            "<action><tool_name>finish</tool_name><summary>done</summary></action>",
            false,
            Some(ParseFault::Action(ToolCallError::NoActionFound)),
            None,
        ),
        case(
            "missing-required",
            action("finish", &[]),
            false,
            Some(schema("missing arg summary")),
            None,
        ),
        case(
            "unknown-tool",
            action("shell.run", &[("command", "pwd")]),
            false,
            Some(ParseFault::UnknownTool),
            None,
        ),
        case(
            "duplicate-field",
            duplicate_path(),
            false,
            Some(ParseFault::Action(ToolCallError::DuplicateTag(
                "argument/path".into(),
            ))),
            None,
        ),
        case(
            "unknown-field",
            action("finish", &[("summary", "done"), ("extra", "x")]),
            false,
            Some(schema("unknown arg extra")),
            None,
        ),
        case(
            "placeholder-path",
            action("fs.read", &[("path", "FIELD_VALUE")]),
            true,
            None,
            Some(AdmissionStatus::Rejected),
        ),
        case(
            "prose-outside",
            format!("note {}", action("finish", &[("summary", "done")])),
            false,
            Some(ParseFault::Action(ToolCallError::EnvelopeMalformed)),
            None,
        ),
        case(
            "unclosed",
            "<lkjagent_action><decision_id>experiment-decision</decision_id>",
            false,
            Some(ParseFault::Action(ToolCallError::EnvelopeMalformed)),
            None,
        ),
        case(
            "empty",
            "<lkjagent_action></lkjagent_action>",
            false,
            Some(schema("missing decision_id")),
            None,
        ),
        case(
            "workspace-escape",
            action("fs.read", &[("path", "../secret")]),
            true,
            None,
            Some(AdmissionStatus::Rejected),
        ),
    ]
}

fn action(tool: &str, args: &[(&str, &str)]) -> String {
    let mut out = format!(
        "<lkjagent_action><decision_id>experiment-decision</decision_id><context_fingerprint></context_fingerprint><tool_name>{tool}</tool_name>"
    );
    for (name, value) in args {
        out.push_str(&format!(
            "<argument><name>{name}</name><value>{value}</value></argument>"
        ));
    }
    out.push_str("</lkjagent_action>");
    out
}

fn duplicate_path() -> String {
    action("fs.read", &[("path", "a"), ("path", "b")])
}

fn schema(message: &str) -> ParseFault {
    ParseFault::Action(ToolCallError::ArgsSchemaViolation(message.into()))
}

fn case(
    name: &'static str,
    raw: impl Into<String>,
    accept: bool,
    fault: Option<ParseFault>,
    admission: Option<AdmissionStatus>,
) -> ExperimentCase {
    ExperimentCase {
        name,
        raw: raw.into(),
        accept,
        fault,
        admission,
    }
}
