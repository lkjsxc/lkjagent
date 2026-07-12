use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use lkjagent_xtask::acceptance::{
    derive_attachment, inspect_attachment, scan_history, source_contract_files, source_contracts,
};

const SOURCE: &str = "2222222222222222222222222222222222222222";

static NEXT: AtomicU64 = AtomicU64::new(0);

fn temp() -> Result<PathBuf, Box<dyn Error>> {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "lkjagent-acceptance-history-{}-{id}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn git(root: &Path, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git").args(args).current_dir(root).output()?;
    if !output.status.success() {
        return Err(format!("git command failed: {args:?}").into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

#[test]
fn acceptance_negative_allows_only_checker_result_status_rows() {
    let bytes = b"predicate_id\tcategory\tderived_status\tevidence_path\tmeasured_value\tchecker_sha256\tpredicate_schema_sha256\nA01\tchecker\tpass\tfacts.tsv\tderived\t1111\t2222\n";
    let claimed = inspect_attachment(Path::new("claimed.tsv"), bytes, SOURCE);
    assert!(claimed.iter().any(|error| error.contains("editable pass")));
    assert!(inspect_attachment(Path::new("result.tsv"), bytes, SOURCE).is_empty());
}

#[test]
fn exact_campaign_derivation_requires_all_semantic_facts() {
    let body = format!("field\tvalue\nsource_commit\t{SOURCE}\nscenario\texact-file-edit\nmode\trun\nsemantic_status\tevaluated\noutcome\tpassed\nsemantic_detail\tfile_exact=true;one_file=true;closed=4;owner=5;agent=4;passed_checks=12;effects=1;admissions=10;providers=24;tables=18\n");
    let path = Path::new("campaign-exact-file-edit-run.tsv");
    let derived = derive_attachment(path, body.as_bytes(), SOURCE);
    assert_eq!(
        derived,
        ["F01", "F07", "F08", "W02"]
            .into_iter()
            .map(str::to_string)
            .collect()
    );
    let altered = body.replace("file_exact=true", "file_exact=false");
    let derived = derive_attachment(path, altered.as_bytes(), SOURCE);
    assert_eq!(derived, ["F08"].into_iter().map(str::to_string).collect());
}

#[test]
fn source_contracts_require_exact_implementation_and_tests() -> Result<(), Box<dyn Error>> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let expected = [
        "C01", "C02", "F03", "F04", "P01", "P02", "P03", "P04", "P05", "P06", "P07", "R01", "R02",
        "R03", "R04", "R05", "R06", "R10", "S02", "X03",
    ];
    let actual = source_contracts(&repository);
    assert_eq!(
        actual.iter().map(String::as_str).collect::<Vec<_>>(),
        expected
    );

    let root = temp()?;
    for relative in source_contract_files() {
        let destination = root.join(relative);
        fs::create_dir_all(destination.parent().ok_or("contract path has no parent")?)?;
        fs::copy(repository.join(relative), destination)?;
    }
    assert_eq!(source_contracts(&root), actual);

    let implementation = root.join("crates/lkjagent-effects/src/workspace_capability.rs");
    let text = fs::read_to_string(&implementation)?;
    fs::write(
        &implementation,
        text.replace("OFlags::NOFOLLOW", "OFlags::empty()"),
    )?;
    assert!(!source_contracts(&root).contains("F03"));
    assert!(source_contracts(&root).contains("F04"));
    fs::create_dir_all(root.join("crates/lkjagent-app/src"))?;
    fs::write(
        root.join("crates/lkjagent-app/src/daemon_route_effects.rs"),
        "retired",
    )?;
    assert!(!source_contracts(&root).contains("S02"));
    fs::remove_file(root.join("crates/lkjagent-app/src/daemon_route_effects.rs"))?;

    fs::remove_file(root.join("crates/lkjagent-llm/tests/wire_contract.rs"))?;
    let incomplete = source_contracts(&root);
    assert!(!incomplete.contains("P07"));
    assert!(!incomplete.contains("X03"));
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn acceptance_negative_scans_head_not_unrelated_refs() -> Result<(), Box<dyn Error>> {
    let root = temp()?;
    git(&root, &["init", "-q"])?;
    git(&root, &["config", "user.email", "history@example.invalid"])?;
    git(&root, &["config", "user.name", "History Test"])?;
    fs::write(root.join("safe.txt"), "safe\n")?;
    git(&root, &["add", "safe.txt"])?;
    git(&root, &["commit", "-q", "-m", "safe"])?;
    let safe = git(&root, &["rev-parse", "HEAD"])?;

    git(&root, &["checkout", "-q", "-b", "unrelated"])?;
    let secret = ["sk", "-", "abcdefghijklmnopqrstuvwxyz1234"].concat();
    fs::write(root.join("secret.bin"), secret)?;
    git(&root, &["add", "secret.bin"])?;
    git(&root, &["commit", "-q", "-m", "unrelated object"])?;
    assert!(!scan_history(&root).is_empty());

    git(&root, &["checkout", "-q", "--detach", &safe])?;
    assert!(scan_history(&root).is_empty());
    fs::remove_dir_all(root)?;
    Ok(())
}
