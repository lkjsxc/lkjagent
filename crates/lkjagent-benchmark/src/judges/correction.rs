use std::fs;
use std::path::Path;

const EXPECTED: &[i64] = &[21, 22, 24, 25, 26, 27, 28];

pub fn judge(workspace: &Path) -> Result<(), String> {
    let text = fs::read_to_string(workspace.join("answer.txt"))
        .map_err(|error| format!("answer.txt missing or unreadable: {error}"))?;
    let values = parse_values(&text)?;
    if values == EXPECTED {
        Ok(())
    } else {
        Err(format!("expected {EXPECTED:?}, got {values:?}"))
    }
}

fn parse_values(text: &str) -> Result<Vec<i64>, String> {
    let mut values = Vec::new();
    for raw in text.trim().split(',') {
        let value = raw
            .trim()
            .parse::<i64>()
            .map_err(|error| format!("bad integer '{raw}': {error}"))?;
        values.push(value);
    }
    Ok(values)
}
