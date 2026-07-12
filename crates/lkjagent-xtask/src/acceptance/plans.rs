use std::collections::HashSet;
use std::path::Path;

use super::table::{self, Table};
use super::{experiments, workgraph};

const WORK: &[&str] = &[
    "id",
    "wave",
    "depends",
    "owner",
    "completion_predicate",
    "focused_command",
    "real_command",
    "required",
];
const ACCEPTANCE: &[&str] = &["id", "category", "predicate", "evidence_class", "required"];
const EXPERIMENT: &[&str] = &[
    "cell",
    "stage",
    "envelope",
    "tool_view",
    "example",
    "edit_form",
    "context",
    "observation",
    "prefix",
    "recovery",
    "verification",
    "scenarios",
    "min_repeats",
    "max_repeats",
    "escalation",
];

pub fn validate(root: &Path) -> Result<Vec<String>, Vec<String>> {
    let base = root.join("evaluation");
    let work = table::read(&base.join("workgraph.tsv"), WORK);
    let acceptance = table::read(&base.join("acceptance.tsv"), ACCEPTANCE);
    let experiment = table::read(&base.join("experiment-plan.tsv"), EXPERIMENT);
    let mut errors = Vec::new();
    match work {
        Ok(table) => errors.extend(workgraph::validate(&table)),
        Err(error) => errors.push(error),
    }
    let required = match acceptance {
        Ok(table) => {
            errors.extend(validate_acceptance(&table));
            table
                .rows
                .iter()
                .filter(|row| row[4] == "yes")
                .map(|row| row[0].clone())
                .collect()
        }
        Err(error) => {
            errors.push(error);
            Vec::new()
        }
    };
    match experiment {
        Ok(table) => errors.extend(experiments::validate(&table)),
        Err(error) => errors.push(error),
    }
    if errors.is_empty() {
        Ok(required)
    } else {
        Err(errors)
    }
}

fn validate_acceptance(table: &Table) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for row in &table.rows {
        if row.iter().any(String::is_empty) {
            errors.push(format!("acceptance plan: empty field for {}", row[0]));
        }
        if !seen.insert(row[0].clone()) {
            errors.push(format!("acceptance plan: duplicate id {}", row[0]));
        }
        if row[4] != "yes" && row[4] != "no" {
            errors.push(format!(
                "acceptance plan: invalid required flag for {}",
                row[0]
            ));
        }
    }
    errors
}
