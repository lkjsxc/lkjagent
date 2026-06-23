use std::collections::BTreeSet;

use super::model::ScaffoldInput;

pub(super) fn requested_terms(input: &ScaffoldInput) -> Vec<String> {
    let source = if input.sections.is_empty() {
        input.title.clone()
    } else {
        input.sections.join("\n")
    };
    let mut terms = Vec::new();
    for chunk in source.split([',', ';', '\n']) {
        let words = words(chunk);
        if words.len() > 4 {
            terms.extend(decomposed_terms(&words));
        } else if !words.is_empty() {
            terms.push(words.join("-"));
        }
    }
    dedup_limited(terms)
}

pub(super) fn bullet_terms(terms: &[String]) -> String {
    if terms.is_empty() {
        return "- none recorded".to_string();
    }
    terms
        .iter()
        .map(|term| format!("- `{term}`"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn title(term: &str) -> String {
    term.split('-')
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

fn decomposed_terms(words: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut index = 0;
    while index < words.len() {
        if two_word(words, index, "model", "endpoint") {
            out.push("model-endpoint".to_string());
            index += 2;
        } else if two_word(words, index, "united", "states") {
            out.push("united-states".to_string());
            index += 2;
        } else {
            out.push(words[index].clone());
            index += 1;
        }
    }
    out
}

fn two_word(words: &[String], index: usize, first: &str, second: &str) -> bool {
    index + 1 < words.len() && words[index] == first && words[index + 1] == second
}

fn words(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty() && !stopword(word))
        .map(str::to_string)
        .collect()
}

fn stopword(word: &str) -> bool {
    matches!(
        word,
        "a" | "an" | "and" | "the" | "to" | "of" | "for" | "with"
    )
}

fn dedup_limited(terms: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for term in terms {
        if seen.insert(term.clone()) {
            out.push(term);
        }
        if out.len() >= 8 {
            break;
        }
    }
    out
}

fn capitalize(part: &str) -> String {
    let mut chars = part.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}
