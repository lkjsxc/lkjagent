use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;
const BASELINE: &str = "97e00698f348fc2435d47a107b5b8453c98b9d1f";

#[derive(Clone)]
pub struct Facts {
    values: BTreeMap<String, String>,
}
impl Facts {
    #[rustfmt::skip]
    pub fn from_path(path: &Path) -> Result<Self, String> { Ok(Self { values: pairs(path)? }) }
    #[rustfmt::skip]
    pub fn expected_error(&self) -> Option<&str> { self.values.get("expected_error").map(String::as_str) }
    #[rustfmt::skip]
    pub fn set(&mut self, key: &str, value: &str) { self.values.insert(key.into(), value.into()); }
}
#[rustfmt::skip]
pub fn validate(facts: &Facts) -> Vec<String> {
    let value = |key: &str| facts.values.get(key).map(String::as_str);
    let number = |key: &str| value(key).and_then(|item| item.parse::<u64>().ok());
    let mut errors = Vec::new();
    if value("actual_terminal") != value("claimed_terminal") { errors.push("claimed terminal contradicts raw terminal".into()); }
    if value("skipped") != Some("false") { errors.push("required command was skipped".into()); }
    if value("generated_placeholder") != Some("false") { errors.push("generated placeholder is not evidence".into()); }
    if number("test_count") == Some(0) { errors.push("test count must be positive".into()); }
    if number("useful_decision_count").is_some_and(|count| count < 5) { errors.push("useful decision floor not met".into()); }
    errors
}
#[rustfmt::skip]
pub fn fixture_errors(path: &Path) -> Result<Vec<String>, String> { Facts::from_path(path).map(|facts| validate(&facts)) }

#[rustfmt::skip]
pub fn check_baseline(root: &Path, source: Option<&str>) -> Result<String, Vec<String>> {
    let requested = source.unwrap_or(BASELINE);
    if !full_commit(requested) || requested != BASELINE { return Err(vec!["baseline source must be its exact full tracked commit".into()]); }
    let bundle = root.join("evaluation/evidence").join(requested).join("baseline");
    if !bundle.is_dir() { return Err(vec!["baseline evidence bundle is unavailable".into()]); }
    let mut errors = Vec::new(); let has_git=git_ok(root,&["rev-parse","--is-inside-work-tree"]); if has_git { git_source(root, requested, &mut errors); }
    let expected = manifest_rows(&bundle.join("manifest.sha256"), &mut errors);
    let actual = bundle_files(&bundle, &mut errors);
    if expected.keys().cloned().collect::<BTreeSet<_>>() != actual { errors.push("baseline manifest coverage is not exact".into()); }
    for (name, digest) in &expected {
        match fs::read(bundle.join(name)) {
            Ok(bytes) if format!("{:x}", Sha256::digest(&bytes)) == *digest => {},
            Ok(_) => errors.push(format!("baseline hash differs: {name}")),
            Err(error) => errors.push(format!("baseline file unreadable {name}: {error}")),
        }
    }
    let manifest_name = "manifest.sha256".to_string();
    if has_git { for name in actual.iter().chain(std::iter::once(&manifest_name)) {
        let relative = format!("evaluation/evidence/{requested}/baseline/{name}");
        if !git_ok(root, &["ls-files", "--error-unmatch", "--", &relative]) { errors.push(format!("baseline file is not tracked: {name}")); }
    } }
    semantic_checks(&bundle, requested, &mut errors);
    for name in expected.keys() {
        if let Ok(bytes) = fs::read(bundle.join(name)) {
            if secret(&bytes) { errors.push(format!("secret pattern in baseline file: {name}")); }
            let words = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
            if words.split(|c: char| !c.is_ascii_alphanumeric()).any(|word| word == "success" || word == "pass") {
                errors.push(format!("semantic success/pass label in baseline file: {name}"));
            }
        }
    }
    if errors.is_empty() { Ok(format!("ok evidence campaign=baseline source={requested} outcome=blocked semantic_success=false")) } else { Err(errors) }
}

