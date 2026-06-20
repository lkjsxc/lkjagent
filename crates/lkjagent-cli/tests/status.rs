mod support;

use lkjagent_cli::run_cli;
use support::{open_store, temp_data, TestResult};

#[test]
fn status_prints_ranked_active_state_tracks() -> TestResult<()> {
    let data = temp_data("status-tracks")?;
    let conn = open_store(&data)?;
    lkjagent_runtime::graph_state::open_owner_case(
        &conn,
        "Create structured documentation for lkjagent.",
        "2026-06-20T00:00:00Z",
    )?;

    let status = run_cli(["--data", data.to_string_lossy().as_ref(), "status"]);

    assert!(status.stdout.contains("active_states=1."));
    assert!(status.stdout.contains("document-structure"));
    assert!(status.stdout.contains("phase=planning"));
    Ok(())
}
