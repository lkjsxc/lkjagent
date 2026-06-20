pub(super) fn oversize_error(preview: &str) -> String {
    if preview.is_empty() {
        "endpoint completion hit max tokens".to_string()
    } else {
        format!("endpoint completion hit max tokens\npreview={preview}")
    }
}

pub(super) fn oversize_recovery(preview: &str) -> String {
    if preview.contains("<tool>fs.write</tool>") || preview.contains("<content>") {
        return "recovery: completion hit max tokens inside a write payload; raw fs.write retry is blocked while payload risk is active; use doc.scaffold, fs.batch_write, or a smaller section write".to_string();
    }
    if preview.contains("<act>") {
        return "recovery: completion hit max tokens after starting an action; next act must stay bounded; use fs.batch_write or doc.scaffold for large file payloads".to_string();
    }
    "recovery: completion hit max tokens; next act must stay bounded; prefer typed fs/doc tools and narrow observations".to_string()
}
