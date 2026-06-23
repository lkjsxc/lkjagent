pub fn encode_list(values: &[String]) -> String {
    values
        .iter()
        .map(|value| encode_value(value))
        .collect::<Vec<_>>()
        .join(",")
}

fn encode_value(value: &str) -> String {
    let mut encoded = String::new();
    for character in value.chars() {
        match character {
            '\\' => encoded.push_str("\\\\"),
            ',' => encoded.push_str("\\,"),
            '\n' => encoded.push_str("\\n"),
            other => encoded.push(other),
        }
    }
    encoded
}
