use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn read(path: &Path, expected: &[&str]) -> Result<Table, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("{}: cannot read table: {error}", display(path)))?;
    parse(&text, expected, &display(path))
}

pub fn parse(text: &str, expected: &[&str], label: &str) -> Result<Table, String> {
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| format!("{label}: empty table"))?;
    let headers = fields(header);
    let wanted = expected
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    if headers != wanted {
        return Err(format!("{label}: unexpected TSV header"));
    }
    let mut rows = Vec::new();
    for (offset, line) in lines.enumerate() {
        if line.is_empty() {
            return Err(format!("{label}: blank row at line {}", offset + 2));
        }
        let row = fields(line);
        if row.len() != headers.len() {
            return Err(format!("{label}: wrong field count at line {}", offset + 2));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err(format!("{label}: table has no rows"));
    }
    Ok(Table { headers, rows })
}

fn fields(line: &str) -> Vec<String> {
    line.trim_end_matches('\r')
        .split('\t')
        .map(str::to_string)
        .collect()
}

fn display(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
