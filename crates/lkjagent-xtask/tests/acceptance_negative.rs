use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use lkjagent_xtask::acceptance::{inspect_attachment, scan_history, validate_plans, verify};

static NEXT: AtomicU64 = AtomicU64::new(0);
const SOURCE: &str = "2222222222222222222222222222222222222222";

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(fs::read(
        root().join("evaluation/false-positive-fixtures").join(name),
    )?)
}

fn temp(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "lkjagent-acceptance-{name}-{}-{id}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn has(errors: &[String], needle: &str) -> bool {
    errors.iter().any(|error| error.contains(needle))
}

fn git(root: &Path, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(args).current_dir(root).output()?;
    if !output.status.success() {
        return Err(format!("git command failed: {args:?}").into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn copy_plans(target: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(target.join("evaluation"))?;
    for name in ["workgraph.tsv", "acceptance.tsv", "experiment-plan.tsv"] {
        fs::copy(
            root().join("evaluation").join(name),
            target.join("evaluation").join(name),
        )?;
    }
    Ok(())
}

#[test]
fn acceptance_negative_rejects_claims_staleness_and_semantic_fakes() -> Result<(), Box<dyn Error>> {
    let cases = [
        ("acceptance-fake-pass.tsv", "editable pass"),
        ("acceptance-stale-source.tsv", "stale source"),
        ("acceptance-scripted-semantic.tsv", "scripted semantic"),
        ("acceptance-placeholder-output.tsv", "placeholder-only"),
    ];
    for (name, wanted) in cases {
        let errors = inspect_attachment(Path::new(name), &fixture(name)?, SOURCE);
        assert!(
            errors.iter().any(|error| error.contains(wanted)),
            "{name}: {errors:?}"
        );
    }
    Ok(())
}

#[test]
fn acceptance_negative_rejects_short_quiet_and_secret_bytes() -> Result<(), Box<dyn Error>> {
    let campaign = fixture("acceptance-short-quiet-campaign.tsv")?;
    let errors = inspect_attachment(Path::new("campaign.tsv"), &campaign, SOURCE);
    assert!(errors
        .iter()
        .any(|error| error.contains("shorter than 900")));
    assert!(
        errors
            .iter()
            .filter(|error| error.contains("quiet or missing"))
            .count()
            >= 4
    );

    let mut secret = String::from_utf8(vec![
        65, 117, 116, 104, 111, 114, 105, 122, 97, 116, 105, 111, 110,
    ])?;
    secret.push_str(": Bearer concealed-value-123456789012");
    let errors = inspect_attachment(Path::new("secret.bin"), secret.as_bytes(), SOURCE);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("bytes suppressed"));
    assert!(!errors[0].contains("concealed-value"));

    let safe = b"disk-cache-key-keeps-builds-small\nAuthorization: Bearer short-example";
    let errors = inspect_attachment(Path::new("safe.txt"), safe, SOURCE);
    assert!(errors.is_empty(), "false positive: {errors:?}");
    Ok(())
}

#[test]
fn acceptance_negative_rejects_malformed_tracked_plans() -> Result<(), Box<dyn Error>> {
    let target = temp("plans")?;
    copy_plans(&target)?;
    fs::write(
        target.join("evaluation/workgraph.tsv"),
        fixture("acceptance-malformed-workgraph.tsv")?,
    )?;
    let errors = validate_plans(&target)
        .err()
        .ok_or("malformed workgraph passed")?;
    assert!(errors
        .iter()
        .any(|error| error.contains("duplicate dependency")));
    assert!(errors
        .iter()
        .any(|error| error.contains("unknown dependency")));
    assert!(errors
        .iter()
        .any(|error| error.contains("outside final ancestry")));

    fs::copy(
        root().join("evaluation/workgraph.tsv"),
        target.join("evaluation/workgraph.tsv"),
    )?;
    fs::write(
        target.join("evaluation/experiment-plan.tsv"),
        fixture("acceptance-nonconcrete-experiment.tsv")?,
    )?;
    let errors = validate_plans(&target)
        .err()
        .ok_or("nonconcrete experiment passed")?;
    assert!(errors.iter().any(|error| error.contains("nonconcrete")));
    assert!(errors
        .iter()
        .any(|error| error.contains("unknown scenario")));
    assert!(errors.iter().any(|error| error.contains("repeat bounds")));
    fs::remove_dir_all(target)?;
    Ok(())
}

#[test]
fn acceptance_negative_rejects_source_drift_untracked_and_history_secret(
) -> Result<(), Box<dyn Error>> {
    let target = temp("repo")?;
    copy_plans(&target)?;
    git(&target, &["init", "-q"])?;
    git(
        &target,
        &["config", "user.email", "acceptance@example.invalid"],
    )?;
    git(&target, &["config", "user.name", "Acceptance Test"])?;
    git(&target, &["add", "evaluation"])?;
    git(&target, &["commit", "-q", "-m", "plans"])?;
    let source = git(&target, &["rev-parse", "HEAD"])?;
    let wrong_source = verify(&target, SOURCE, Path::new("evaluation/evidence/missing"));
    assert!(has(&wrong_source.errors, "source"));

    let evidence = target.join("evaluation/evidence").join(&source);
    let missing = verify(&target, &source, &evidence);
    assert!(has(&missing.errors, "missing"));

    fs::create_dir_all(&evidence)?;
    fs::write(evidence.join("attachment.tsv"), "raw_count\t1\n")?;
    let untracked = verify(&target, &source, &evidence);
    assert!(has(&untracked.errors, "untracked"));
    assert!(!untracked.missing.is_empty());

    git(&target, &["add", "evaluation/evidence"])?;
    git(&target, &["commit", "-q", "-m", "tracked evidence"])?;
    let tracked = verify(&target, &source, &evidence);
    assert!(tracked.errors.is_empty(), "{:?}", tracked.errors);
    assert!(!tracked.missing.is_empty());

    fs::write(target.join("product.txt"), "source drift\n")?;
    git(&target, &["add", "product.txt"])?;
    git(&target, &["commit", "-q", "-m", "product drift"])?;
    let drift = verify(&target, &source, &evidence);
    assert!(has(&drift.errors, "outside source evidence"));

    let secret = ["sk", "-", "abcdefghijklmnopqrstuvwxyz1234"].concat();
    fs::write(target.join("old-secret.bin"), secret)?;
    git(&target, &["add", "old-secret.bin"])?;
    git(&target, &["commit", "-q", "-m", "old object"])?;
    git(&target, &["rm", "-q", "old-secret.bin"])?;
    git(&target, &["commit", "-q", "-m", "remove object"])?;
    let errors = scan_history(&target);
    assert!(has(&errors, "bytes suppressed"));
    assert!(errors
        .iter()
        .all(|error| !error.contains("abcdefghijklmnopqrstuvwxyz")));
    fs::remove_dir_all(target)?;
    Ok(())
}
