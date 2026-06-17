use std::path::Path;

use crate::config::{load_or_initialize, ConfigLoad};
use crate::error::CliError;
use crate::store::{now_stamp, open_store};

pub fn run(data_dir: &Path) -> Result<String, CliError> {
    let config = match load_or_initialize(data_dir)? {
        ConfigLoad::WroteDefault { path } => {
            return Err(CliError::failure(format!(
                "config_written={}\nmissing=endpoint.model",
                path.to_string_lossy()
            )));
        }
        ConfigLoad::Ready(config) => config,
    };
    let conn = open_store(data_dir)?;
    let holder = format!("pid{}", std::process::id());
    let now = now_stamp();
    match lkjagent_runtime::daemon::take_daemon_lock(&conn, &holder, &now, "0")? {
        lkjagent_runtime::daemon::StartupLock::Taken
        | lkjagent_runtime::daemon::StartupLock::Reclaimed { .. } => {
            lkjagent_store::state::set(&conn, "daemon state", "maintaining")?;
            lkjagent_store::state::set(&conn, "endpoint model", &config.endpoint_model)?;
            lkjagent_store::state::set(&conn, "daemon state", "stopped")?;
            lkjagent_store::state::delete(&conn, "daemon lock")?;
            Ok("startup=ok\nshutdown=clean".to_string())
        }
        lkjagent_runtime::daemon::StartupLock::Refused { holder } => {
            Err(CliError::failure(format!("daemon_refused={holder}")))
        }
    }
}
