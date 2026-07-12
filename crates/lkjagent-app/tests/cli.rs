use std::fs;

use lkjagent_app::cli;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn removed_commands_create_no_data_or_workspace() -> TestResult<()> {
    let parent = std::env::temp_dir().join(format!("lkjagent-removed-cli-{}", std::process::id()));
    if parent.exists() {
        fs::remove_dir_all(&parent)?;
    }
    let data = parent.join("data");
    let commands = vec![
        vec!["console"],
        vec!["workbench"],
        vec!["workspace", "--rebuild"],
        vec!["workspace", "search", "query"],
        vec!["workspace", "plan-rebalance"],
        vec!["workspace", "apply-rebalance"],
        vec!["workspace", "validate"],
        vec!["log", "--follow"],
        vec!["matter", "list"],
        vec!["queue", "list"],
        vec!["context", "resolve", "matter", "key", "item"],
        vec!["record", "list"],
        vec!["memory", "query"],
        vec!["watch"],
        vec!["today", "entry"],
        vec!["journal", "entry"],
        vec!["todo", "entry"],
        vec!["calendar", "entry"],
        vec!["finance", "entry"],
        vec!["note", "entry"],
        vec!["project", "entry"],
        vec!["artifact", "entry"],
        vec!["dev", "entry"],
    ];
    for command in commands {
        let mut args = vec!["--data", data.to_str().ok_or("non-UTF-8 data path")?];
        args.extend(command);
        assert!(cli::run(args).is_err());
        assert!(!parent.exists(), "removed command created storage");
    }
    Ok(())
}