#[rustfmt::skip]
fn semantic_checks(bundle: &Path, source: &str, errors: &mut Vec<String>) {
    let outcome = read_pairs(&bundle.join("outcome.tsv"), errors);
    for (key, expected) in [("source_commit", source), ("outcome", "blocked"), ("observed_seconds", "901"),
        ("exact_target_change", "false"), ("failure", "unsupported_executor"), ("model_call_count", "0"),
        ("runtime_decision_count", "0"), ("effect_count", "0"), ("check_count", "0")] { require(&outcome, key, expected, errors); }
    let process = read_pairs(&bundle.join("process-status.tsv"), errors);
    for (key, expected) in [("process-after-2-seconds", "alive"), ("process-after-901-seconds", "alive"),
        ("process-stop", "harness-sent-termination-after-observation"), ("process-stop-exit", "143"),
        ("public-status-after-901-exit", "0"), ("provider-exchanges-after-901", "0"),
        ("admissions-after-901", "0"), ("observations-after-901", "0"), ("blocked-evidence-after-901", "1")] {
        require(&process, key, expected, errors);
    }
    match (fs::read(bundle.join("sample-before.md")), fs::read(bundle.join("sample-after.md"))) {
        (Ok(before), Ok(after)) if before == after => {}, _ => errors.push("target bytes changed or are unreadable".into()),
    }
    let hashes = read_table(&bundle.join("workspace-hashes.tsv"), errors);
    let before = hashes.get("target-before").and_then(|row| row.get(1));
    let after = hashes.get("target-after").and_then(|row| row.get(1));
    if before.is_none() || before != after { errors.push("target hashes changed".into()); }
    let counts = read_table(&bundle.join("sqlite-counts.tsv"), errors);
    for table in ["attempts", "check_results", "token_usage", "effect_journal", "runtime_decisions", "prompt_frames",
        "prompt_cards", "tool_admissions", "observations", "provider_exchanges", "artifacts", "workspace_operations",
        "workspace_operation_revisions"] {
        if counts.get(table).and_then(|row| row.first()).map(String::as_str) != Some("0") { errors.push(format!("baseline count is not zero: {table}")); }
    }
}
#[rustfmt::skip]
fn git_source(root: &Path, source: &str, errors: &mut Vec<String>) {
    if !git_ok(root, &["cat-file", "-e", &format!("{source}^{{commit}}")]) { errors.push("baseline source commit is unavailable".into()); }
    if !git_ok(root, &["merge-base", "--is-ancestor", source, "HEAD"]) { errors.push("baseline source is not reachable from HEAD".into()); }
    let path = root.join("evaluation/evidence").join(source).join("baseline/source.txt");
    if fs::read_to_string(path).ok().map(|value| value.trim().to_string()).as_deref() != Some(source) { errors.push("baseline source.txt differs".into()); }
}
#[rustfmt::skip]
fn manifest_rows(path: &Path, errors: &mut Vec<String>) -> BTreeMap<String, String> {
    let mut rows = BTreeMap::new();
    let text = match fs::read_to_string(path) { Ok(value) => value, Err(error) => { errors.push(error.to_string()); return rows; } };
    for line in text.lines() {
        let Some((digest, name)) = line.split_once("  ") else { errors.push("malformed baseline manifest row".into()); continue; };
        if digest.len() != 64 || name.contains('/') || rows.insert(name.into(), digest.into()).is_some() { errors.push("invalid or duplicate baseline manifest row".into()); }
    } rows
}
#[rustfmt::skip]
fn bundle_files(path: &Path, errors: &mut Vec<String>) -> BTreeSet<String> {
    match fs::read_dir(path) {
        Ok(entries) => entries.filter_map(Result::ok).filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_symlink() || !entry.path().is_file() { errors.push(format!("invalid baseline entry: {name}")); }
            (name != "manifest.sha256").then_some(name)
        }).collect(),
        Err(error) => { errors.push(error.to_string()); BTreeSet::new() }
    }
}
#[rustfmt::skip]
fn pairs(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    Ok(text.lines().filter_map(|line| line.split_once('\t')).map(|(a,b)| (a.into(),b.into())).collect())
}
#[rustfmt::skip]
fn read_pairs(path: &Path, errors: &mut Vec<String>) -> BTreeMap<String, String> { pairs(path).unwrap_or_else(|e| { errors.push(e); BTreeMap::new() }) }
#[rustfmt::skip]
fn read_table(path: &Path, errors: &mut Vec<String>) -> BTreeMap<String, Vec<String>> {
    let text = match fs::read_to_string(path) { Ok(v) => v, Err(e) => { errors.push(e.to_string()); return BTreeMap::new(); } };
    text.lines().skip(1).filter_map(|line| { let mut row = line.split('\t').map(str::to_string).collect::<Vec<_>>();
        if row.is_empty() { None } else { let name = row.remove(0); Some((name, row)) } }).collect()
}
#[rustfmt::skip]
fn require(values: &BTreeMap<String,String>, key: &str, expected: &str, errors: &mut Vec<String>) {
    if values.get(key).map(String::as_str) != Some(expected) { errors.push(format!("baseline requires {key}={expected}")); }
}
#[rustfmt::skip]
fn git_ok(root: &Path, args: &[&str]) -> bool { Command::new("git").args(args).current_dir(root).output().is_ok_and(|out| out.status.success()) }
#[rustfmt::skip]
fn full_commit(value: &str) -> bool { value.len() == 40 && value.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()) }
#[rustfmt::skip]
fn secret(bytes: &[u8]) -> bool {
    let lower = bytes.iter().map(u8::to_ascii_lowercase).collect::<Vec<_>>();
    let private_key = [b"-----begin ".as_slice(), b"private key-----"].concat();
    [b"sk-".as_slice(), b"ghp_", b"github_pat_"].iter().any(|p| lower.windows(p.len()).any(|w| w == *p))
        || lower.windows(private_key.len()).any(|w| w == private_key)
        || lower.windows(21).any(|w| w.starts_with(b"authorization: bearer"))
}
