use std::fs;
use std::path::{Path, PathBuf};

use crate::artifact_content_atom_profile::{atoms, profile, AtomSpec};
use crate::error::ToolResult;

pub(crate) struct AtomContract {
    pub selected: Vec<String>,
    pub valid_example: String,
    pub response: String,
}

struct AtomState {
    path: String,
    label: &'static str,
    words: usize,
}

pub(crate) fn contract_if_missing(
    root: &str,
    kind: &str,
    full: &Path,
) -> ToolResult<Option<AtomContract>> {
    let Some(profile) = profile(root, kind) else {
        return Ok(None);
    };
    if profile == "story" {
        return Ok(None);
    }
    let states = states(full, atoms(profile))?;
    let missing = missing(&states);
    if missing.is_empty() {
        return Ok(None);
    }
    let selected = missing.iter().take(3).cloned().collect::<Vec<_>>();
    let valid_example = crate::artifact_next_example::batch_write_contract(root, kind, &selected);
    let response = format!(
        "artifact_atom_profile={profile}\n{}\n{}",
        status_lines("missing", missing.len(), missing.first(), atoms(profile)),
        crate::artifact_next_response::batch_response(root, kind, &selected, &valid_example)
    );
    Ok(Some(AtomContract {
        selected,
        valid_example,
        response,
    }))
}

pub(crate) fn readiness_report(
    root: &str,
    kind: &str,
    full: &Path,
    report: String,
) -> ToolResult<String> {
    let Some(profile) = profile(root, kind) else {
        return Ok(report);
    };
    let states = states(full, atoms(profile))?;
    let missing = missing(&states);
    if missing.is_empty() {
        let passed = crate::artifact_readiness::content_bearing(report)
            .replace("readiness=content-bearing", "readiness=content-atoms-ready");
        return Ok(format!(
            "{passed}\nartifact_atom_profile={profile}\n{}",
            status_lines("ready", 0, None, atoms(profile))
        ));
    }
    Ok(format!(
        "artifact audit failed\nroot={root}\nreadiness=missing-content-atoms\nartifact_atom_profile={profile}\n{}\nfailed=1\nfailures:\n- content_atoms_missing: {}\nnext_decision_required=true\ncandidate_action=artifact.next",
        status_lines("missing", missing.len(), missing.first(), atoms(profile)),
        missing.join(",")
    ))
}

pub(crate) fn status_lines_for_profile(
    profile: &str,
    status: &str,
    missing_count: usize,
    next_atom: Option<&String>,
) -> String {
    status_lines(status, missing_count, next_atom, atoms(profile))
}

fn status_lines(
    status: &str,
    missing_count: usize,
    next_atom: Option<&String>,
    atoms: &[AtomSpec],
) -> String {
    let required = atoms
        .iter()
        .map(|atom| atom.path)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "atom_status={status}\natom_missing_count={missing_count}\nnext_atom={}\nrequired_atoms={required}",
        next_atom.map(String::as_str).unwrap_or("none")
    )
}

fn states(root: &Path, atoms: &[AtomSpec]) -> ToolResult<Vec<AtomState>> {
    let files = markdown_files(root)?;
    Ok(atoms
        .iter()
        .map(|atom| {
            let text = files
                .iter()
                .find(|file| file.0 == atom.path)
                .map(|file| file.1.as_str())
                .unwrap_or("");
            AtomState {
                path: atom.path.to_string(),
                label: atom.label,
                words: word_count(text),
            }
        })
        .collect())
}

fn missing(states: &[AtomState]) -> Vec<String> {
    states
        .iter()
        .filter(|state| state.words < 40 || !has_label(&state.path, state.label))
        .map(|state| state.path.clone())
        .collect()
}

fn has_label(path: &str, label: &str) -> bool {
    path.contains(label) || label == "content"
}

fn markdown_files(root: &Path) -> ToolResult<Vec<(String, String)>> {
    let mut paths = Vec::new();
    collect(root, root, &mut paths)?;
    paths
        .into_iter()
        .map(|(relative, path)| fs::read_to_string(path).map(|text| (relative, text)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn collect(root: &Path, current: &Path, files: &mut Vec<(String, PathBuf)>) -> ToolResult<()> {
    if current.is_dir() {
        for entry in fs::read_dir(current)? {
            collect(root, &entry?.path(), files)?;
        }
    } else if current.extension().is_some_and(|ext| ext == "md") && !is_readme(current) {
        files.push((relative_path(root, current), current.to_path_buf()));
    }
    Ok(())
}

fn is_readme(path: &Path) -> bool {
    path.file_name().and_then(|name| name.to_str()) == Some("README.md")
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}
