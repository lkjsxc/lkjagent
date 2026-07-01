use std::fs;
use std::path::PathBuf;

use super::db::load_store;
use super::files::load_files;
use super::model::{CollectOptions, ProofBundle};
use super::render;

pub fn collect(options: &CollectOptions) -> Result<PathBuf, String> {
    let mut bundle = ProofBundle::default();
    load_store(&mut bundle, &options.data_dir);
    load_files(&mut bundle, &options.data_dir);
    write_bundle(&bundle, &options.out_dir)
}

fn write_bundle(bundle: &ProofBundle, out_dir: &PathBuf) -> Result<PathBuf, String> {
    fs::create_dir_all(out_dir).map_err(|error| format!("create proof dir: {error}"))?;
    write(out_dir, "summary.md", &render::summary(bundle))?;
    write(out_dir, "status.md", &render::status(bundle))?;
    write(
        out_dir,
        "word-counts.md",
        &render::word_counts(&bundle.word_counts),
    )?;
    write(
        out_dir,
        "workspace.md",
        &render::file_index("Workspace Files", &bundle.workspace_files),
    )?;
    write(
        out_dir,
        "model-logs.md",
        &render::file_index("Model Log Files", &bundle.model_logs),
    )?;
    write(out_dir, "warnings.md", &render::warnings(&bundle.warnings))?;
    Ok(out_dir.join("summary.md"))
}

fn write(out_dir: &PathBuf, name: &str, content: &str) -> Result<(), String> {
    fs::write(out_dir.join(name), format!("{content}\n"))
        .map_err(|error| format!("write {name}: {error}"))
}
