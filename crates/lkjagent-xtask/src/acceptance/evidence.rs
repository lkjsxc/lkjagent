use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use super::{markers, secret};

#[rustfmt::skip]
pub fn derivations(path:&Path,bytes:&[u8],source:&str)->BTreeSet<String>{
 let mut out=BTreeSet::new();let Ok(text)=std::str::from_utf8(bytes)else{return out};let fields=pairs(text);
 if path.file_name().and_then(|x|x.to_str())!=Some("campaign-exact-file-edit-run.tsv")||fields.get("source_commit").map(String::as_str)!=Some(source)||fields.get("scenario").map(String::as_str)!=Some("exact-file-edit")||fields.get("mode").map(String::as_str)!=Some("run")||fields.get("semantic_status").map(String::as_str)!=Some("evaluated")||fields.get("outcome").map(String::as_str)!=Some("passed"){return out}
 let detail=fields.get("semantic_detail").map(|v|v.split(';').filter_map(|x|x.split_once('=')).collect::<HashMap<_,_>>()).unwrap_or_default();
 let n=|key:&str|detail.get(key).and_then(|v|v.parse::<u64>().ok()).unwrap_or(0);
 if detail.get("file_exact")==Some(&"true")&&detail.get("one_file")==Some(&"true")&&n("effects")==1&&n("admissions")>0&&n("providers")>0&&n("tables")==18 { out.extend(["F01","F07","W02"].map(str::to_string)); }
 if n("closed")>=3&&n("owner")>=5&&n("agent")>=3&&n("passed_checks")>=6 { out.insert("F08".into()); }
 out
}

pub fn inspect(path: &Path, bytes: &[u8], source: &str) -> Vec<String> {
    let label = path.to_string_lossy();
    if secret::kind(bytes).is_some() || secret::contains_loaded(bytes) {
        return vec![format!(
            "{label}: secret or authorization pattern detected; bytes suppressed"
        )];
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Vec::new();
    };
    let fields = pairs(text);
    let mut errors = markers::source_errors(path, text, source);
    if !checker_result(path) && (pass_label(&fields) || tabular_pass(text)) {
        errors.push(format!("{label}: editable pass input is forbidden"));
    }
    if fake_terminal(&fields) {
        errors.push(format!(
            "{label}: claimed terminal contradicts raw terminal"
        ));
    }
    if placeholder(&fields) {
        errors.push(format!("{label}: placeholder-only output is not evidence"));
    }
    if scripted(&fields) {
        errors.push(format!(
            "{label}: mock or scripted semantic evidence is forbidden"
        ));
    }
    errors.extend(campaign_errors(&fields, &label));
    errors
}

fn checker_result(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "result.tsv")
}

fn pairs(text: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    for line in text.lines() {
        let mut parts = line.splitn(2, '\t');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            fields
                .entry(key.trim().to_ascii_lowercase())
                .or_insert_with(|| value.trim().to_string());
        }
    }
    fields
}

fn tabular_pass(text: &str) -> bool {
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return false;
    };
    let headers = header
        .split('\t')
        .map(|value| value.trim().to_ascii_lowercase())
        .collect::<Vec<_>>();
    headers.iter().enumerate().any(|(index, key)| {
        matches!(
            key.as_str(),
            "status" | "result" | "passed" | "success" | "derived_status"
        ) && lines
            .clone()
            .any(|line| line.split('\t').nth(index).is_some_and(pass_value))
    })
}

fn pass_value(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "pass" | "passed" | "ok" | "success" | "true"
    )
}

fn pass_label(fields: &HashMap<String, String>) -> bool {
    ["status", "result", "passed", "success", "derived_status"]
        .iter()
        .any(|key| fields.get(*key).is_some_and(|value| pass_value(value)))
}

fn fake_terminal(fields: &HashMap<String, String>) -> bool {
    fields
        .get("claimed_terminal")
        .is_some_and(|value| matches!(value.as_str(), "complete" | "completed" | "success"))
        && fields
            .get("actual_terminal")
            .is_none_or(|value| !matches!(value.as_str(), "complete" | "completed" | "success"))
}

fn placeholder(fields: &HashMap<String, String>) -> bool {
    fields
        .get("generated_placeholder")
        .is_some_and(|value| value == "true")
        || fields.iter().any(|(key, value)| {
            (key.contains("output") || key == "artifact")
                && matches!(
                    value.trim().to_ascii_lowercase().as_str(),
                    "todo" | "tbd" | "placeholder" | "lorem ipsum" | "generated content" | "..."
                )
        })
}

fn scripted(fields: &HashMap<String, String>) -> bool {
    [
        "endpoint_mode",
        "provider_mode",
        "model_mode",
        "semantic_source",
    ]
    .iter()
    .any(|key| {
        fields.get(*key).is_some_and(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "mock" | "scripted" | "replay" | "fixture" | "canned"
            )
        })
    })
}

fn campaign_errors(fields: &HashMap<String, String>, label: &str) -> Vec<String> {
    if !fields.contains_key("duration_seconds") {
        return Vec::new();
    }
    let mut errors = Vec::new();
    if fields
        .get("duration_seconds")
        .and_then(|value| value.parse::<u64>().ok())
        .is_none_or(|value| value < 900)
    {
        errors.push(format!(
            "{label}: campaign duration is shorter than 900 seconds"
        ));
    }
    for key in [
        "decision_count",
        "useful_decision_count",
        "progress_decision_count",
        "owner_turn_count",
    ] {
        if fields
            .get(key)
            .and_then(|value| value.parse::<u64>().ok())
            .is_none_or(|value| value == 0)
        {
            errors.push(format!("{label}: campaign is quiet or missing {key}"));
        }
    }
    errors
}
