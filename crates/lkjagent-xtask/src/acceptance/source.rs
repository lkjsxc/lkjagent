use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

#[path = "source_facts.rs"]
mod source_facts;
use source_facts::FACTS;

pub fn contract_files() -> Vec<&'static str> {
    FACTS
        .iter()
        .map(|fact| fact.path)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn contract_derivations(root: &Path) -> BTreeSet<String> {
    let mut complete = BTreeMap::<&str, bool>::new();
    for fact in FACTS {
        let text = fs::read_to_string(root.join(fact.path)).ok();
        let present = text
            .as_ref()
            .is_some_and(|value| fact.needles.iter().all(|needle| value.contains(needle)));
        complete
            .entry(fact.id)
            .and_modify(|value| *value &= present)
            .or_insert(present);
    }
    let mut derived = complete
        .into_iter()
        .filter(|(_, complete)| *complete)
        .map(|(id, _)| id.to_string())
        .collect::<BTreeSet<_>>();
    if retired_authority_absent(root) {
        derived.insert("S02".into());
    }
    derived
}

#[rustfmt::skip]
fn retired_authority_absent(root:&Path)->bool{
 const PATHS:&[&str]=&["crates/lkjagent-core/src/owner_turn.rs","crates/lkjagent-core/src/plan.rs","crates/lkjagent-core/src/templates.rs","crates/lkjagent-core/src/artifact_manifest.rs","crates/lkjagent-core/src/runtime_artifact.rs","crates/lkjagent-core/src/workspace_manifest.rs","crates/lkjagent-core/src/workspace_record.rs","crates/lkjagent-app/src/daemon_route_effects.rs","crates/lkjagent-app/src/runtime_bridge.rs","crates/lkjagent-app/src/workspace_search/inventory.rs","crates/lkjagent-store/src/plan_schema.rs"];
 if PATHS.iter().any(|path|root.join(path).exists()){return false}
 const TOKENS:&[&str]=&["CREATE TABLE tasks","CREATE TABLE steps","Command::Workbench","Command::Record","pub mod plan_schema","pub mod runtime_bridge"];
 ["crates/lkjagent-core/src","crates/lkjagent-app/src","crates/lkjagent-store/src"].iter().flat_map(|dir|fs::read_dir(root.join(dir)).into_iter().flatten().flatten()).filter(|row|row.path().extension().and_then(|x|x.to_str())==Some("rs")).all(|row|fs::read_to_string(row.path()).is_ok_and(|text|TOKENS.iter().all(|token|!text.contains(token))))
}

pub fn validate(root: &Path, source: &str) -> Result<(), String> {
    validate_shape(source)?;
    let revision = format!("{source}^{{commit}}");
    let resolved = git(root, &["rev-parse", "--verify", &revision])?;
    if !resolved.status.success() || String::from_utf8_lossy(&resolved.stdout).trim() != source {
        return Err("source is not an exact reachable commit".to_string());
    }
    let ancestor = git(root, &["merge-base", "--is-ancestor", source, "HEAD"])?;
    if !ancestor.status.success() {
        return Err("source is not an ancestor of Git HEAD".to_string());
    }
    validate_later_paths(root, source)
}

fn validate_shape(source: &str) -> Result<(), String> {
    if source.len() != 40
        || !source
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("source must be a full lowercase Git commit ID".to_string());
    }
    Ok(())
}

fn validate_later_paths(root: &Path, source: &str) -> Result<(), String> {
    let range = format!("{source}..HEAD");
    let output = git(root, &["diff", "--name-only", "-z", &range, "--"])?;
    if !output.status.success() {
        return Err("cannot compare source with Git HEAD".to_string());
    }
    let allowed = format!("evaluation/evidence/{source}/");
    let changed = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path))
        .find(|path| !path.starts_with(&allowed));
    match changed {
        Some(path) => Err(format!(
            "Git HEAD changed outside source evidence after freeze: {path}"
        )),
        None => Ok(()),
    }
}

fn git(root: &Path, args: &[&str]) -> Result<Output, String> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))
}
