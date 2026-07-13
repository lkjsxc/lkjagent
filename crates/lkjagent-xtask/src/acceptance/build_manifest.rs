use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

pub fn binary(
    root: &Path,
    evidence: &Path,
    source: &str,
    development: bool,
    campaign_source: Option<&str>,
) -> Option<String> {
    let text = std::fs::read_to_string(evidence.join("build-manifest.tsv")).ok()?;
    let mut lines = text.lines();
    (lines.next()? == "field\tvalue").then_some(())?;
    let fields = lines
        .map(|line| line.split_once('\t'))
        .collect::<Option<BTreeMap<_, _>>>()?;
    let final_binary = *fields.get("binary_sha256")?;
    let development_binary = *fields.get("development_binary_sha256")?;
    if fields.len() != 8
        || fields.get("source_commit") != Some(&source)
        || !hash(final_binary)
        || !hash(development_binary)
        || !source_hash(
            root,
            source,
            "crates/lkjagent-xtask/src/acceptance.rs",
            fields.get("checker_sha256")?,
        )
        || !source_hash(
            root,
            source,
            "crates/lkjagent-store/src/native-schema.sql",
            fields.get("schema_sha256")?,
        )
        || !source_hash(
            root,
            source,
            "evaluation/experiment-plan.tsv",
            fields.get("profile_plan_sha256")?,
        )
    {
        return None;
    }
    let development_source = *fields.get("development_source_commit")?;
    if development_source.len() != 40
        || development_source == source
        || !ancestor(root, development_source, source)
        || (development && campaign_source != Some(development_source))
        || fields.get("manifest_kind") != Some(&"source-build-binding")
    {
        return None;
    }
    Some(
        if development {
            development_binary
        } else {
            final_binary
        }
        .to_string(),
    )
}
fn source_hash(root: &Path, source: &str, path: &str, expected: &&str) -> bool {
    let spec = format!("{source}:{path}");
    Command::new("git")
        .args(["show", &spec])
        .current_dir(root)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .is_some_and(|output| digest(&output.stdout) == **expected)
}
fn ancestor(root: &Path, older: &str, newer: &str) -> bool {
    Command::new("git")
        .args(["merge-base", "--is-ancestor", older, newer])
        .current_dir(root)
        .status()
        .is_ok_and(|status| status.success())
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
