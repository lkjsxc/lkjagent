use lkjagent_xtask::{acceptance::derive_campaign_attachment, evaluation_harness};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const SOURCE: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

#[test]
fn daily_campaign_derives_only_from_strict_measured_rows() -> Result<(), String> {
    let root = repository_root();
    let path = Path::new("campaign-daily-life-recall-run.tsv");
    let valid = daily(&root)?;
    assert_eq!(
        derive_campaign_attachment(&root, path, valid.as_bytes(), SOURCE),
        ids(&["C04", "E07", "W05", "W06"])
    );
    for (index, invalid) in [
        valid.replace("mode\trun", "mode\tprobe"),
        valid.replace("duration_seconds\t900", "duration_seconds\t899"),
        valid.replace(
            "measured_useful_decision_count\t5",
            "measured_useful_decision_count\t0",
        ),
        valid.replace(
            "fact_stale_memory_context_count\t0",
            "fact_stale_memory_context_count\t1",
        ),
        format!("{valid}mode\trun\n"),
        format!(
            "{valid}note\t{}: {} {}\n",
            "authorization",
            "Bearer",
            "secret".repeat(6)
        ),
    ]
    .into_iter()
    .enumerate()
    {
        assert!(
            derive_campaign_attachment(&root, path, invalid.as_bytes(), SOURCE).is_empty(),
            "invalid fixture {index} derived"
        );
    }
    assert!(
        derive_campaign_attachment(&root, Path::new("result.tsv"), valid.as_bytes(), SOURCE)
            .is_empty()
    );
    Ok(())
}

#[test]
fn bounded_cast_derives_counts_without_retaining_frames() -> Result<(), String> {
    let path = std::env::temp_dir().join(format!("lkjagent-cast-{}.cast", std::process::id()));
    let cast = concat!(
        "{\"version\":2}\n",
        "[0.0,\"o\",\"\\u001b[?1049h activity\"]\n",
        "[0.1,\"i\",\"日本語/検索\"]\n",
        "[0.2,\"r\",\"120x40\"]\n",
        "[0.3,\"m\",\"slow-start\"]\n",
        "[1.5,\"o\",\"activity updated\"]\n",
        "[1.6,\"m\",\"slow-end\"]\n",
        "[1.7,\"o\",\"\\u001b[?1049l\"]\n"
    );
    std::fs::write(&path, cast).map_err(|error| error.to_string())?;
    let facts = evaluation_harness::validate_cast(&path)?;
    assert_eq!(facts.input_frames, 1);
    assert_eq!(facts.resize_frames, 1);
    assert_eq!(facts.japanese_inputs, 1);
    assert_eq!(facts.slow_interval_ms, 1_300);
    assert_eq!((facts.alt_screen_enter, facts.alt_screen_exit), (1, 1));
    std::fs::remove_file(path).map_err(|error| error.to_string())
}

fn daily(root: &Path) -> Result<String, String> {
    let scenario = evaluation_harness::scenario_fingerprint(root, "daily-life-recall")?;
    let hash = format!("sha256:{}", "a".repeat(64));
    Ok(format!(
        "field\tvalue\nsource_commit\t{SOURCE}\nscenario\tdaily-life-recall\nscenario_sha256\t{scenario}\nbinary_sha256\t{hash}\nmode\trun\nsemantic_status\tevaluated\noutcome\tpassed\nsemantic_detail\tmeasured-native-facts\nduration_seconds\t900\nprovider_exchange_count\t9\nactivity_count\t30\ncommand_count\t1\ncommand_capture_sha256\t{hash}\nworkspace_before_sha256\t{hash}\nworkspace_after_sha256\t{hash}\nworkspace_diff_sha256\t{hash}\nmeasured_owner_turn_count\t5\nmeasured_runtime_decision_count\t8\nmeasured_progress_decision_count\t3\nmeasured_useful_decision_count\t5\nmeasured_provider_exchange_count\t9\nmeasured_activity_count\t30\nfact_journal_path\tlife/journal/2026/07/13/entry.md\nfact_journal_revision_id\trevision-journal\nfact_journal_sha256\t{hash}\nfact_journal_token_units\t400\nfact_journal_placeholder_count\t0\nfact_journal_lineage_count\t2\nfact_memory_current_revision_id\trevision-memory\nfact_relevant_recall_context_count\t1\nfact_noise_recall_context_count\t0\nfact_stale_memory_context_count\t0\nfact_rogue_memory_context_count\t0\n"
    ))
}
fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn ids(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).into()).collect()
}
