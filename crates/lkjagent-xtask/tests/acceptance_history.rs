use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use lkjagent_xtask::acceptance::scan_history;

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
