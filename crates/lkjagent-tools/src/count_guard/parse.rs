use super::{CountGuard, CountKind, CountMode};
use crate::count_number::{number_spans, span_distance, span_matches, Span};

const MAX_SIGNAL_DISTANCE: usize = 80;
const MAX_AGGREGATE_SIGNAL_DISTANCE: usize = 32;

pub(super) fn count_target(lower: &str, content: &str) -> Option<CountGuard> {
    let signals = signal_spans(lower, content);
    if signals.is_empty() {
        return None;
    }
    let target = target_number(lower, content, &signals)?;
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

fn target_number(lower: &str, content: &str, signals: &[Span]) -> Option<usize> {
    let aggregate_signals = aggregate_signal_spans(lower, content);
    let non_file_units = non_file_unit_spans(lower, content);
    number_spans(content)
        .into_iter()
        .filter_map(|number| {
            let file_score = signals
                .iter()
                .map(|signal| span_distance(number.span, *signal))
                .min();
            let aggregate_score =
                aggregate_signal_score(number.span, &aggregate_signals, &non_file_units);
            let score = match (file_score, aggregate_score) {
                (Some(file), Some(aggregate)) => file.min(aggregate),
                (Some(file), None) => file,
                (None, Some(aggregate)) => aggregate,
                (None, None) => return None,
            };
            (score <= MAX_SIGNAL_DISTANCE).then_some((score, number.value))
        })
        .min_by_key(|(score, value)| (*score, usize::MAX.saturating_sub(*value)))
        .map(|(_, value)| value)
}

fn aggregate_signal_score(
    number: Span,
    aggregate_signals: &[Span],
    non_file_units: &[Span],
) -> Option<usize> {
    let (score, aggregate) = aggregate_signals
        .iter()
        .map(|signal| (span_distance(number, *signal), *signal))
        .min_by_key(|(score, _)| *score)?;
    if score > MAX_AGGREGATE_SIGNAL_DISTANCE
        || non_file_units.iter().any(|unit| {
            span_distance(number, *unit) <= MAX_AGGREGATE_SIGNAL_DISTANCE
                && span_distance(aggregate, *unit) <= MAX_AGGREGATE_SIGNAL_DISTANCE
        })
    {
        return None;
    }
    Some(score)
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

fn aggregate_signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in ["total", "overall", "combined", "altogether", "in all"] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["合計", "総計", "全体", "合わせて"] {
        spans.extend(span_matches(content, needle));
    }
    spans
}

fn non_file_unit_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in [
        "word",
        "words",
        "token",
        "tokens",
        "character",
        "characters",
        "page",
        "pages",
        "section",
        "sections",
        "paragraph",
        "paragraphs",
        "chapter",
        "chapters",
        "scene",
        "scenes",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["文字", "単語", "ページ", "頁", "節", "章", "場面", "シーン"] {
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
    !negated_exact_signal(lower)
        && (lower.contains("exact")
            || lower.contains("exactly")
            || lower.contains("precisely")
            || content.contains("ちょうど")
            || content.contains("ぴったり")
            || content.contains("正確"))
}

fn negated_exact_signal(lower: &str) -> bool {
    lower.contains("not exact")
        || lower.contains("not exactly")
        || lower.contains("not precise")
        || lower.contains("not precisely")
}
