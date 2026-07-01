use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CollectOptions {
    pub data_dir: PathBuf,
    pub out_dir: PathBuf,
}

#[derive(Debug, Default)]
pub struct ProofBundle {
    pub store_path: String,
    pub store_present: bool,
    pub cases: Vec<CaseRow>,
    pub queue_counts: Vec<CountRow>,
    pub queue_recent: Vec<QueueRow>,
    pub readiness: Vec<ReadinessRow>,
    pub active_contracts: Vec<ContractRow>,
    pub decisions: Vec<DecisionRow>,
    pub transcript: Vec<EventRow>,
    pub model_logs: Vec<FileRow>,
    pub workspace_files: Vec<FileRow>,
    pub word_counts: Vec<WordCountRow>,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub struct CaseRow {
    pub id: i64,
    pub family: String,
    pub phase: String,
    pub node: String,
    pub status: String,
    pub next_action: String,
    pub objective_chars: i64,
}

#[derive(Debug)]
pub struct CountRow {
    pub name: String,
    pub count: i64,
}

#[derive(Debug)]
pub struct QueueRow {
    pub id: i64,
    pub status: String,
    pub delivered_turn: String,
    pub content_chars: i64,
    pub created_at: String,
}

#[derive(Debug)]
pub struct ReadinessRow {
    pub plan_id: i64,
    pub root: String,
    pub profile: String,
    pub status: String,
    pub atoms: String,
    pub measured: i64,
    pub floor: i64,
    pub active_contract: String,
    pub next_path: String,
    pub blockers: String,
}

#[derive(Debug)]
pub struct ContractRow {
    pub id: String,
    pub root: String,
    pub status: String,
    pub limits: String,
    pub paths: String,
}

#[derive(Debug)]
pub struct DecisionRow {
    pub id: i64,
    pub mission: String,
    pub mode: String,
    pub node: String,
    pub next_action: String,
    pub completion: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct EventRow {
    pub id: i64,
    pub turn: String,
    pub kind: String,
    pub tokens: i64,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct FileRow {
    pub path: String,
    pub bytes: u64,
}

#[derive(Debug)]
pub struct WordCountRow {
    pub root: String,
    pub files: usize,
    pub words: usize,
    pub manuscript_files: usize,
    pub manuscript_words: usize,
}
