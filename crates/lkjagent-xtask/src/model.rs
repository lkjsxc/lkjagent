#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoFile {
    pub path: String,
    pub text: String,
}

impl RepoFile {
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
        }
    }

    pub fn line_count(&self) -> usize {
        self.text.lines().count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub path: String,
    pub rule: String,
    pub fix: String,
}

impl Violation {
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
