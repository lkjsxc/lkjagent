use std::fs;
use std::path::Path;

const EDGES: &[(&str, &str, u32)] = &[
    ("A", "B", 2),
    ("A", "C", 5),
    ("B", "C", 1),
    ("B", "D", 4),
    ("C", "D", 1),
    ("C", "E", 7),
    ("D", "E", 3),
    ("D", "F", 6),
    ("E", "F", 1),
];

pub fn judge(workspace: &Path) -> Result<(), String> {
    let text = fs::read_to_string(workspace.join("path.txt"))
        .map_err(|error| format!("path.txt missing or unreadable: {error}"))?;
    let path: Vec<&str> = text.split_whitespace().collect();
    if path.first().copied() != Some("A") || path.last().copied() != Some("F") {
        return Err("path must start at A and end at F".to_string());
    }
    let mut cost = 0_u32;
    for pair in path.windows(2) {
        let from = pair.first().copied().unwrap_or("");
        let to = pair.get(1).copied().unwrap_or("");
        let edge = edge_cost(from, to).ok_or_else(|| format!("missing edge {from}->{to}"))?;
        cost = cost.saturating_add(edge);
    }
    if cost == 8 {
        Ok(())
    } else {
        Err(format!("path cost {cost}, optimum is 8"))
    }
}

fn edge_cost(from: &str, to: &str) -> Option<u32> {
    EDGES
        .iter()
        .find(|(left, right, _)| *left == from && *right == to)
        .map(|(_, _, cost)| *cost)
}
