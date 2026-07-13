use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::campaign_predicates::{artifact, daily, exact, projects, terminal};

pub fn derivations(root: &Path, path: &Path, bytes: &[u8], source: &str) -> BTreeSet<String> {
    if super::secret::kind(bytes).is_some() || super::secret::contains_loaded(bytes) {
        return BTreeSet::new();
    }
    let Some((scenario, development, fields)) = bound(root, path, bytes, source) else {
        return BTreeSet::new();
    };
    let mut out = match scenario {
        "exact-file-edit" if exact(&fields) => ids(&["F01", "F02", "F07", "W02"]),
        "daily-life-recall" if daily(&fields) => ids(&["C04", "W05", "W06"]),
        "multi-project-development" if projects(&fields) => ids(&["C05"]),
        "long-artifact-recovery" if artifact(&fields) => ids(&["R09", "R11", "W08", "X02"]),
        "slow-japanese-pty" if terminal(&fields) => {
            ids(&["T03", "T05", "T06", "T07", "T08", "T09", "T10", "W07"])
        }
        _ => BTreeSet::new(),
    };
    if !out.is_empty() {
        if let Some(id) = campaign_id(scenario, development) {
            out.insert(id.into());
        }
    }
    out
}

fn bound<'a>(
    root: &Path,
    path: &Path,
    bytes: &'a [u8],
    source: &str,
) -> Option<(&'a str, bool, BTreeMap<&'a str, &'a str>)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let fields = pairs(text)?;
    let scenario = *fields.get("scenario")?;
    let final_name = format!("campaign-{scenario}-run.tsv");
    let development_name = format!("campaign-{scenario}-development.tsv");
    let name = path.file_name()?.to_str()?;
    let development = name == development_name;
    let source_ok = if development {
        let commit = *fields.get("development_source_commit")?;
        commit != source && ancestor(root, commit, source)
    } else {
        name == final_name && fields.get("source_commit") == Some(&source)
    };
    let scenario_fingerprint =
        crate::evaluation_harness::scenario_fingerprint(root, scenario).ok()?;
    let campaign_source =
        development.then(|| *fields.get("development_source_commit").unwrap_or(&""));
    let expected_binary =
        super::build_manifest::binary(root, path.parent()?, source, development, campaign_source)?;
    let progress_floor = if scenario == "exact-file-edit" { 2 } else { 3 };
    if !source_ok
        || fields.get("mode") != Some(&"run")
        || fields.get("semantic_status") != Some(&"evaluated")
        || fields.get("outcome") != Some(&"passed")
        || fields.get("scenario_sha256") != Some(&scenario_fingerprint.as_str())
        || fields.get("binary_sha256") != Some(&expected_binary.as_str())
        || !hash(fields.get("command_capture_sha256")?)
        || !hash(fields.get("workspace_before_sha256")?)
        || !hash(fields.get("workspace_after_sha256")?)
        || !hash(fields.get("workspace_diff_sha256")?)
        || number(&fields, "duration_seconds") < 900
        || number(&fields, "measured_owner_turn_count") < 5
        || number(&fields, "measured_runtime_decision_count") < 8
        || number(&fields, "measured_useful_decision_count") < 5
        || number(&fields, "measured_progress_decision_count") < progress_floor
        || number(&fields, "measured_provider_exchange_count") == 0
        || number(&fields, "measured_activity_count") == 0
        || number(&fields, "command_count") == 0
    {
        return None;
    }
    Some((scenario, development, fields))
}

fn campaign_id(scenario: &str, development: bool) -> Option<&'static str> {
    Some(match (scenario, development) {
        ("exact-file-edit", true) => "E05",
        ("long-artifact-recovery", true) => "E06",
        ("daily-life-recall", true) => "E07",
        ("multi-project-development", true) => "E08",
        ("slow-japanese-pty", true) => "E09",
        ("exact-file-edit", false) => "E10",
        ("long-artifact-recovery", false) => "E11",
        ("daily-life-recall", false) => "E12",
        ("multi-project-development", false) => "E13",
        ("slow-japanese-pty", false) => "E14",
        _ => return None,
    })
}
fn ancestor(root: &Path, older: &str, newer: &str) -> bool {
    if older.len() != 40
        || !older
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return false;
    }
    std::process::Command::new("git")
        .args(["merge-base", "--is-ancestor", older, newer])
        .current_dir(root)
        .output()
        .is_ok_and(|output| output.status.success())
}
fn pairs(text: &str) -> Option<BTreeMap<&str, &str>> {
    let mut lines = text.lines();
    (lines.next()? == "field\tvalue").then_some(())?;
    let mut out = BTreeMap::new();
    for line in lines {
        let (key, value) = line.split_once('\t')?;
        if key.is_empty()
            || value.is_empty()
            || value.contains('\t')
            || out.insert(key, value).is_some()
        {
            return None;
        }
    }
    Some(out)
}
fn hash(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
fn number(fields: &BTreeMap<&str, &str>, key: &str) -> u64 {
    fields
        .get(key)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}
fn ids(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
