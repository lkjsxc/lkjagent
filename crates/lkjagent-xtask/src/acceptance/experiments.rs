use std::collections::HashSet;

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

fn scenario(value: &str) -> bool {
    matches!(value, "S1" | "S2" | "S3" | "S4" | "S5" | "S6" | "S7" | "S8")
}
