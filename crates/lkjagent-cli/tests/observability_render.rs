mod support;

use lkjagent_cli::console::render_snapshot;
use lkjagent_cli::run_cli;
use support::{open_store, temp_data, TestResult};

#[test]
fn status_output_uses_stable_section_keys() -> TestResult<()> {
    let data = temp_data("status-stable-keys")?;
    open_store(&data)?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);
    let keys = status
        .stdout
        .lines()
        .map(|line| line.split_once('=').map_or(line, |parts| parts.0))
        .collect::<Vec<_>>();

    assert_eq!(status.code, 0);
    assert_eq!(
        &keys[..8],
        &[
            "runtime.daemon_state",
            "runtime.turns",
            "runtime.continuation_epoch",
            "runtime.continuation_turns",
            "runtime.checkpoint_turns",
            "runtime.last_checkpoint_reason",
            "runtime.continuation_decision",
            "queue.pending",
        ]
    );
    assert!(keys.contains(&"task.active_case"));
    assert!(keys.contains(&"artifact.root"));
    assert!(keys.contains(&"tokens.usage"));
    assert!(keys.contains(&"model.log"));
    assert!(keys.contains(&"next.action"));
    Ok(())
}

#[test]
fn console_bottom_deck_uses_status_next_action_fact() -> TestResult<()> {
    let data = temp_data("console-shared-deck")?;
    let conn = open_store(&data)?;
    lkjagent_store::state::set(&conn, "authority next action", "artifact.next")?;

    let screen = render_snapshot(&data, "ready", 80, 18)?;

    assert!(screen.contains("next artifact.next"));
    Ok(())
}
