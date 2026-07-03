use crate::parse::Action;

pub(crate) fn finish_summary(action: &Action) -> Option<String> {
    if action.tool == "finish" {
        Some(param(action, "summary").unwrap_or_else(|| "explore finished".to_string()))
    } else {
        None
    }
}

pub(crate) fn memory_save(action: &Action) -> Option<(String, String)> {
    if action.tool == "memory.save" {
        Some((param(action, "topic")?, param(action, "content")?))
    } else {
        None
    }
}

fn param(action: &Action, name: &str) -> Option<String> {
    action
        .params
        .iter()
        .find(|(param, _)| param == name)
        .map(|(_, value)| value.clone())
}
