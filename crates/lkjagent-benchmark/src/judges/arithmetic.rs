use std::fs;
use std::path::Path;

pub fn judge(workspace: &Path) -> Result<(), String> {
    let text = fs::read_to_string(workspace.join("answer.txt"))
        .map_err(|error| format!("answer.txt missing or unreadable: {error}"))?;
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.len() != 1 {
        return Err("answer.txt must contain exactly one integer".to_string());
    }
    let value = tokens[0]
        .parse::<i64>()
        .map_err(|error| format!("answer is not an integer: {error}"))?;
    if value != 346 {
        return Err(format!(
            "expected smallest nonnegative CRT solution 346, got {value}"
        ));
    }
    Ok(())
}
