pub(super) fn oversize_error(preview: &str) -> String {
    if preview.is_empty() {
        "endpoint completion hit max tokens".to_string()
    } else {
        format!("endpoint completion hit max tokens\npreview={preview}")
    }
}

pub(super) fn oversize_recovery(preview: &str) -> String {
    if preview.contains("<tool>fs.write</tool>") || preview.contains("<content>") {
        return "recovery: completion hit max tokens inside a write payload; same-shape retry is blocked; use artifact.next, then one-file fs.batch_write or artifact.audit".to_string();
    }
    if preview.contains("<action>") {
        return "recovery: completion hit max tokens after starting an action; next act must stay bounded; use artifact.next or one-file fs.batch_write".to_string();
    }
    "recovery: completion hit max tokens; next act must stay bounded; prefer artifact.next, audit, or a one-file write".to_string()
}
