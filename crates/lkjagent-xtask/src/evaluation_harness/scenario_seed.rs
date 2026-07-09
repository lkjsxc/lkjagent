use std::fs;
use std::path::{Component, Path};

use super::hash;

pub(super) fn check(path: &Path, id: &str, failures: &mut Vec<String>) {
    let manifest = path.join("seed-manifest.tsv");
    let text = fs::read_to_string(&manifest).unwrap_or_else(|error| {
        failures.push(format!("could not read {}: {error}", manifest.display()));
        String::new()
    });
    let mut rows = 0;
    for row in text.lines().skip(1) {
        rows += 1;
        let fields = row.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 || !safe_relative(fields.first().copied().unwrap_or("")) {
            failures.push(format!("scenario {id} seed row is malformed"));
            continue;
        }
        let seed = path.join("seed").join(fields[0]);
        let bytes = fs::read(&seed).unwrap_or_default();
        if bytes.is_empty() || hash::bytes(&bytes) != fields[3] || seed.is_symlink() {
            failures.push(format!("scenario {id} seed differs: {}", fields[0]));
        }
    }
    if rows < 2 {
        failures.push(format!("scenario {id} has fewer than two seed files"));
    }
}

fn safe_relative(value: &str) -> bool {
    !value.is_empty()
        && Path::new(value)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}
