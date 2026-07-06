use lkjagent_core::parse::ParseFault;
use lkjagent_core::runtime_admission::AdmissionStatus;

pub struct ExperimentCase {
    pub name: &'static str,
    pub raw: &'static str,
    pub accept: bool,
    pub fault: Option<ParseFault>,
    pub admission: Option<AdmissionStatus>,
}

pub fn cases() -> Vec<ExperimentCase> {
    vec![
        case("valid-tool-call", "<tool_call><tool_name>finish</tool_name><summary>done</summary></tool_call>", true, None, Some(AdmissionStatus::Admitted)),
        case("safe-fs-read-example", "<tool_call><tool_name>fs.read</tool_name><path>README.md</path><count>20</count></tool_call>", true, None, Some(AdmissionStatus::Admitted)),
        case("invalid-count", "<tool_call><tool_name>fs.read</tool_name><path>README.md</path><count>many</count></tool_call>", true, None, Some(AdmissionStatus::Rejected)),
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
