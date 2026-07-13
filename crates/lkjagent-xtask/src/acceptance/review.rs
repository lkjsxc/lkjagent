use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

pub fn valid(root: &Path, bytes: &[u8], source: &str) -> bool {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    let mut lines = text.lines();
    if lines.next() != Some("field\tvalue") {
        return false;
    }
    let fields = lines
        .filter_map(|line| line.split_once('\t'))
        .collect::<BTreeMap<_, _>>();
    if fields.len() != 9
        || fields.get("source_commit") != Some(&source)
        || fields.get("reviewer_kind") != Some(&"independent-subagent")
        || fields.get("blocker_count") != Some(&"0")
        || !hash(
            fields
                .get("review_capture_sha256")
                .copied()
                .unwrap_or_default(),
        )
    {
        return false;
    }
    let Some(acceptance) = source_bytes(root, source, "evaluation/acceptance.tsv") else {
        return false;
    };
    let Some(checker) = source_bytes(root, source, "crates/lkjagent-xtask/src/acceptance.rs")
    else {
        return false;
    };
    let Some(schema) = source_bytes(root, source, "crates/lkjagent-store/src/native-schema.sql")
    else {
        return false;
    };
    let required = String::from_utf8_lossy(&acceptance)
        .lines()
        .skip(1)
        .filter(|line| line.split('\t').nth(4) == Some("yes"))
        .count()
        .to_string();
    let acceptance_hash = digest(&acceptance);
    let checker_hash = digest(&checker);
    let schema_hash = digest(&schema);
    fields.get("acceptance_sha256") == Some(&acceptance_hash.as_str())
        && fields.get("checker_sha256") == Some(&checker_hash.as_str())
        && fields.get("schema_sha256") == Some(&schema_hash.as_str())
        && fields.get("reviewed_predicate_count") == Some(&required.as_str())
        && fields.get("review_command") == Some(&"acceptance verify --allow-incomplete")
}

fn source_bytes(root: &Path, source: &str, path: &str) -> Option<Vec<u8>> {
    let spec = format!("{source}:{path}");
    let output = Command::new("git")
        .args(["show", &spec])
        .current_dir(root)
        .output()
        .ok()?;
    output.status.success().then_some(output.stdout)
}
fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
fn hash(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
