pub(super) fn join_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .take(16)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    }
}
