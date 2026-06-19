use crate::count_number::{number_spans, span_distance, span_matches, Span};

const MAX_SPLIT_FILE_DISTANCE: usize = 48;
const MAX_REMAINING_DISTANCE: usize = 64;

pub(crate) fn remaining_split_hint(
    objective: &str,
    lower: &str,
    file_signals: &[Span],
) -> Option<usize> {
    let split_signals = split_signal_spans(lower);
    if file_signals.is_empty() || split_signals.is_empty() {
        return None;
    }
    number_spans(objective)
        .into_iter()
        .filter(|number| number.value > 0)
        .filter_map(|number| {
            split_score(objective, number.span, file_signals, &split_signals)
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
        "rest of the files",
        "rest of the documents",
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
            if !same_clause(text, number.end, split.start) {
                continue;
            }
            let score = file_distance.saturating_add(split_distance);
            best = Some(best.map_or(score, |current| current.min(score)));
        }
    }
    best
}

fn same_clause(text: &str, start: usize, end: usize) -> bool {
    text.get(start..end)
        .is_some_and(|between| !between.chars().any(clause_break))
}

fn clause_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '.' | ',' | ';' | '。' | '、' | '；')
}
