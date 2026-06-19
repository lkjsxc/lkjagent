const MAX_DESIGN_SIGNAL_DISTANCE: usize = 72;

use crate::count_number::{number_spans, span_distance, span_matches, Span};

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
    let design_signals = design_signal_spans(&lower, objective);
    if design_signals.is_empty() {
        return None;
    }
    let file_signals = file_signal_spans(&lower, objective);
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

fn closest_distance(span: Span, signals: &[Span]) -> Option<usize> {
    signals
        .iter()
        .map(|signal| span_distance(span, *signal))
        .min()
}

fn design_signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in [
        "design",
        "memo",
        "memos",
        "planning",
        "viewpoint",
        "viewpoints",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["設計", "観点", "メモ"] {
        spans.extend(span_matches(content, needle));
    }
    spans
}

fn file_signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in ["file", "files", "document", "documents", "docs", ".md"] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["ファイル", "文書", "ドキュメント", "マークダウン"] {
        spans.extend(span_matches(content, needle));
    }
    spans
}
