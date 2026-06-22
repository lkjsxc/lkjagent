use super::model::ScaffoldInput;

pub fn requested_scopes(input: &ScaffoldInput) -> (bool, bool) {
    let text = format!(
        "{} {} {}",
        input.kind,
        input.title,
        input.sections.join(" ")
    )
    .to_ascii_lowercase();
    (
        implementation_requested(&text),
        domain_examples_requested(&text),
    )
}

fn implementation_requested(text: &str) -> bool {
    text.contains("rust") || text.contains("implementation")
}

fn domain_examples_requested(text: &str) -> bool {
    ["asia", "food", "minecraft", "factorio"]
        .iter()
        .any(|needle| text.contains(needle))
}
