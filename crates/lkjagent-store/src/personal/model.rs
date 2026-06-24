#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalRecordInput<'a> {
    pub kind: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub status: &'a str,
    pub tags: &'a str,
    pub timezone: Option<&'a str>,
    pub start_at: Option<&'a str>,
    pub end_at: Option<&'a str>,
    pub due_at: Option<&'a str>,
    pub recurrence: Option<&'a str>,
    pub priority: Option<&'a str>,
    pub project: Option<&'a str>,
    pub source_case_id: Option<i64>,
    pub now: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalRecord {
    pub id: i64,
    pub kind: String,
    pub title: String,
    pub body: String,
    pub status: String,
    pub tags: String,
    pub timezone: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub due_at: Option<String>,
    pub recurrence: Option<String>,
    pub priority: Option<String>,
    pub project: Option<String>,
    pub source_case_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PersonalListFilter<'a> {
    pub kind: Option<&'a str>,
    pub status: Option<&'a str>,
    pub project: Option<&'a str>,
    pub start: Option<&'a str>,
    pub end: Option<&'a str>,
    pub limit: usize,
}
