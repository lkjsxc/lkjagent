use crate::count_number::{NumberSpan, Span};
use crate::count_seed_allocation_lead::allocation_lead_before;

pub(crate) fn inferred_split_unit_spans(
    text: &str,
    numbers: &[NumberSpan],
    split_signals: &[Span],
) -> Vec<Span> {
    let mut spans = Vec::new();
    for number in numbers.iter().copied().filter(|number| number.value > 0) {
        for split in split_signals
            .iter()
            .copied()
            .filter(|split| number.span.end <= split.start)
        {
            if !segment_allowed(text, number.span, split, numbers) {
                continue;
            }
            if !rest_segment_mentions_main(text, split) {
                continue;
            }
            if let Some(unit) = inferred_unit_span(text, number.span, split) {
                spans.push(unit);
            }
        }
    }
    spans
}

fn segment_allowed(text: &str, number: Span, split: Span, numbers: &[NumberSpan]) -> bool {
    if total_count_candidate(text, number, split) {
        return false;
    }
    if intervening_number(number, split, numbers) {
        return false;
    }
    allocation_lead_before(text, number)
}

fn inferred_unit_span(text: &str, number: Span, split: Span) -> Option<Span> {
    let between = text.get(number.end..split.start)?;
    let start_offset = between.find(|ch: char| ch.is_alphabetic())?;
    let start = number.end.saturating_add(start_offset);
    let tail = text.get(start..split.start)?;
    let stop_offset = tail
        .char_indices()
        .find(|(_, ch)| clause_break(*ch))
        .map(|(index, _)| index)
        .unwrap_or(tail.len());
    let end = trim_end(text, start, start.saturating_add(stop_offset));
    if start >= end {
        return None;
    }
    text.get(start..end).and_then(|phrase| {
        if !phrase.chars().any(|ch| ch.is_alphabetic()) || phrase_mentions_main_unit(phrase) {
            return None;
        }
        Some(Span { start, end })
    })
}

fn trim_end(text: &str, start: usize, mut end: usize) -> usize {
    while start < end {
        let Some(slice) = text.get(start..end) else {
            break;
        };
        let Some(ch) = slice.chars().next_back() else {
            break;
        };
        if !ch.is_whitespace() {
            break;
        }
        end = end.saturating_sub(ch.len_utf8());
    }
    end
}

fn rest_segment_mentions_main(text: &str, split: Span) -> bool {
    let end = split.end.saturating_add(96).min(text.len());
    text.get(split.end..end).is_some_and(|tail| {
        let sentence = tail.split(['\n', '\r', '.', '。']).next().unwrap_or(tail);
        let lower = sentence.to_lowercase();
        lower.contains("main content")
            || (lower.contains("ordered")
                && lower
                    .split(|ch: char| !ch.is_alphanumeric())
                    .any(main_unit_word))
    })
}

fn total_count_candidate(text: &str, number: Span, split: Span) -> bool {
    text.get(number.start..split.start).is_some_and(|segment| {
        let lower = segment.to_lowercase();
        lower
            .split(|ch: char| !ch.is_alphanumeric())
            .any(|word| matches!(word, "total" | "altogether" | "overall"))
    })
}

fn intervening_number(number: Span, split: Span, numbers: &[NumberSpan]) -> bool {
    numbers
        .iter()
        .any(|other| number.start < other.span.start && other.span.start < split.start)
}

fn main_unit_word(word: &str) -> bool {
    matches!(
        word,
        "section"
            | "sections"
            | "chapter"
            | "chapters"
            | "module"
            | "modules"
            | "part"
            | "parts"
            | "draft"
            | "drafts"
            | "content"
    )
}

fn phrase_mentions_main_unit(phrase: &str) -> bool {
    let lower = phrase.to_lowercase();
    lower
        .split(|ch: char| !ch.is_alphanumeric())
        .any(main_unit_word)
}

fn clause_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '.' | ',' | ';' | '。' | '、' | '；')
}
