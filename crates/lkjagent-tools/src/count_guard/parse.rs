use super::{CountGuard, CountKind, CountMode};
use crate::count_number::{number_spans, span_distance, span_matches, Span};

const MAX_SIGNAL_DISTANCE: usize = 80;

pub(super) fn count_target(lower: &str, content: &str) -> Option<CountGuard> {
    let signals = signal_spans(lower, content);
    if signals.is_empty() {
        return None;
    }
    let target = target_number(content, &signals)?;
    let kind = if markdown_signal(lower, content) {
        CountKind::Markdown
    } else {
        CountKind::File
    };
    let mode = if approximate_signal(lower, content) && !exact_signal(lower, content) {
        CountMode::Approximate
    } else {
        CountMode::Exact
    };
    Some(CountGuard { kind, target, mode })
}

fn target_number(text: &str, signals: &[Span]) -> Option<usize> {
    number_spans(text)
        .into_iter()
        .filter_map(|number| {
            let score = signals
                .iter()
                .map(|signal| span_distance(number.span, *signal))
                .min()?;
            (score <= MAX_SIGNAL_DISTANCE).then_some((score, number.value))
        })
        .min_by_key(|(score, value)| (*score, usize::MAX.saturating_sub(*value)))
        .map(|(_, value)| value)
}

fn signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in ["file", "document", "docs", "markdown", ".md"] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["ファイル", "文書", "ドキュメント", "マークダウン"] {
        spans.extend(span_matches(content, needle));
    }
    spans
}

fn markdown_signal(lower: &str, content: &str) -> bool {
    lower.contains("markdown")
        || lower.contains(".md")
        || content.contains("マークダウン")
        || content.contains("ドキュメント")
}

fn approximate_signal(lower: &str, content: &str) -> bool {
    lower.contains("about")
        || lower.contains("around")
        || lower.contains("roughly")
        || lower.contains("approximately")
        || lower.contains("approx ")
        || number_ish_signal(lower, content)
        || lower.contains("or so")
        || content.contains("ぐらい")
        || content.contains("くらい")
        || content.contains("程度")
        || content.contains("ほど")
        || content.contains("前後")
        || content.contains("約")
}

fn number_ish_signal(lower: &str, content: &str) -> bool {
    let numbers = number_spans(content);
    span_matches(lower, "ish").into_iter().any(|ish| {
        numbers.iter().any(|number| {
            number.span.end <= ish.start && ish.start.saturating_sub(number.span.end) <= 1
        })
    })
}

fn exact_signal(lower: &str, content: &str) -> bool {
    lower.contains("exact")
        || lower.contains("exactly")
        || lower.contains("precisely")
        || content.contains("ちょうど")
        || content.contains("ぴったり")
        || content.contains("正確")
}
