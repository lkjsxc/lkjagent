use std::fs;
use std::path::PathBuf;

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

fn fixture_root(name: &str) -> TestResult<PathBuf> {
    let path = std::env::temp_dir().join(format!("lkjagent-effects-{name}-{}", std::process::id()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path.canonicalize()?)
}
