use lkjagent_core::parse::ParseFault;
use lkjagent_core::runtime_admission::AdmissionStatus;
use lkjagent_core::runtime_tool_call_v2::ToolCallV2Error;

pub struct ExperimentCase {
    pub name: &'static str,
    pub raw: String,
    pub accept: bool,
    pub fault: Option<ParseFault>,
    pub admission: Option<AdmissionStatus>,
}

pub fn cases() -> Vec<ExperimentCase> {
    vec![
        case("valid-v2-finish", v2("finish", "{\"summary\":\"done\"}"), true, None, Some(AdmissionStatus::Admitted)),
        case("safe-v2-fs-read", v2("fs.read", "{\"path\":\"README.md\",\"count\":20}"), true, None, Some(AdmissionStatus::Admitted)),
        case("invalid-count", v2("fs.read", "{\"path\":\"README.md\",\"count\":\"many\"}"), false, Some(ParseFault::ActionV2(ToolCallV2Error::ArgsSchemaViolation("wrong primitive for count".into()))), None),
        case("legacy-v1-tool-call", "<tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>", false, Some(ParseFault::ActionV2(ToolCallV2Error::NoActionFound)), None),
        case("old-action-envelope", "<action><tool_name>finish</tool_name><summary>done</summary></action>", false, Some(ParseFault::ActionV2(ToolCallV2Error::NoActionFound)), None),
        case("missing-required", v2("finish", "{}"), false, Some(ParseFault::ActionV2(ToolCallV2Error::ArgsSchemaViolation("missing arg summary".into()))), None),
        case("unknown-tool", v2("shell.run", "{\"command\":\"pwd\"}"), false, Some(ParseFault::UnknownTool), None),
        case("duplicate-field", "<lkjagent_action_v2>{\"schema_version\":\"lkjagent.tool_call.v2\",\"decision_id\":\"experiment-decision\",\"tool_name\":\"fs.read\",\"args\":{\"path\":\"a\",\"path\":\"b\"},\"context_frame_fingerprint\":\"\"}</lkjagent_action_v2>", false, Some(ParseFault::ActionV2(ToolCallV2Error::DuplicateKey("/args/path".into()))), None),
        case("unknown-field", v2("finish", "{\"summary\":\"done\",\"extra\":\"x\"}"), false, Some(ParseFault::ActionV2(ToolCallV2Error::ArgsSchemaViolation("unknown arg extra".into()))), None),
        case("placeholder-path", v2("fs.read", "{\"path\":\"FIELD_VALUE\"}"), true, None, Some(AdmissionStatus::Rejected)),
        case("prose-outside", "note <lkjagent_action_v2>{\"schema_version\":\"lkjagent.tool_call.v2\",\"decision_id\":\"experiment-decision\",\"tool_name\":\"finish\",\"args\":{\"summary\":\"done\"},\"context_frame_fingerprint\":\"\"}</lkjagent_action_v2>", false, Some(ParseFault::ActionV2(ToolCallV2Error::EnvelopeMalformed)), None),
        case("unclosed", "<lkjagent_action_v2>{\"schema_version\":\"lkjagent.tool_call.v2\"}", false, Some(ParseFault::ActionV2(ToolCallV2Error::EnvelopeMalformed)), None),
        case("empty", "<lkjagent_action_v2></lkjagent_action_v2>", false, Some(ParseFault::ActionV2(ToolCallV2Error::JsonMalformed)), None),
        case("workspace-escape", v2("fs.read", "{\"path\":\"../secret\"}"), true, None, Some(AdmissionStatus::Rejected)),
    ]
}

fn v2(tool: &str, args: &str) -> String {
    format!(
        "<lkjagent_action_v2>{{\"schema_version\":\"lkjagent.tool_call.v2\",\"decision_id\":\"experiment-decision\",\"tool_name\":\"{tool}\",\"args\":{args},\"context_frame_fingerprint\":\"\"}}</lkjagent_action_v2>"
    )
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
