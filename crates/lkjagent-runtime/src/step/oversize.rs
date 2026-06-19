pub(super) fn oversize_error(preview: &str) -> String {
    if preview.is_empty() {
        "endpoint completion hit max tokens".to_string()
    } else {
        format!("endpoint completion hit max tokens\npreview={preview}")
    }
}

pub(super) fn oversize_recovery(preview: &str) -> String {
    if preview.contains("<act>") {
        return "recovery: completion hit max tokens after starting an action; next act must stay under about 1200 chars; shell.run starts in workspace, so do not cd /workspace; use direct /bin/sh loops with printf templates; no brace expansion, cat heredocs, bash scripts, or literal file bodies".to_string();
    }
    "recovery: completion hit max tokens; next act must stay under about 1200 chars; use direct /bin/sh loops with printf templates for large generated output; do not cd /workspace".to_string()
}
