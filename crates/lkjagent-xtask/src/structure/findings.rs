#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureFinding {
    pub path: String,
    pub rule: String,
    pub fix: String,
}

impl StructureFinding {
    pub fn new(path: impl Into<String>, rule: impl Into<String>, fix: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            rule: rule.into(),
            fix: fix.into(),
        }
    }

    pub fn message(&self) -> String {
        format!("{}: {}: {}", self.path, self.rule, self.fix)
    }
}
