use lkjagent_xtask::{acceptance::derive_campaign_attachment, evaluation_harness};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn daily_campaign_derives_only_from_source_bound_measured_rows() -> Result<(), String> {
    let root = repository_root();
    let source = git(&root, &["rev-parse", "HEAD"])?;
    let directory =
        std::env::temp_dir().join(format!("lkjagent-campaign-evidence-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let path = directory.join("campaign-daily-life-recall-run.tsv");
    let valid = daily(&root, &source)?;
    manifest(&root, &directory, &source, 'a')?;
    assert_eq!(
        derive_campaign_attachment(&root, &path, valid.as_bytes(), &source),
        ids(&["C04", "E12", "W05", "W06"])
    );
    for invalid in [
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
        valid.replace(
            &format!("sha256:{}", "a".repeat(64)),
            &format!("sha256:{}", "b".repeat(64)),
        ),
        format!("{valid}mode\trun\n"),
    ] {
        assert!(derive_campaign_attachment(&root, &path, invalid.as_bytes(), &source).is_empty());
    }
    manifest(&root, &directory, &source, 'b')?;
    assert!(derive_campaign_attachment(&root, &path, valid.as_bytes(), &source).is_empty());
    manifest(&root, &directory, &source, 'a')?;
    let manifest_path = directory.join("build-manifest.tsv");
    let original = std::fs::read_to_string(&manifest_path).map_err(|error| error.to_string())?;
    for key in ["checker_sha256", "profile_plan_sha256"] {
        let changed = original
            .lines()
            .map(|line| {
                if line.starts_with(key) {
                    format!("{key}\tsha256:{}", "b".repeat(64))
                } else {
                    line.into()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
            + "\n";
        std::fs::write(&manifest_path, changed).map_err(|error| error.to_string())?;
        assert!(derive_campaign_attachment(&root, &path, valid.as_bytes(), &source).is_empty());
    }
    std::fs::remove_dir_all(directory).map_err(|error| error.to_string())
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
    assert_eq!(
        (
            facts.input_frames,
            facts.resize_frames,
            facts.japanese_inputs
        ),
        (1, 1, 1)
    );
    assert_eq!(facts.slow_interval_ms, 1_300);
    assert_eq!((facts.alt_screen_enter, facts.alt_screen_exit), (1, 1));
    std::fs::remove_file(path).map_err(|error| error.to_string())
}

fn manifest(root: &Path, directory: &Path, source: &str, binary: char) -> Result<(), String> {
    let development = git(root, &["rev-parse", &format!("{source}^")])?;
    let rows = [
        ("manifest_kind", "source-build-binding".into()),
        ("source_commit", source.into()),
        ("development_source_commit", development),
        (
            "binary_sha256",
            format!("sha256:{}", binary.to_string().repeat(64)),
        ),
        (
            "development_binary_sha256",
            format!("sha256:{}", "d".repeat(64)),
        ),
        (
            "checker_sha256",
            source_hash(root, source, "crates/lkjagent-xtask/src/acceptance.rs")?,
        ),
        (
            "schema_sha256",
            source_hash(root, source, "crates/lkjagent-store/src/native-schema.sql")?,
        ),
        (
            "profile_plan_sha256",
            source_hash(root, source, "evaluation/experiment-plan.tsv")?,
        ),
    ];
    let text = "field\tvalue\n".to_string()
        + &rows
            .iter()
            .map(|(key, value)| format!("{key}\t{value}\n"))
            .collect::<String>();
    std::fs::write(directory.join("build-manifest.tsv"), text).map_err(|error| error.to_string())
}
fn daily(root: &Path, source: &str) -> Result<String, String> {
    let scenario = evaluation_harness::scenario_fingerprint(root, "daily-life-recall")?;
    let hash = format!("sha256:{}", "a".repeat(64));
    Ok(format!("field\tvalue\nsource_commit\t{source}\nscenario\tdaily-life-recall\nscenario_sha256\t{scenario}\nbinary_sha256\t{hash}\nmode\trun\nsemantic_status\tevaluated\noutcome\tpassed\nsemantic_detail\tmeasured-native-facts\nduration_seconds\t900\nprovider_exchange_count\t9\nactivity_count\t30\ncommand_count\t1\ncommand_capture_sha256\t{hash}\nworkspace_before_sha256\t{hash}\nworkspace_after_sha256\t{hash}\nworkspace_diff_sha256\t{hash}\nmeasured_owner_turn_count\t5\nmeasured_runtime_decision_count\t8\nmeasured_progress_decision_count\t3\nmeasured_useful_decision_count\t5\nmeasured_provider_exchange_count\t9\nmeasured_activity_count\t30\nfact_journal_path\tlife/journal/2026/07/13/entry.md\nfact_journal_revision_id\trevision-journal\nfact_journal_sha256\t{hash}\nfact_journal_token_units\t400\nfact_journal_placeholder_count\t0\nfact_journal_lineage_count\t2\nfact_journal_grounded_owner_fact_count\t1\nfact_memory_current_revision_id\trevision-memory\nfact_memory_grounded_correction_count\t1\nfact_initial_recall_context_count\t1\nfact_relevant_recall_context_count\t1\nfact_correction_memory_context_count\t0\nfact_noise_recall_context_count\t0\nfact_stale_memory_context_count\t0\nfact_rogue_memory_context_count\t0\n"))
}
fn source_hash(root: &Path, source: &str, path: &str) -> Result<String, String> {
    let bytes = Command::new("git")
        .args(["show", &format!("{source}:{path}")])
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?
        .stdout;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}
fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("git failed".into());
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().into())
}
fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn ids(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).into()).collect()
}
