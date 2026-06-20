use crate::count_number::{number_spans, span_distance, span_matches, NumberSpan, Span};
use crate::count_seed_allocation_infer::inferred_split_unit_spans;
use crate::count_seed_allocation_lead::allocation_lead_before;
use crate::count_seed_allocation_units::split_unit_spans;

const MAX_SPLIT_FILE_DISTANCE: usize = 48;
const MAX_REMAINING_DISTANCE: usize = 64;

pub(crate) fn remaining_split_hint(
    objective: &str,
    lower: &str,
    file_signals: &[Span],
) -> Option<usize> {
    let split_signals = split_signal_spans(lower);
    if split_signals.is_empty() {
        return None;
    }
    let numbers = number_spans(objective);
    let mut unit_signals = split_unit_spans(lower, file_signals);
    unit_signals.extend(inferred_split_unit_spans(
        objective,
        &numbers,
        &split_signals,
    ));
    if unit_signals.is_empty() {
        return None;
    }
    numbers
        .iter()
        .copied()
        .filter(|number| number.value > 0)
        .filter_map(|number| {
            split_score(
                objective,
                number.span,
                &unit_signals,
                &split_signals,
                &numbers,
            )
            .map(|score| (score, number.value))
        })
        .min_by_key(|(score, value)| (*score, *value))
        .map(|(_, value)| value)
}

fn split_signal_spans(lower: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in [
        "remaining file",
        "remaining files",
        "remaining document",
        "remaining documents",
        "remaining docs",
        "remainder",
        "all other files",
        "all other documents",
        "all other docs",
        "everything else",
        "the rest",
        "rest of the files",
        "rest of the documents",
        "残り",
        "それ以外",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    spans
}

fn split_score(
    text: &str,
    number: Span,
    file_signals: &[Span],
    split_signals: &[Span],
    numbers: &[NumberSpan],
) -> Option<usize> {
    let mut best: Option<usize> = None;
    for file in file_signals
        .iter()
        .copied()
        .filter(|file| number.end <= file.start)
    {
        let file_distance = span_distance(number, file);
        if file_distance > MAX_SPLIT_FILE_DISTANCE {
            continue;
        }
        for split in split_signals
            .iter()
            .copied()
            .filter(|split| file.end <= split.start)
        {
            let split_distance = span_distance(file, split);
            if split_distance > MAX_REMAINING_DISTANCE {
                continue;
            }
            if !split_segment_allowed(text, number, split, numbers) {
                continue;
            }
            let score = file_distance.saturating_add(split_distance);
            best = Some(best.map_or(score, |current| current.min(score)));
        }
    }
    best
}

fn split_segment_allowed(text: &str, number: Span, split: Span, numbers: &[NumberSpan]) -> bool {
    if total_count_candidate(text, number, split) {
        return false;
    }
    if same_clause(text, number.end, split.start) {
        return true;
    }
    soft_separator_segment(text, number, split, numbers)
        || sentence_separator_segment(text, number, split, numbers)
}

fn soft_separator_segment(text: &str, number: Span, split: Span, numbers: &[NumberSpan]) -> bool {
    let Some(between) = text.get(number.end..split.start) else {
        return false;
    };
    if !between.chars().any(soft_separator) || between.chars().any(hard_break) {
        return false;
    }
    if intervening_number(number, split, numbers) {
        return false;
    }
    allocation_lead_before(text, number) || allocation_after_number(text, number, split)
}

fn sentence_separator_segment(
    text: &str,
    number: Span,
    split: Span,
    numbers: &[NumberSpan],
) -> bool {
    let Some(between) = text.get(number.end..split.start) else {
        return false;
    };
    if !between.chars().any(sentence_separator) || between.chars().any(line_break) {
        return false;
    }
    if intervening_number(number, split, numbers) {
        return false;
    }
    allocation_lead_before(text, number) || allocation_after_number(text, number, split)
}

fn allocation_after_number(text: &str, number: Span, split: Span) -> bool {
    text.get(number.end..split.start).is_some_and(|between| {
        between.contains('使')
            || between.contains("用意")
            || between.contains("作り")
            || between.contains("作成")
    })
}

fn intervening_number(number: Span, split: Span, numbers: &[NumberSpan]) -> bool {
    numbers
        .iter()
        .any(|other| number.start < other.span.start && other.span.start < split.start)
}

fn total_count_candidate(text: &str, number: Span, split: Span) -> bool {
    text.get(number.start..split.start).is_some_and(|segment| {
        let lower = segment.to_lowercase();
        if lower.contains("合計") || lower.contains("総計") || lower.contains("合わせて") {
            return true;
        }
        lower
            .split(|ch: char| !ch.is_alphanumeric())
            .any(|word| matches!(word, "total" | "altogether" | "overall"))
    })
}

fn same_clause(text: &str, start: usize, end: usize) -> bool {
    text.get(start..end)
        .is_some_and(|between| !between.chars().any(clause_break))
}

fn hard_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '.' | '。')
}

fn line_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r')
}

fn sentence_separator(ch: char) -> bool {
    matches!(ch, '.' | '。')
}

fn soft_separator(ch: char) -> bool {
    matches!(ch, ',' | ';' | '、' | '；')
}

fn clause_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '.' | ',' | ';' | '。' | '、' | '；')
}
