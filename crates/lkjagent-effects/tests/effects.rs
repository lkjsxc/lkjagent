use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use lkjagent_core::model::CheckSpec;
use lkjagent_effects::checks::run_check;
use lkjagent_effects::exchange::{write_exchange, ExchangeFiles};
use lkjagent_effects::observation::observation;
use lkjagent_effects::shell;
use lkjagent_effects::workspace;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn path_guard_rejects_escape_and_symlink_out() -> TestResult<()> {
    let root = fixture_root("path")?;
    assert!(workspace::resolve(&root, "../x").is_err());
    assert!(workspace::resolve(&root, "/tmp/x").is_err());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("/tmp", root.join("out"))?;
        assert!(workspace::resolve(&root, "out/file").is_err());
    }
    Ok(())
}

#[test]
fn workspace_shell_observation_and_exchange_work() -> TestResult<()> {
    let root = fixture_root("ops")?;
    workspace::write(&root, "a/readme.md", "hello\nworld\nRelease")?;
    let read = workspace::read(&root, "a/readme.md", 0, 2)?;
    assert!(read.contains("total=3"));
    assert!(workspace::list(&root, ".", 2)?.contains("a/readme.md"));
    assert!(workspace::tree(&root, ".", 2)?.contains("dir a"));
    assert!(workspace::search(&root, ".", "release")?.contains("Release"));

    let report = shell::run(&root, "printf ok", 30)?;
    assert!(report.success());
    assert_eq!(report.output, "ok");
    assert!(observation("ok", &"x".repeat(7000)).contains("[...]"));

    let paths = write_exchange(
        &root.join("logs"),
        1,
        2,
        3,
        ExchangeFiles {
            request: "{}",
            response: "{}",
            outcome: "{}",
            timing: "{}",
        },
    )?;
    assert_eq!(fs::read_to_string(paths.request)?, "{}");
    Ok(())
}

#[test]
fn shell_bounds_only_its_background_and_detached_descendants() -> TestResult<()> {
    let root = fixture_root("shell-background")?;
    let mut unrelated = Command::new("sleep").arg("5").spawn()?;
    fs::write(
        root.join("chain.sh"),
        "#!/bin/sh\nif [ \"$1\" -eq 0 ]; then echo $$ > detached.pid; sleep 5; else sh \"$0\" $(( $1 - 1 )) & wait; fi\n",
    )?;
    let detached = "setsid sh chain.sh 12 & while [ ! -s detached.pid ]; do :; done";
    for command in ["sleep 5 &", detached] {
        let started = Instant::now();
        let report = shell::run(&root, command, 1)?;
        assert!(started.elapsed() < Duration::from_secs(3));
        assert!(report.timed_out);
        assert!(!report.success());
    }
    let pid = fs::read_to_string(root.join("detached.pid"))?;
    let status = fs::read_to_string(PathBuf::from(format!("/proc/{}/status", pid.trim())))
        .unwrap_or_default();
    assert!(status.is_empty() || status.contains("State:\tZ"));
    let escaped = "env -u LKJAGENT_SHELL_SCOPE setsid sh -c 'echo $$ > escaped.pid; sleep 5' & while [ ! -s escaped.pid ]; do :; done";
    let started = Instant::now();
    let report = shell::run(&root, escaped, 1)?;
    assert!(started.elapsed() < Duration::from_secs(2));
    assert!(report.timed_out);
    let escaped_pid = fs::read_to_string(root.join("escaped.pid"))?;
    assert!(Command::new("kill")
        .arg("-KILL")
        .arg(escaped_pid.trim())
        .status()?
        .success());
    let started = Instant::now();
    let hot = shell::run(&root, "while :; do printf 0123456789; done", 1)?;
    assert!(started.elapsed() < Duration::from_secs(2));
    assert!(hot.timed_out);
    assert!(hot.output.len() <= shell::SHELL_OUTPUT_BYTES);
    assert!(unrelated.try_wait()?.is_none());
    unrelated.kill()?;
    unrelated.wait()?;
    Ok(())
}

#[test]
fn combined_catalog_checks_match_expected_results() -> TestResult<()> {
    let root = fixture_root("checks")?;
    workspace::write(&root, "docs/README.md", "# Docs\n\n- [page.md](page.md)\n")?;
    workspace::write(&root, "docs/page.md", "Release words here\n")?;
    let specs = vec![
        CheckSpec::FileExists {
            path: "docs/page.md".to_string(),
        },
        CheckSpec::MinWords {
            path: "docs/page.md".to_string(),
            n: 3,
        },
        CheckSpec::MinWordsTotal {
            glob: "docs/*.md".to_string(),
            n: 5,
        },
        CheckSpec::MaxLines {
            path: "docs/page.md".to_string(),
            n: 2,
        },
        CheckSpec::FileCount {
            glob: "docs/*.md".to_string(),
            min: 2,
            max: Some(2),
        },
        CheckSpec::Contains {
            path: "docs/page.md".to_string(),
            needle: "Release".to_string(),
        },
        CheckSpec::Absent {
            path: "docs/page.md".to_string(),
            needle: "scaffold".to_string(),
        },
        CheckSpec::ReadmeCoverage {
            root: "docs".to_string(),
        },
        CheckSpec::LinksResolve {
            root: "docs".to_string(),
        },
        CheckSpec::Command {
            cmd: "test -f docs/page.md".to_string(),
        },
    ];
    let results = specs
        .iter()
        .map(|spec| run_check(&root, spec))
        .collect::<Result<Vec<_>, _>>()?;
    assert!(results.iter().all(|result| result.passed));
    assert_eq!(
        results
            .iter()
            .map(|result| result.name.as_str())
            .collect::<Vec<_>>()
            .len(),
        10
    );
    Ok(())
}

#[test]
fn missing_tree_checks_return_failed_results_not_io_errors() -> TestResult<()> {
    let root = fixture_root("missing-tree")?;
    let result = run_check(
        &root,
        &CheckSpec::ReadmeCoverage {
            root: "docs/missing".to_string(),
        },
    )?;
    assert!(!result.passed);
    Ok(())
}

#[test]
#[rustfmt::skip]
fn shell_does_not_inherit_endpoint_credentials() -> TestResult<()> {
    if std::env::var_os("LKJAGENT_SECRET_TEST_CHILD").is_some() {
        let root = fixture_root("shell-secret")?;
        let report = shell::run(&root, "test -z \"${LKJAGENT_API_KEY+x}\" && printf clean", 5)?;
        assert_eq!(report.output, "clean"); return Ok(());
    }
    let status = Command::new(std::env::current_exe()?)
        .args(["--exact", "shell_does_not_inherit_endpoint_credentials", "--nocapture"])
        .env("LKJAGENT_SECRET_TEST_CHILD", "1").env("LKJAGENT_API_KEY", "must-not-escape").status()?;
    assert!(status.success()); Ok(())
}

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-effects-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path.canonicalize()?)
}
