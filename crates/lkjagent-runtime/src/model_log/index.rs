use std::fs;
use std::path::Path;

use super::{json_escape, ProviderLogContext, ProviderLogHandle};
use crate::error::{RuntimeError, RuntimeResult};

pub fn record_provider_index(
    root: &Path,
    handle: &ProviderLogHandle,
    context: &ProviderLogContext,
) -> RuntimeResult<()> {
    let path = root.join("index.ndjson");
    let mut content = match fs::read_to_string(&path) {
        Ok(existing) => existing,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(io_error(error)),
    };
    content.push_str(&format!(
        "{{\"id\":\"{}\",\"case_id\":\"{}\",\"turn_id\":{},\"created_at\":\"{}\",\"path\":\"{}\"}}\n",
        json_escape(&handle.id),
        json_escape(&context.case_id),
        context.turn_id,
        json_escape(&context.created_at),
        json_escape(&handle.dir.to_string_lossy())
    ));
    atomic_write(&path, &content)
}

fn atomic_write(path: &Path, content: &str) -> RuntimeResult<()> {
    let parent = path
        .parent()
        .ok_or_else(|| RuntimeError::Store("provider index path has no parent".to_string()))?;
    fs::create_dir_all(parent).map_err(io_error)?;
    let tmp = path.with_extension("ndjson.tmp");
    fs::write(&tmp, content).map_err(io_error)?;
    fs::rename(&tmp, path).map_err(io_error)?;
    Ok(())
}

fn io_error(error: std::io::Error) -> RuntimeError {
    RuntimeError::Store(error.to_string())
}
