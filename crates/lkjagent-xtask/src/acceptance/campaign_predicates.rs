use std::collections::BTreeMap;

pub fn exact(fields: &BTreeMap<&str, &str>) -> bool {
    fields.get("fact_exact_path") == Some(&"notes/exact-base.txt")
        && fields.get("fact_created_path") == Some(&"notes/created-proof.txt")
        && hash_value(fields, "fact_exact_sha256")
        && hash_value(fields, "fact_created_sha256")
        && number(fields, "fact_workspace_file_count") == 2
        && number(fields, "fact_changed_path_count") == 2
        && fields.get("fact_first_effect_path") == Some(&"notes/exact-base.txt")
        && hash_value(fields, "fact_edit_prior_sha256")
        && hash_value(fields, "fact_edit_intended_sha256")
        && number(fields, "fact_edit_prior_mode") == number(fields, "fact_edit_intended_mode")
        && number(fields, "fact_edit_current_mode") == number(fields, "fact_edit_intended_mode")
        && one(
            fields,
            &[
                "fact_create_prior_absent_count",
                "fact_create_effect_count",
                "fact_edit_effect_count",
            ],
        )
        && number(fields, "fact_effect_count") == 2
        && number(fields, "fact_current_passed_check_count") >= 6
        && number(fields, "fact_table_count") == 18
}
pub fn daily(fields: &BTreeMap<&str, &str>) -> bool {
    fields
        .get("fact_journal_path")
        .is_some_and(|path| path.starts_with("life/journal/") && path.ends_with("/entry.md"))
        && present(fields, "fact_journal_revision_id")
        && hash_value(fields, "fact_journal_sha256")
        && (1..=512).contains(&number(fields, "fact_journal_token_units"))
        && number(fields, "fact_journal_placeholder_count") == 0
        && number(fields, "fact_journal_lineage_count") > 0
        && one(
            fields,
            &[
                "fact_journal_grounded_owner_fact_count",
                "fact_memory_grounded_correction_count",
                "fact_initial_recall_context_count",
                "fact_relevant_recall_context_count",
            ],
        )
        && present(fields, "fact_memory_current_revision_id")
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
pub fn projects(fields: &BTreeMap<&str, &str>) -> bool {
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
pub fn artifact(fields: &BTreeMap<&str, &str>) -> bool {
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
pub fn terminal(fields: &BTreeMap<&str, &str>) -> bool {
    hash_value(fields, "fact_cast_sha256")
        && [
            "fact_input_frame_count",
            "fact_output_frame_count",
            "fact_resize_count",
            "fact_japanese_input_count",
            "fact_search_input_count",
            "fact_alternate_screen_enter_count",
            "fact_alternate_screen_exit_count",
            "fact_activity_responsive_count",
            "fact_restart_resume_count",
        ]
        .iter()
        .all(|key| number(fields, key) > 0)
        && number(fields, "fact_slow_call_interval_ms") >= 1_000
        && number(fields, "fact_message_identity_count") >= 5
        && number(fields, "fact_duplicate_message_identity_count") == 0
        && fields.contains_key("fact_duplicate_message_identity_count")
}
fn hash_value(fields: &BTreeMap<&str, &str>, key: &str) -> bool {
    fields.get(key).is_some_and(|value| {
        value.len() == 71
            && value.starts_with("sha256:")
            && value[7..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    })
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
fn one(fields: &BTreeMap<&str, &str>, keys: &[&str]) -> bool {
    keys.iter().all(|key| number(fields, key) == 1)
}
