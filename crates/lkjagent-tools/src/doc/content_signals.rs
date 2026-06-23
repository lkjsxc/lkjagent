pub(super) fn marker_failure(relative: &str, text: &str) -> Option<String> {
    if scaffold_only(text) {
        return Some(format!("scaffold_only_content: {relative}"));
    }
    if let Some(state) = explicit_weak_state(text) {
        return Some(format!("{state}: {relative}"));
    }
    if let Some(phrase) = generated_banned_phrase(text) {
        return Some(format!(
            "generated_boilerplate_content: {relative} phrase={phrase}"
        ));
    }
    if let Some(phrase) = crate::placeholder::detect(text) {
        return Some(format!("placeholder_content: {relative} phrase={phrase}"));
    }
    None
}

fn scaffold_only(text: &str) -> bool {
    text.contains("This file records the")
        && text.contains("generated documentation tree")
        && text.contains("\nscaffolded\n")
}

fn explicit_weak_state(text: &str) -> Option<&'static str> {
    if text.contains("content_state=structure-only") {
        return Some("structure_only_content");
    }
    if text.contains("content_state=owner-term-only") {
        return Some("owner_term_only_content");
    }
    None
}

fn generated_banned_phrase(text: &str) -> Option<&'static str> {
    let normalized_text = normalized(text);
    [
        "keep this file semantic and linked from its local readme",
        "record concrete facts decisions and verification evidence",
        "implementation hooks",
        "failure modes",
        "defines the artifact role the observed constraints",
        "example one names a path an invariant",
    ]
    .into_iter()
    .find(|phrase| normalized_text.contains(&normalized(phrase)))
}

fn normalized(text: &str) -> String {
    text.chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}
