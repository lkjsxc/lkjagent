mod attestation;
mod io;
mod run;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::Value;

use self::io::{field, file_hash, git, git_ok, pairs, source_clean, source_hash, table, text};

const CAMPAIGN: &str = "tmp/lkjagent-progress/nodes/domain-experiments/campaign";
#[rustfmt::skip]
pub(super) const SCENARIOS: &[&str] = &["daily-life-recall", "multi-project-development", "long-artifact-recovery"];

#[derive(Clone)]
#[rustfmt::skip]
pub(super) struct Cell { pub id: String, pub kind: String, pub factors: BTreeMap<String, Value> }

#[rustfmt::skip]
pub fn check(root: &Path) -> Result<(), Vec<String>> { validate(root).map_err(|error| vec![error]) }

#[rustfmt::skip]
fn validate(root: &Path) -> Result<(), String> {
    let mut campaign = root.to_path_buf();
    for item in CAMPAIGN.split('/') { campaign.push(item);
        if campaign.is_symlink() { return Err("campaign path is a symlink".into()); } }
    let threshold = file_hash(&root.join(
        "tmp/lkjagent-evidence-first-rebuild-20260710/09-evaluation/adoption-thresholds.md",
    ))?;
    let threshold = threshold.strip_prefix("sha256:").ok_or("threshold hash malformed")?;
    let plan = plan(root, threshold)?;
    let matrix = table(&campaign.join("experiment-matrix.tsv"))?;
    let declared = matrix.iter().map(|row| field(row, "run_ref").to_string()).collect::<BTreeSet<_>>();
    let mut active = BTreeSet::new();
    for item in fs::read_dir(campaign.join("runs")).map_err(io::err)? { let path = item.map_err(io::err)?.path();
        if path.is_symlink() || !path.is_dir() { return Err("active run path is not a directory".into()); }
        active.insert(format!("runs/{}", path.file_name().and_then(|name| name.to_str()).ok_or("run name is not UTF-8")?)); }
    if active != declared { return Err("active run directories differ from matrix".into()); }
    let source = text(&campaign.join("source-commit.txt"))?; let plan_commit = text(&campaign.join("plan-commit.txt"))?;
    let valid_commit = |value: &str| value.len()==40 && value.chars().all(|item| item.is_ascii_hexdigit());
    if !valid_commit(&source) || !valid_commit(&plan_commit) || !source_clean(root)
        || text(&campaign.join("source-tree-sha256.txt"))? != source_hash(root)? { return Err("campaign source bytes differ".into()); }
    if root.join(".git").exists() { let introduced = git(root, &["log", "--format=%H", "--diff-filter=A", "--", "evaluation/experiment-plan.tsv"])?;
        if source != git(root, &["rev-parse", "HEAD"])? || plan_commit != introduced.lines().next().unwrap_or("") || plan_commit == source
            || !git_ok(root, &["merge-base", "--is-ancestor", &plan_commit, &source])
            || !git_ok(root, &["diff", "--quiet", &plan_commit, &source, "--", "evaluation/experiment-plan.tsv"]) {
            return Err("experiment plan is not a strict candidate ancestor".into()); }
    } else if plan_commit == source { return Err("experiment plan is not a strict candidate ancestor".into()); }
    let executable = campaign.join("lkjagent");
    if executable.is_symlink() { return Err("campaign executable is a symlink".into()); }
    let executable_hash = file_hash(&executable)?;
    let rebuilt_hash = attestation::rebuild_hash(root)?;
    if root.join(".git").exists() && rebuilt_hash != executable_hash {
        return Err("candidate executable differs from independent build".into()); }
    let build = pairs(&campaign.join("build.tsv"))?;
    if field(&build, "source_commit") != source
        || field(&build, "source_tree_sha256") != source_hash(root)?
        || field(&build, "cargo_lock_sha256") != file_hash(&root.join("Cargo.lock"))?
        || field(&build, "executable_sha256") != executable_hash
        || field(&build, "build_log_sha256") != file_hash(&campaign.join("build.log"))?
        || field(&build, "build_mode") != "detached-offline-release-remapped"
        || field(&build, "build_exit") != "0"
    {
        return Err("candidate build provenance mismatch".into());
    }
    let baseline = io::json(&root.join("data/lkjagent.json"))?;
    let mut failures = Vec::new();
    let mut run_ids = BTreeSet::new();
    let mut stores = BTreeSet::new();
    let mut provider_controls = BTreeSet::new();
    let mut used_rows = 0;
    for cell in &plan {
        for scenario in SCENARIOS {
            let mut matches = matrix
                .iter()
                .filter(|row| {
                    field(row, "cell_id") == cell.id && field(row, "scenario_id") == *scenario
                })
                .collect::<Vec<_>>();
            matches.sort_by_key(|row| field(row, "repeat").parse::<u32>().unwrap_or(0));
            let first_outcomes = matches
                .iter()
                .take(3)
                .map(|row| field(row, "outcome_fingerprint"))
                .collect::<BTreeSet<_>>();
            let expected = if first_outcomes.len() > 1 { 5 } else { 3 };
            if matches.len() != expected
                || matches.iter().enumerate().any(|(index, row)| {
                    field(row, "repeat").parse::<usize>().ok() != Some(index + 1)
                })
            {
                failures.push(format!("{} {scenario} repeat coverage invalid", cell.id));
                continue;
            }
            used_rows += matches.len();
            for row in matches {
                if let Err(error) = run::validate(
                    root,
                    &campaign,
                    row,
                    cell,
                    &baseline,
                    &source,
                    &executable_hash,
                    scenario,
                    &mut run_ids,
                    &mut stores,
                    &mut provider_controls,
                ) {
                    failures.push(error);
                }
            }
        }
    }
    if matrix.len() != used_rows {
        failures.push("matrix contains undeclared rows".into());
    }
    if provider_controls.len() != 1 {
        failures.push("provider controls changed across cells".into());
    }
    validate_kinds(&plan, &mut failures);
    adoption(&campaign, &plan, &mut failures)?;
    if let Err(error) = io::campaign_manifest(&campaign) {
        failures.push(error);
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join(" | "))
    }
}

