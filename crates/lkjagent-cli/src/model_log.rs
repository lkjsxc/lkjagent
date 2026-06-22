use std::fs;
use std::path::Path;

use crate::config::load_context_policy_for_status;
use crate::error::CliError;
use crate::store::{now_stamp, open_store};

pub fn model_log(data_dir: &Path, print: bool) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let path = lkjagent_runtime::model_log::current_log_path(data_dir);
    let policy = load_context_policy_for_status(data_dir)?;
    lkjagent_runtime::model_log::write_current_log(&conn, &path, &now_stamp(), policy)?;
    if print {
        fs::read_to_string(path).map_err(Into::into)
    } else {
        Ok(format!("model_log={}", path.to_string_lossy()))
    }
}
