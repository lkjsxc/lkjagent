use std::fs;
use std::path::{Path, PathBuf};

use super::model::{FileRow, ProofBundle, ReadinessRow, WordCountRow};

const FILE_LIMIT: usize = 500;

pub fn load_files(bundle: &mut ProofBundle, data_dir: &Path) {
    let logs_dir = data_dir.join("logs");
    bundle.model_logs = scan_files(&logs_dir, "logs", &mut bundle.warnings);
    let workspace = data_dir.join("workspace");
    bundle.workspace_files = scan_files(&workspace, "workspace", &mut bundle.warnings);
    bundle.word_counts = count_roots(&workspace, &bundle.readiness, &mut bundle.warnings);
}

fn scan_files(dir: &Path, label: &str, warnings: &mut Vec<String>) -> Vec<FileRow> {
    if !dir.exists() {
        warnings.push(format!("{label} directory missing"));
        return Vec::new();
    }
    let mut rows = Vec::new();
    walk(dir, dir, &mut rows, warnings);
    rows.sort_by(|left, right| left.path.cmp(&right.path));
    if rows.len() > FILE_LIMIT {
        warnings.push(format!("{label} file list truncated at {FILE_LIMIT}"));
        rows.truncate(FILE_LIMIT);
    }
    if rows.is_empty() {
        warnings.push(format!("{label} file list empty"));
    }
    rows
}

fn walk(root: &Path, dir: &Path, rows: &mut Vec<FileRow>, warnings: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => {
            warnings.push(format!("read {} failed: {error}", dir.display()));
            return;
        }
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            walk(root, &path, rows, warnings);
        } else if path.is_file() {
            rows.push(file_row(root, &path));
        }
    }
}

fn file_row(root: &Path, path: &Path) -> FileRow {
    let bytes = fs::metadata(path).map_or(0, |metadata| metadata.len());
    FileRow {
        path: relative(root, path),
        bytes,
    }
}

fn count_roots(
    workspace: &Path,
    readiness: &[ReadinessRow],
    warnings: &mut Vec<String>,
) -> Vec<WordCountRow> {
    let mut rows = Vec::new();
    for item in readiness {
        let root = workspace.join(&item.root);
        if !root.exists() {
            warnings.push(format!(
                "artifact root missing from workspace: {}",
                item.root
            ));
            rows.push(WordCountRow {
                root: item.root.clone(),
                files: 0,
                words: 0,
                manuscript_files: 0,
                manuscript_words: 0,
            });
            continue;
        }
        rows.push(count_root(&item.root, &root, warnings));
    }
    rows
}

fn count_root(root_label: &str, root: &Path, warnings: &mut Vec<String>) -> WordCountRow {
    let mut files = Vec::new();
    collect_text_files(root, &mut files, warnings);
    let mut words = 0usize;
    let mut manuscript_files = 0usize;
    let mut manuscript_words = 0usize;
    for path in &files {
        let count = fs::read_to_string(path).map_or(0, |text| word_count(&text));
        words = words.saturating_add(count);
        if path_has_segment(path, "manuscript") {
            manuscript_files += 1;
            manuscript_words = manuscript_words.saturating_add(count);
        }
    }
    WordCountRow {
        root: root_label.to_string(),
        files: files.len(),
        words,
        manuscript_files,
        manuscript_words,
    }
}

fn collect_text_files(dir: &Path, out: &mut Vec<PathBuf>, warnings: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => {
            warnings.push(format!("read {} failed: {error}", dir.display()));
            return;
        }
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_text_files(&path, out, warnings);
        } else if matches!(
            path.extension().and_then(|value| value.to_str()),
            Some("md" | "txt")
        ) {
            out.push(path);
        }
    }
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(char::is_alphanumeric))
        .count()
}

fn path_has_segment(path: &Path, segment: &str) -> bool {
    path.components()
        .any(|part| part.as_os_str().to_string_lossy() == segment)
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|value| value.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
        .replace('\\', "/")
}
