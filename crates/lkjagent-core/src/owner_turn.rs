#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordIntent {
    pub kind: String,
    pub title: String,
    pub body: String,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RouteContext {
    pub waiting_matter: bool,
    pub open_matter: bool,
    pub force_new: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnRoute {
    pub lane: String,
    pub desired_durability: String,
    pub title_seed: String,
    pub body_seed: String,
    pub transformation_allowed: bool,
}
const CONTINUATION_WORDS: &[&str] = &["continue", "also", "same matter", "this matter", "append"];
const INSPECTION_WORDS: &[&str] = &[
    "status",
    "show",
    "list",
    "inspect",
    "current state",
    "queue",
    "matter",
];
const SYSTEM_WORDS: &[&str] = &[
    "run test",
    "cargo test",
    "cargo fmt",
    "clippy",
    "docker compose",
    "verify",
];
pub fn route_turn(text: &str, context: RouteContext) -> Option<TurnRoute> {
    let body = text.trim();
    if body.is_empty() {
        return None;
    }
    let lower = body.to_ascii_lowercase();
    if context.waiting_matter && !context.force_new {
        return Some(route("existing_matter", "queue_answer", body, false));
    }
    if let Some(intent) = record_intent_from_parts(body, &lower) {
        return Some(TurnRoute {
            lane: "record".to_string(),
            desired_durability: "workspace_record".to_string(),
            title_seed: intent.title,
            body_seed: intent.body,
            transformation_allowed: false,
        });
    }
    if artifact_request(body, &lower) {
        return Some(route("artifact_request", "runtime_decision", body, true));
    }
    if context.open_matter && !context.force_new && continuation(body, &lower) {
        return Some(route("existing_matter", "matter_update", body, true));
    }
    if inspection(body, &lower) {
        return Some(route("inspection", "read_only_report", body, false));
    }
    if system_operation(body, &lower) {
        return Some(route("system_operation", "runtime_decision", body, false));
    }
    Some(route("new_matter", "matter", body, true))
}
pub fn record_intent(text: &str) -> Option<RecordIntent> {
    let body = text.trim();
    if body.is_empty() {
        return None;
    }
    let lower = body.to_ascii_lowercase();
    record_intent_from_parts(body, &lower)
}
fn record_intent_from_parts(body: &str, lower: &str) -> Option<RecordIntent> {
    if !explicit_record(body, lower) && !typed_record(body, lower) {
        return None;
    }
    Some(RecordIntent {
        kind: record_kind(body, lower).to_string(),
        title: title(body),
        body: body.to_string(),
    })
}
fn explicit_record(text: &str, lower: &str) -> bool {
    japanese_record(text)
        || has_any(
            lower,
            &[
                "record ",
                "record that",
                "record this",
                "remember ",
                "remember that",
                "note that",
                "log that",
                "write down",
                "journal this",
                "diary entry",
                "save this note",
                "save this",
                "keep this",
            ],
        )
}
fn typed_record(text: &str, lower: &str) -> bool {
    has_any(
        lower,
        &[
            "todo",
            "to-do",
            "meeting",
            "calendar",
            "finance",
            "receipt",
            "project note",
            "artifact record",
        ],
    ) || has_any(
        text,
        &["やること", "予定", "支払", "レシート", "メモ", "日記"],
    )
}
fn japanese_record(text: &str) -> bool {
    has_any(text, &["記録", "メモ"])
        || text.contains("日記") && has_any(text, &["書", "保存", "残", "つけ"])
        || has_any(text, &["保存して", "残して", "残しといて", "覚えておいて"])
}
fn record_kind(text: &str, lower: &str) -> &'static str {
    if has_any(lower, &["todo", "to-do", "checklist"]) || text.contains("やること") {
        "todo"
    } else if has_any(lower, &["calendar", "meeting", "appointment", "schedule"])
        || has_any(text, &["予定", "会議", "予約"])
    {
        "calendar"
    } else if has_any(lower, &["finance", "receipt", "invoice", "paid", "budget"])
        || text.contains('円')
        || has_any(text, &["支払", "レシート", "家計"])
    {
        "finance"
    } else if has_any(lower, &["project", "milestone"]) || text.contains("プロジェクト") {
        "project"
    } else if has_any(lower, &["artifact", "deliverable"]) || text.contains("成果物") {
        "artifact"
    } else if has_any(lower, &["note", "memo"]) || text.contains("メモ") {
        "note"
    } else {
        "journal"
    }
}
fn artifact_request(text: &str, lower: &str) -> bool {
    let object = has_any(lower, &["artifact", "deliverable", "report", "brief"])
        || has_any(text, &["成果物", "資料", "報告書"]);
    let action = has_any(lower, &["create", "make", "generate", "draft", "produce"])
        || has_any(text, &["作って", "作成", "生成"]);
    object && action
}
fn continuation(text: &str, lower: &str) -> bool {
    has_any(lower, CONTINUATION_WORDS) || has_any(text, &["この件", "それも", "続き", "追加で"])
}
fn inspection(text: &str, lower: &str) -> bool {
    has_any(lower, INSPECTION_WORDS) || has_any(text, &["状態", "見せて", "一覧", "確認"])
}
fn system_operation(text: &str, lower: &str) -> bool {
    has_any(lower, SYSTEM_WORDS) || has_any(text, &["検証", "テスト"])
}
fn route(lane: &str, desired_durability: &str, body: &str, allowed: bool) -> TurnRoute {
    TurnRoute {
        lane: lane.to_string(),
        desired_durability: desired_durability.to_string(),
        title_seed: title(body),
        body_seed: body.to_string(),
        transformation_allowed: allowed,
    }
}

fn title(text: &str) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars = compact.chars().take(48).collect::<String>();
    if chars.is_empty() {
        "record".to_string()
    } else {
        chars
    }
}

fn has_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
