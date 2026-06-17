use std::path::Path;

use crate::error::CliError;
use crate::store::{now_stamp, open_store};

pub fn send(data_dir: &Path, text: &str) -> Result<String, CliError> {
    let mut conn = open_store(data_dir)?;
    let id = lkjagent_store::queue::enqueue(&mut conn, text, "owner-send", &now_stamp())?;
    Ok(format!("queue_id={id}"))
}
