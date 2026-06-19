use std::path::Path;
use std::time::Duration;

use crate::config::RuntimeConfig;
use crate::config::{load_or_initialize, ConfigLoad};
use crate::error::CliError;
use crate::paths::workspace;
use crate::store::{now_stamp, open_store};

pub fn run(data_dir: &Path) -> Result<String, CliError> {
    let workspace = workspace(data_dir)?;
    let config = match load_or_initialize(data_dir)? {
        ConfigLoad::WroteDefault { path } => {
            return Err(CliError::failure(format!(
                "config_written={}\nmissing=endpoint.model",
                path.to_string_lossy()
            )));
        }
        ConfigLoad::Ready(config) => config,
    };
    let mut conn = open_store(data_dir)?;
    let holder = format!("pid{}", std::process::id());
    let now = now_stamp();
    let stale_before = stale_before(&now, effective_lock_stale(&config));
    match lkjagent_runtime::daemon::take_daemon_lock(&conn, &holder, &now, &stale_before)? {
        lkjagent_runtime::daemon::StartupLock::Taken
        | lkjagent_runtime::daemon::StartupLock::Reclaimed { .. } => {
            lkjagent_store::state::set(&conn, "daemon state", "idle")?;
            lkjagent_store::state::set(&conn, "endpoint model", &config.endpoint_model)?;
            let prefix = lkjagent_runtime::daemon::build_prefix_from_store(&conn, &workspace)?;
            let mut state = lkjagent_runtime::daemon::startup_state(
                prefix,
                lkjagent_runtime::daemon::startup_summary(&conn)?,
            );
            state.turn = stored_turn(&conn)?;
            let runtime = lkjagent_runtime::daemon::ResidentRuntime::new(
                holder,
                client_config(&config),
                workspace,
                &now,
            )
            .with_budget(config.context_policy);
            let mut daemon = lkjagent_runtime::daemon::ResidentDaemon::new(state, runtime);
            lkjagent_runtime::daemon::restore_completion_guard(&conn, &mut daemon.dispatch_state)?;
            loop {
                let tick = daemon.poll_once(&mut conn, &now_stamp())?;
                if should_sleep(tick) {
                    std::thread::sleep(Duration::from_secs(1));
                }
            }
        }
        lkjagent_runtime::daemon::StartupLock::Refused { holder } => {
            Err(CliError::failure(format!("daemon_refused={holder}")))
        }
    }
}

fn client_config(config: &RuntimeConfig) -> lkjagent_runtime::daemon::EndpointClientConfig {
    lkjagent_runtime::daemon::client_config(
        &config.endpoint_url,
        &config.endpoint_model,
        secret_env(&config.api_key_env),
        config.endpoint_timeout_seconds,
        config.context_policy.reserve as u16,
    )
}

fn should_sleep(tick: lkjagent_runtime::daemon::DaemonTick) -> bool {
    matches!(
        tick,
        lkjagent_runtime::daemon::DaemonTick::Idle
            | lkjagent_runtime::daemon::DaemonTick::Waiting
            | lkjagent_runtime::daemon::DaemonTick::EndpointError
            | lkjagent_runtime::daemon::DaemonTick::Paused
    )
}

fn effective_lock_stale(config: &RuntimeConfig) -> u64 {
    config
        .daemon_lock_stale_seconds
        .max(config.endpoint_timeout_seconds.saturating_add(60))
}

fn stale_before(now: &str, stale_seconds: u64) -> String {
    now.parse::<u64>()
        .map(|stamp| stamp.saturating_sub(stale_seconds).to_string())
        .unwrap_or_else(|_| "0".to_string())
}

fn stored_turn(conn: &rusqlite::Connection) -> Result<i64, CliError> {
    Ok(lkjagent_store::state::get(conn, "turn")?
        .and_then(|turn| turn.parse::<i64>().ok())
        .unwrap_or(0))
}

fn secret_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
