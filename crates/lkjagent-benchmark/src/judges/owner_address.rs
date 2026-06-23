use std::fs;
use std::path::Path;

pub fn artifact_address_controller(workspace: &Path) -> Result<(), String> {
    let text = read_any(workspace)?;
    require_all(
        &text,
        &[
            "fixture=sf-novel-file-root-audit-loop",
            "address_status=root_is_file",
            "normalized_root=stories/sf-novel-with-detailed-settings",
            "weak_path=characters/protagonist.md",
            "next_action=fs.batch_write",
            "fixture=artifact-next-file-root-missing-zero-false",
            "file_root_audit_example=absent",
            "missing=not-zero",
            "fixture=markdown-suffix-directory-created-by-artifact-apply",
            "address_status=root_ends_with_markdown_suffix",
            "directory_created=false",
            "fixture=batch-write-json-in-files",
            "fs.batch_write input_format=json-array",
            "files_written=2",
            "canonical_grammar=line-protocol",
            "fixture=oversized-fs-write-after-recovery",
            "fs.write payload_too_large=blocked",
            "split_semantic_files=required",
        ],
    )?;
    forbid_any(
        &text,
        &[
            "io error: Not a directory",
            "root=stories/sf-novel-with-detailed-settings/characters/protagonist.md\nmissing=0",
            "next_action=artifact.audit\nroot=stories/sf-novel-with-detailed-settings/characters/protagonist.md",
            "directory_created=true",
            "markdown_suffix_directory",
            "json_payload=refused",
            "partial_write=present",
            "retry raw fs.write",
            "split_semantic_files=absent",
        ],
    )
}

fn read_any(workspace: &Path) -> Result<String, String> {
    for path in ["transcript.md", "run.log"] {
        let candidate = workspace.join(path);
        if candidate.is_file() {
            return fs::read_to_string(&candidate)
                .map_err(|error| format!("{} unreadable: {error}", candidate.display()));
        }
    }
    Err("none of transcript.md or run.log exists".to_string())
}

fn require_all(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if !text.contains(needle) {
            return Err(format!("missing {needle}"));
        }
    }
    Ok(())
}

fn forbid_any(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if text.contains(needle) {
            return Err(format!("forbidden stale shape {needle}"));
        }
    }
    Ok(())
}
