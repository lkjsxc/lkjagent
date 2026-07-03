#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueRow {
    pub id: i64,
    pub content: String,
    pub state: String,
    pub task_id: Option<i64>,
    pub force_new: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredEvent {
    pub id: i64,
    pub task_id: Option<i64>,
    pub kind: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanExchange {
    pub path: String,
}
