use rusqlite::Connection;

use lkjagent_runtime::daemon::{DaemonTick, ResidentDaemon};

use super::TestResult;

pub fn poll_until_done(
    daemon: &mut ResidentDaemon,
    conn: &mut Connection,
    times: &[&str],
) -> TestResult<()> {
    let mut ticks = Vec::new();
    for time in times {
        let tick = daemon.poll_once(conn, time)?;
        ticks.push(format!("{time}:{tick:?}"));
        if matches!(tick, DaemonTick::Done | DaemonTick::Idle) {
            return Ok(());
        }
    }
    Err(format!(
        "daemon did not reach Done within poll budget; ticks={} recent={}",
        ticks.join(","),
        recent_frames(daemon)
    )
    .into())
}

fn recent_frames(daemon: &ResidentDaemon) -> String {
    daemon
        .state
        .context
        .log
        .iter()
        .rev()
        .take(6)
        .map(|frame| frame.content.replace('\n', " "))
        .collect::<Vec<_>>()
        .join(" || ")
}
