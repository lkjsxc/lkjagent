use std::collections::{HashMap, HashSet};

use super::table::Table;

pub fn validate(table: &Table) -> Vec<String> {
    let mut errors = simple_errors(table);
    let rows = table
        .rows
        .iter()
        .map(|row| (row[0].as_str(), row))
        .collect::<HashMap<_, _>>();
    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for row in &table.rows {
        let deps = if row[2] == "none" {
            Vec::new()
        } else {
            row[2].split(',').collect()
        };
        let mut seen = HashSet::new();
        for dep in &deps {
            if !seen.insert(*dep) {
                errors.push(format!(
                    "workgraph: {} has duplicate dependency {dep}",
                    row[0]
                ));
            }
            if !rows.contains_key(dep) {
                errors.push(format!(
                    "workgraph: {} has unknown dependency {dep}",
                    row[0]
                ));
            }
            if *dep == row[0] {
                errors.push(format!("workgraph: {} depends on itself", row[0]));
            }
        }
        graph.insert(&row[0], deps);
    }
    let ancestry = final_ancestry(&graph, &mut errors);
    for row in &table.rows {
        if row[7] == "yes" && !ancestry.contains(row[0].as_str()) {
            errors.push(format!(
                "workgraph: required node {} is outside final ancestry",
                row[0]
            ));
        }
    }
    cycle_errors(&graph, &mut errors);
    errors
}

fn simple_errors(table: &Table) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut errors = Vec::new();
    for row in &table.rows {
        if row.iter().any(String::is_empty) {
            errors.push(format!("workgraph: empty field for {}", row[0]));
        }
        if !seen.insert(row[0].clone()) {
            errors.push(format!("workgraph: duplicate id {}", row[0]));
        }
        if row[7] != "yes" && row[7] != "no" {
            errors.push(format!("workgraph: invalid required flag for {}", row[0]));
        }
    }
    errors
}

fn final_ancestry<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    errors: &mut Vec<String>,
) -> HashSet<&'a str> {
    let mut done = HashSet::new();
    if !graph.contains_key("final-acceptance") {
        errors.push("workgraph: missing final node final-acceptance".to_string());
        return done;
    }
    gather("final-acceptance", graph, &mut done);
    done
}

fn gather<'a>(id: &'a str, graph: &HashMap<&'a str, Vec<&'a str>>, done: &mut HashSet<&'a str>) {
    if !done.insert(id) {
        return;
    }
    if let Some(deps) = graph.get(id) {
        for dep in deps.iter().filter(|dep| graph.contains_key(**dep)) {
            gather(dep, graph, done);
        }
    }
}

fn cycle_errors<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, errors: &mut Vec<String>) {
    let mut done = HashSet::new();
    let mut active = HashSet::new();
    for id in graph.keys() {
        detect(id, graph, &mut done, &mut active, errors);
    }
}

fn detect<'a>(
    id: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    done: &mut HashSet<&'a str>,
    active: &mut HashSet<&'a str>,
    errors: &mut Vec<String>,
) {
    if done.contains(id) {
        return;
    }
    if !active.insert(id) {
        errors.push(format!("workgraph: dependency cycle at {id}"));
        return;
    }
    if let Some(deps) = graph.get(id) {
        for dep in deps.iter().filter(|dep| graph.contains_key(**dep)) {
            detect(dep, graph, done, active, errors);
        }
    }
    active.remove(id);
    done.insert(id);
}
