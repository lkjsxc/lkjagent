#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveState {
    pub raw_owner_message: String,
    pub normalized: String,
    pub version: u32,
    pub non_goals: Vec<String>,
    pub owner_constraints: Vec<String>,
}

impl ObjectiveState {
    pub fn new(raw: &str) -> Self {
        Self {
            raw_owner_message: raw.to_string(),
            normalized: normalize_objective(raw),
            version: 1,
            non_goals: extract_lines(raw, &["do not", "no ", "out of scope"]),
            owner_constraints: extract_lines(raw, &["must", "keep", "avoid", "required"]),
        }
    }
}

fn normalize_objective(raw: &str) -> String {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_lines(raw: &str, needles: &[&str]) -> Vec<String> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            needles.iter().any(|needle| lower.contains(needle))
        })
        .take(12)
        .map(str::to_string)
        .collect()
}
