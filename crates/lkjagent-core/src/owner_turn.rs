pub use crate::owner_record::{record_intent, RecordIntent};

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
#[rustfmt::skip]
const INSPECTION_WORDS: &[&str] = &["status", "show", "list", "inspect", "current state", "queue", "matter"];
#[rustfmt::skip]
const SYSTEM_WORDS: &[&str] = &["run test", "cargo test", "cargo fmt", "clippy", "docker compose", "verify"];

pub fn route_turn(text: &str, context: RouteContext) -> Option<TurnRoute> {
    let body = text.trim();
    if body.is_empty() {
        return None;
    }
    let lower = body.to_ascii_lowercase();
    if context.waiting_matter && !context.force_new {
        return Some(route("existing_matter", "queue_answer", body, false));
    }
    if crate::owner_record::ambiguous_inbox(body, &lower) {
        return Some(route("inbox", "workspace_inbox", body, false));
    }
    if let Some(intent) = crate::owner_record::record_intent_from_parts(body, &lower) {
        let allowed = !intent.body.starts_with("Verbatim\n");
        return Some(TurnRoute {
            lane: "record".to_string(),
            desired_durability: "workspace_record".to_string(),
            title_seed: intent.title,
            body_seed: intent.body,
            transformation_allowed: allowed,
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
