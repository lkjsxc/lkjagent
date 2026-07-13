use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn derivations(root: &Path, path: &Path, bytes: &[u8], source: &str) -> BTreeSet<String> {
    if super::secret::kind(bytes).is_some() || super::secret::contains_loaded(bytes) {
        return BTreeSet::new();
    }
    let Some((scenario, fields)) = bound(root, path, bytes, source) else {
        return BTreeSet::new();
    };
    match scenario {
        "daily-life-recall" if daily(&fields) => ids(&["C04", "E07", "W05", "W06"]),
        "multi-project-development" if projects(&fields) => ids(&["C05", "E08"]),
        "long-artifact-recovery" if artifact(&fields) => ids(&["E06", "R09", "R11", "W08", "X02"]),
        "slow-japanese-pty" if terminal(&fields) => ids(&[
            "E09", "T03", "T05", "T06", "T07", "T08", "T09", "T10", "W07",
        ]),
        _ => BTreeSet::new(),
    }
}

fn bound<'a>(
    root: &Path,
    path: &Path,
    bytes: &'a [u8],
    source: &str,
) -> Option<(&'a str, BTreeMap<&'a str, &'a str>)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let fields = pairs(text)?;
    let scenario = *fields.get("scenario")?;
    let expected = format!("campaign-{scenario}-run.tsv");
    let scenario_fingerprint =
        crate::evaluation_harness::scenario_fingerprint(root, scenario).ok()?;
    if path.file_name()?.to_str()? != expected
        || fields.get("source_commit") != Some(&source)
        || fields.get("mode") != Some(&"run")
        || fields.get("semantic_status") != Some(&"evaluated")
        || fields.get("outcome") != Some(&"passed")
        || fields.get("scenario_sha256") != Some(&scenario_fingerprint.as_str())
        || !hash(fields.get("binary_sha256")?)
        || !hash(fields.get("command_capture_sha256")?)
        || !hash(fields.get("workspace_before_sha256")?)
        || !hash(fields.get("workspace_after_sha256")?)
        || !hash(fields.get("workspace_diff_sha256")?)
        || number(&fields, "duration_seconds") < 900
        || number(&fields, "measured_owner_turn_count") < 5
        || number(&fields, "measured_runtime_decision_count") < 8
        || number(&fields, "measured_useful_decision_count") < 5
        || number(&fields, "measured_progress_decision_count") < 3
        || number(&fields, "measured_provider_exchange_count") == 0
        || number(&fields, "measured_activity_count") == 0
        || number(&fields, "command_count") == 0
    {
        return None;
    }
    Some((scenario, fields))
}

fn daily(fields: &BTreeMap<&str, &str>) -> bool {
    fields
        .get("fact_journal_path")
        .is_some_and(|path| path.starts_with("life/journal/") && path.ends_with("/entry.md"))
        && present(fields, "fact_journal_revision_id")
        && hash_value(fields, "fact_journal_sha256")
        && (1..=512).contains(&number(fields, "fact_journal_token_units"))
        && number(fields, "fact_journal_placeholder_count") == 0
        && number(fields, "fact_journal_lineage_count") > 0
        && number(fields, "fact_journal_grounded_owner_fact_count") == 1
        && present(fields, "fact_memory_current_revision_id")
        && number(fields, "fact_memory_grounded_correction_count") == 1
        && number(fields, "fact_initial_recall_context_count") == 1
        && number(fields, "fact_relevant_recall_context_count") == 1
        && zero(
            fields,
            &[
                "fact_correction_memory_context_count",
                "fact_noise_recall_context_count",
                "fact_stale_memory_context_count",
                "fact_rogue_memory_context_count",
            ],
        )
}

fn projects(fields: &BTreeMap<&str, &str>) -> bool {
    present(fields, "fact_orbit_revision_id")
        && present(fields, "fact_orbital_revision_id")
        && number(fields, "fact_changed_path_count") > 0
        && number(fields, "fact_current_passed_check_count") > 0
        && number(fields, "fact_restart_resume_count") > 0
        && zero(
            fields,
            &[
                "fact_orbit_in_orbital_context_count",
                "fact_orbital_in_orbit_context_count",
                "fact_duplicate_effect_count",
            ],
        )
}

fn artifact(fields: &BTreeMap<&str, &str>) -> bool {
    fields.get("fact_artifact_readme_path").is_some_and(|path| {
        path.starts_with("artifacts/documents/") && path.ends_with("/README.md")
    }) && number(fields, "fact_artifact_child_count") >= 2
        && number(fields, "fact_readme_link_count") == number(fields, "fact_artifact_child_count")
        && number(fields, "fact_nonplaceholder_unit_count")
            == number(fields, "fact_artifact_child_count") + 1
        && number(fields, "fact_aggregate_word_count") >= 1_500
        && number(fields, "fact_current_revision_count")
            == number(fields, "fact_artifact_child_count") + 1
        && number(fields, "fact_report_current_check_count")
            >= number(fields, "fact_artifact_child_count") + 2
        && number(fields, "fact_output_limit_recovery_count") > 0
        && number(fields, "fact_strategy_change_count") > 1
        && number(fields, "fact_restart_resume_count") > 0
        && number(fields, "fact_source_lineage_count") > number(fields, "fact_artifact_child_count")
        && zero(
            fields,
            &[
                "fact_empty_unit_count",
                "fact_truncated_revision_count",
                "fact_early_completion_count",
            ],
        )
}

fn terminal(fields: &BTreeMap<&str, &str>) -> bool {
    hash_value(fields, "fact_cast_sha256")
        && number(fields, "fact_input_frame_count") > 0
        && number(fields, "fact_output_frame_count") > 0
        && number(fields, "fact_resize_count") > 0
        && number(fields, "fact_japanese_input_count") > 0
        && number(fields, "fact_search_input_count") > 0
        && number(fields, "fact_slow_call_interval_ms") >= 1_000
        && number(fields, "fact_alternate_screen_enter_count") > 0
        && number(fields, "fact_alternate_screen_exit_count") > 0
        && number(fields, "fact_activity_responsive_count") > 0
        && number(fields, "fact_message_identity_count") >= 5
        && number(fields, "fact_duplicate_message_identity_count") == 0
        && fields.contains_key("fact_duplicate_message_identity_count")
        && number(fields, "fact_restart_resume_count") > 0
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
fn hash_value(fields: &BTreeMap<&str, &str>, key: &str) -> bool {
    fields.get(key).is_some_and(|value| hash(value))
}
fn number(fields: &BTreeMap<&str, &str>, key: &str) -> u64 {
    fields
        .get(key)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}
fn present(fields: &BTreeMap<&str, &str>, key: &str) -> bool {
    fields.get(key).is_some_and(|value| !value.is_empty())
}
fn zero(fields: &BTreeMap<&str, &str>, keys: &[&str]) -> bool {
    keys.iter()
        .all(|key| number(fields, key) == 0 && fields.contains_key(key))
}
fn ids(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