#[rustfmt::skip]
fn plan(root: &Path, threshold: &str) -> Result<Vec<Cell>, String> {
    let rows = table(&root.join("evaluation/experiment-plan.tsv"))?;
    let mut cells = Vec::new();
    let mut ids = BTreeSet::new();
    for row in rows {
        if field(&row, "threshold_sha256") != threshold
            || field(&row, "threshold_ref")
                != "tmp/lkjagent-evidence-first-rebuild-20260710/09-evaluation/adoption-thresholds.md"
            || field(&row, "scenario_ids") != SCENARIOS.join("|")
            || field(&row, "repeat_policy") != "3;5-if-first-three-differ"
            || !ids.insert(field(&row, "cell_id").to_string())
        {
            return Err("experiment plan declaration mismatch".into());
        }
        let value: Value =
            serde_json::from_str(field(&row, "factor_config_json")).map_err(io::err)?;
        let factors = value
            .as_object()
            .ok_or("factor config is not an object")?
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        cells.push(Cell { id: field(&row, "cell_id").to_string(),
            kind: field(&row, "cell_kind").to_string(), factors });
    }
    Ok(cells)
}

#[rustfmt::skip]
fn validate_kinds(cells: &[Cell], failures: &mut Vec<String>) {
    for (kind, minimum) in [("baseline", 1), ("isolated", 3), ("pairwise", 1), ("integrated", 3)] {
        if cells.iter().filter(|cell| cell.kind == kind).count() < minimum { failures.push(format!("too few {kind} cells")); }
    }
}

#[rustfmt::skip]
fn adoption(root: &Path, cells: &[Cell], failures: &mut Vec<String>) -> Result<(), String> {
    let rows = table(&root.join("adoption.tsv"))?; let matrix = table(&root.join("experiment-matrix.tsv"))?;
    if rows.len() != cells.len() { failures.push("adoption contains undeclared rows".into()); }
    for cell in cells {
        let matches = rows.iter().filter(|row| field(row, "cell_id") == cell.id).collect::<Vec<_>>();
        let outcomes=matrix.iter().filter(|row|field(row,"cell_id")==cell.id).map(|row|field(row,"outcome")).collect::<BTreeSet<_>>();
        let config_rejected=outcomes.contains("probe-config-rejected"); let no_exchange=outcomes.contains("probe-no-exchange");
        let rejected=config_rejected || no_exchange || outcomes.iter().any(|outcome|matches!(*outcome,"probe-parse-fault"|"probe-admission-rejected"));
        let expected=if config_rejected { ("rejected","configuration-rejected") }
            else if no_exchange { ("rejected","no-provider-exchange") } else if rejected { ("rejected","probe-protocol-failure") }
            else { ("conditional","requires-fault-and-frozen-live-campaign") };
        if matches.len() != 1 || field(matches[0], "decision") != expected.0
            || field(matches[0], "rationale") != expected.1 { failures.push(format!("{} adoption missing", cell.id)); }
    }
    Ok(())
}
