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

pub(crate) fn action_fingerprint(action: &Action) -> String {
    let mut hash = 14_695_981_039_346_656_037_u64;
    mix(&mut hash, action.tool.as_bytes());
    for (name, value) in &action.params {
        mix(&mut hash, name.as_bytes());
        mix(&mut hash, value.as_bytes());
    }
    format!("{hash:016x}")
}

fn mix(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(1_099_511_628_211);
    }
}

fn param(action: &Action, name: &str) -> Option<String> {
    action
        .params
        .iter()
        .find(|(param, _)| param == name)
        .map(|(_, value)| value.clone())
}
