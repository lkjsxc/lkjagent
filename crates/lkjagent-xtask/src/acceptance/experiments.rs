use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::Path;
use std::process::Command;

use super::table::Table;

pub fn validate(table: &Table) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for row in &table.rows {
        if !seen.insert(row[0].clone()) {
            errors.push(format!("experiment plan: duplicate cell {}", row[0]));
        }
        errors.extend(concrete_errors(table, row));
        let min = row[12].parse::<u32>();
        let max = row[13].parse::<u32>();
        if !matches!((&min, &max), (Ok(low), Ok(high)) if *low > 0 && low <= high) {
            errors.push(format!(
                "experiment plan: invalid repeat bounds for cell {}",
                row[0]
            ));
        }
        if row[11].split(',').any(|item| !scenario(item)) {
            errors.push(format!(
                "experiment plan: unknown scenario for cell {}",
                row[0]
            ));
        }
    }
    errors
}

fn concrete_errors(table: &Table, row: &[String]) -> Vec<String> {
    row.iter()
        .enumerate()
        .filter_map(|(index, value)| {
            let lower = value.to_ascii_lowercase();
            (value.is_empty()
                || matches!(
                    lower.as_str(),
                    "tbd" | "todo" | "placeholder" | "unknown" | "n/a" | "?"
                ))
            .then(|| {
                format!(
                    "experiment plan: nonconcrete {} for cell {}",
                    table.headers[index], row[0]
                )
            })
        })
        .collect()
}

pub fn derivations(root: &Path, bytes: &[u8], source: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let Ok(text) = std::str::from_utf8(bytes) else {
        return out;
    };
    let mut lines = text.lines();
    if lines.next() != Some("cell\tscenario\trepeat\texperiment_source_commit\tplan_commit\tprofile_sha256\toutcome\tresponse_sha256\tendpoint_call_count") { return out }
    let Ok(plan) = std::fs::read_to_string(root.join("evaluation/experiment-plan.tsv")) else {
        return out;
    };
    let mut plan_lines = plan.lines();
    let headers = plan_lines
        .next()
        .unwrap_or_default()
        .split('\t')
        .collect::<Vec<_>>();
    let cells = plan_lines
        .filter_map(|line| {
            let values = line.split('\t').collect::<Vec<_>>();
            (values.len() == headers.len()).then(|| {
                (
                    values[0],
                    (
                        values[11].split(',').collect::<BTreeSet<_>>(),
                        hard(&headers, &values),
                    ),
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut winner = true;
    let mut experiment_source = None;
    let mut plan_commit = None;
    for line in lines {
        let row = line.split('\t').collect::<Vec<_>>();
        if row.len() != 9
            || !cells.contains_key(row[0])
            || !cells[row[0]].0.contains(row[1])
            || !matches!(row[2], "1" | "2" | "3")
            || !hash(row[5])
            || !hash(row[7])
            || !matches!(row[6], "admitted" | "parse-fault" | "static-rejected")
            || !seen.insert((row[0], row[1], row[2]))
        {
            return out;
        }
        let is_hard = cells[row[0]].1;
        if (is_hard && (row[6] != "static-rejected" || row[8] != "0"))
            || (!is_hard && (row[6] == "static-rejected" || row[8] != "1"))
        {
            return out;
        }
        experiment_source.get_or_insert(row[3]);
        plan_commit.get_or_insert(row[4]);
        if experiment_source != Some(row[3]) || plan_commit != Some(row[4]) {
            return out;
        }
        if row[0] == "K" && row[6] != "admitted" {
            winner = false
        }
    }
    let expected: usize = cells
        .values()
        .map(|(scenarios, _)| scenarios.len() * 3)
        .sum();
    let (Some(run_source), Some(plan_source)) = (experiment_source, plan_commit) else {
        return out;
    };
    if seen.len() != expected
        || run_source == source
        || !ancestor(root, run_source, source)
        || !ancestor(root, plan_source, run_source)
        || changed(root, plan_source, source, "evaluation/experiment-plan.tsv")
    {
        return out;
    }
    out.insert("E03".into());
    if winner {
        out.insert("E04-candidate".into());
    }
    out
}

fn hard(headers: &[&str], values: &[&str]) -> bool {
    let row = headers
        .iter()
        .copied()
        .zip(values.iter().copied())
        .collect::<BTreeMap<_, _>>();
    row["envelope"] == "tool-named"
        || row["tool_view"] == "broad-workspace"
        || row["edit_form"] != "exact-text"
}
fn ancestor(root: &Path, older: &str, newer: &str) -> bool {
    Command::new("git")
        .args(["merge-base", "--is-ancestor", older, newer])
        .current_dir(root)
        .status()
        .is_ok_and(|s| s.success())
}
fn changed(root: &Path, old: &str, new: &str, path: &str) -> bool {
    Command::new("git")
        .args(["diff", "--quiet", old, new, "--", path])
        .current_dir(root)
        .status()
        .map_or(true, |status| !status.success())
}
fn hash(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

fn scenario(value: &str) -> bool {
    matches!(value, "S1" | "S2" | "S3" | "S4" | "S5" | "S6" | "S7" | "S8")
}
