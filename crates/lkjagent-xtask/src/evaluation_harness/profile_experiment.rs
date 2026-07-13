use lkjagent_core::runtime_decision::{OperationKey, OutputEnvelope, RuntimeDecision};
use lkjagent_core::runtime_tool_call::parse_model_value;
use lkjagent_core::runtime_tool_catalog::direct_tool_view_for_state;
use lkjagent_llm::client::{complete, ClientConfig};
use lkjagent_llm::message::Message;
use lkjagent_llm::wire::CallSpec;
use std::fs;
use std::path::Path;
use std::time::Duration;

pub fn run(root: &Path, endpoint: &Path) -> Result<String, String> {
    let values = super::scenario::endpoint_file(endpoint)?;
    let mut client = ClientConfig::new(
        required(&values, "LKJAGENT_ENDPOINT_URL")?,
        required(&values, "LKJAGENT_MODEL")?,
    );
    client.api_key = Some(required(&values, "LKJAGENT_API_KEY")?.to_string());
    client.timeout = Duration::from_secs(
        values
            .get("LKJAGENT_ENDPOINT_TIMEOUT_SECONDS")
            .and_then(|value| value.parse().ok())
            .unwrap_or(900),
    );
    client.max_tokens = 256;
    let source = git(root, &["rev-parse", "HEAD"])?;
    let plan_commit = git(
        root,
        &[
            "log",
            "--diff-filter=A",
            "--format=%H",
            "--",
            "evaluation/experiment-plan.tsv",
        ],
    )?
    .lines()
    .last()
    .unwrap_or_default()
    .to_string();
    if plan_commit.is_empty() || plan_commit == source || !ancestor(root, &plan_commit, &source) {
        return Err("experiment plan is not a strict ancestor".into());
    }
    let plan = fs::read_to_string(root.join("evaluation/experiment-plan.tsv"))
        .map_err(|error| error.to_string())?;
    let mut lines = plan.lines();
    let headers = lines
        .next()
        .ok_or("experiment plan is empty")?
        .split('\t')
        .collect::<Vec<_>>();
    let mut output = String::from("cell\tscenario\trepeat\texperiment_source_commit\tplan_commit\tprofile_sha256\toutcome\tresponse_sha256\tendpoint_call_count\n");
    let mut call_count = 0_u64;
    for line in lines {
        let values = line.split('\t').collect::<Vec<_>>();
        if values.len() != headers.len() {
            return Err("experiment row is malformed".into());
        }
        let row = headers
            .iter()
            .copied()
            .zip(values.iter().copied())
            .collect::<std::collections::BTreeMap<_, _>>();
        let cell = row["cell"];
        let rejected = hard_rejected(&row);
        let profile = super::sha256(line.as_bytes());
        for scenario in row["scenarios"].split(',') {
            for repeat in 1..=3 {
                let (outcome, response_hash, calls) = if rejected {
                    (
                        "static-rejected".to_string(),
                        super::sha256(b"static-rejected"),
                        0,
                    )
                } else {
                    let prompt = prompt(&row, scenario);
                    let completion = complete(
                        &client,
                        &[
                            Message::system(prompt),
                            Message::user("Return the action now."),
                        ],
                        &CallSpec::action(256),
                        0,
                    )
                    .map_err(|error| format!("configured profile call failed: {error:?}"))?;
                    let response_hash = super::sha256(completion.content.as_bytes());
                    let decision = RuntimeDecision::new(
                        "profile-decision",
                        "profile-matter",
                        OperationKey("orient.matter".into()),
                        direct_tool_view_for_state("orient", None),
                        OutputEnvelope::Action,
                    );
                    let outcome = if parse_model_value(&completion.content, &decision).is_ok() {
                        "admitted"
                    } else {
                        "parse-fault"
                    };
                    call_count += 1;
                    (outcome.to_string(), response_hash, 1)
                };
                output.push_str(&format!("{cell}\t{scenario}\t{repeat}\t{source}\t{plan_commit}\t{profile}\t{outcome}\t{response_hash}\t{calls}\n"));
            }
        }
    }
    let directory = root.join("evaluation/evidence").join(&source);
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    fs::write(directory.join("experiment-outcomes.tsv"), output)
        .map_err(|error| error.to_string())?;
    Ok(format!(
        "ok experiment source={source} configured_calls={call_count}"
    ))
}

fn hard_rejected(row: &std::collections::BTreeMap<&str, &str>) -> bool {
    row["envelope"] == "tool-named"
        || row["tool_view"] == "broad-workspace"
        || row["edit_form"] != "exact-text"
}
fn prompt(row: &std::collections::BTreeMap<&str, &str>, scenario: &str) -> String {
    let example = if row["example"] == "none" {
        String::new()
    } else {
        " Example: <tool_call><tool>read_file</tool><path>notes/exact-base.txt</path><offset>1</offset><count>20</count><complete>false</complete></tool_call>.".into()
    };
    let context = if row["context"] == "recent-plus-required" {
        " A recent unrelated note is noise."
    } else {
        ""
    };
    format!("Profile {} scenario {scenario}. Use only the strict attribute-free tool_call grammar. Select read_file for notes/exact-base.txt at offset 1 count 20 with complete false.{context}{example} Output only one action.", row["cell"])
}
fn required<'a>(
    values: &'a std::collections::BTreeMap<String, String>,
    key: &str,
) -> Result<&'a str, String> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("endpoint file lacks {key}"))
}
fn ancestor(root: &Path, older: &str, newer: &str) -> bool {
    std::process::Command::new("git")
        .args(["merge-base", "--is-ancestor", older, newer])
        .current_dir(root)
        .status()
        .is_ok_and(|status| status.success())
}
fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("git command failed".into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
