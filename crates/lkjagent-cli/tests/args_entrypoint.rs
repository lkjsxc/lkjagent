mod support;

use lkjagent_cli::run_cli;
use support::{temp_data, TestResult};

#[test]
fn help_prints_command_summary() {
    let help = run_cli(["--help"]);

    assert_eq!(help.code, 0);
    assert!(help.stderr.is_empty());
    assert!(help.stdout.contains("usage: lkjagent"));
    assert!(help.stdout.contains("send <text>"));
}

#[test]
fn data_option_is_global_after_command() -> TestResult<()> {
    let data = temp_data("data-after-command")?;
    let path = data.to_string_lossy();

    let sent = run_cli(["send", "--data", path.as_ref(), "hello"]);
    assert_eq!(sent.code, 0);
    assert_eq!(sent.stdout, "queue_id=1");

    let status = run_cli(["status", "--data", path.as_ref()]);
    assert_eq!(status.code, 0);
    assert!(status.stdout.contains("queue_depth=1"));
    assert!(status.stdout.contains(&format!(
        "model_log={}",
        data.join("logs/current-model-run.md").to_string_lossy()
    )));
    Ok(())
}

#[test]
fn no_arg_commands_reject_extra_arguments() {
    let status = run_cli(["status", "unexpected"]);

    assert_eq!(status.code, 2);
    assert_eq!(status.stderr, "status takes no arguments");
}

#[test]
fn double_dash_keeps_flag_like_command_text() -> TestResult<()> {
    let data = temp_data("literal-flag-text")?;
    let path = data.to_string_lossy();

    let sent = run_cli(["--data", path.as_ref(), "send", "--", "--literal"]);
    assert_eq!(sent.code, 0);

    let log = run_cli(["--data", path.as_ref(), "log", "--full"]);
    assert!(log.stdout.contains("--literal"));
    Ok(())
}
