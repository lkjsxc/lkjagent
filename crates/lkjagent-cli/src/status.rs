use std::path::Path;

use crate::error::CliError;
use crate::status_deck;
use crate::store::open_store;

pub fn status(data_dir: &Path) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let deck = status_deck::load(data_dir, &conn)?;
    Ok(status_deck::render_status(&deck))
}
