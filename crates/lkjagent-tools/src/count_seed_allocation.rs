const MAX_DESIGN_SIGNAL_DISTANCE: usize = 72;
const MAX_LOCAL_DESIGN_DISTANCE: usize = 36;
const MAX_LOCAL_FILE_DISTANCE: usize = 18;

use crate::count_number::{number_spans, span_distance, Span};
use crate::count_seed_allocation_signals::{design_signal_spans, file_signal_spans};
use crate::count_seed_allocation_split::remaining_split_hint;

pub(crate) struct Allocation {
    pub(crate) docs: usize,
    pub(crate) main: usize,
    pub(crate) indexes: bool,
}

impl Allocation {
    pub(crate) fn index_files(&self) -> usize {
        if self.indexes {
            2
        } else {
            0
        }
    }
}

pub(crate) fn allocation_for(target: usize, objective: &str) -> Allocation {
    if target <= 1 {
        return Allocation {
            docs: 0,
            main: 0,
            indexes: false,
        };
    }
    if target == 2 {
        return Allocation {
            docs: 0,
            main: 1,
            indexes: false,
        };
    }
    let content = target.saturating_sub(3);
    let mut docs = design_count_hint(objective)
        .and_then(|hint| bounded_design_count(hint, content))
        .unwrap_or_else(|| content.min(12));
    let mut main = content.saturating_sub(docs);
    if content > 0 && main == 0 {
        docs = docs.saturating_sub(1);
        main = 1;
    }
    Allocation {
        docs,
        main,
        indexes: true,
    }
}

fn bounded_design_count(hint: usize, content: usize) -> Option<usize> {
    if hint == 0 || content <= 1 {
        return None;
    }
    Some(hint.min(content.saturating_sub(1)))
}

fn design_count_hint(objective: &str) -> Option<usize> {
    let lower = objective.to_lowercase();
    let file_signals = file_signal_spans(&lower, objective);
    if let Some(hint) = remaining_split_hint(objective, &lower, &file_signals) {
        return Some(hint);
    }
    let design_signals = design_signal_spans(&lower, objective);
    if design_signals.is_empty() {
        return None;
    }
    if let Some(hint) = local_design_file_hint(objective, &design_signals, &file_signals) {
        return Some(hint);
    }
    number_spans(objective)
        .into_iter()
        .filter(|number| number.value > 0)
        .filter_map(|number| {
            let score = closest_distance(number.span, &design_signals)?;
            if score > MAX_DESIGN_SIGNAL_DISTANCE {
                return None;
            }
            if closest_distance(number.span, &file_signals).is_some_and(|file| file < score) {
                return None;
            }
            Some((score, number.value))
        })
        .min_by_key(|(score, value)| (*score, *value))
        .map(|(_, value)| value)
}

fn local_design_file_hint(
    objective: &str,
    design_signals: &[Span],
    file_signals: &[Span],
) -> Option<usize> {
    number_spans(objective)
        .into_iter()
        .filter(|number| number.value > 0)
        .filter_map(|number| {
            local_design_file_score(objective, number.span, design_signals, file_signals)
                .map(|score| (score, number.value))
        })
        .min_by_key(|(score, value)| (*score, *value))
        .map(|(_, value)| value)
}

fn local_design_file_score(
    objective: &str,
    number: Span,
    design_signals: &[Span],
    file_signals: &[Span],
) -> Option<usize> {
    let mut best: Option<usize> = None;
    for design in design_signals.iter().copied() {
        let design_distance = span_distance(number, design);
        if design_distance > MAX_LOCAL_DESIGN_DISTANCE {
            continue;
        }
        for file in file_signals
            .iter()
            .copied()
            .filter(|span| same_local_phrase(objective, design, *span, number))
        {
            let file_distance = span_distance(number, file);
            if file_distance > MAX_LOCAL_FILE_DISTANCE {
                continue;
            }
            let score = design_distance.saturating_add(file_distance);
            best = Some(best.map_or(score, |current| current.min(score)));
        }
    }
    best
}

fn same_local_phrase(text: &str, design: Span, file: Span, number: Span) -> bool {
    if design.end <= number.start && number.end <= file.start {
        return same_clause(text, design.end, file.start);
    }
    if design.end <= file.start && file.end <= number.start {
        return same_clause(text, design.end, number.start);
    }
    if number.end <= design.start && design.end <= file.start {
        return same_clause(text, number.end, file.start);
    }
    false
}

fn same_clause(text: &str, start: usize, end: usize) -> bool {
    text.get(start..end)
        .is_some_and(|between| !between.chars().any(clause_break))
}

fn clause_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '.' | ',' | ';' | '。' | '、' | '；')
}

fn closest_distance(span: Span, signals: &[Span]) -> Option<usize> {
    signals
        .iter()
        .map(|signal| span_distance(span, *signal))
        .min()
}
