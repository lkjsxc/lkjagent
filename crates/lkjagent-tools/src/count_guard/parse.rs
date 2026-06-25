use super::{mode::count_mode, CountGuard, CountKind};
use crate::count_number::{number_spans, span_distance, span_matches, NumberSpan, Span};

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
    let mode = count_mode(target.span, lower, content);
    Some(CountGuard {
        kind,
        target: target.value,
        mode,
    })
}

fn target_number(lower: &str, content: &str, signals: &[Span]) -> Option<NumberSpan> {
    let aggregate_signals = aggregate_signal_spans(lower, content);
    let non_file_units = non_file_unit_spans(lower, content);
    number_spans(content)
        .into_iter()
        .filter_map(|number| {
            let file_score = signals
                .iter()
                .map(|signal| span_distance(number.span, *signal))
                .min();
            let unit_score = non_file_units
                .iter()
                .map(|unit| span_distance(number.span, *unit))
                .min();
            if unit_score.is_some_and(|unit| file_score.is_none_or(|file| unit < file)) {
                return None;
            }
            let aggregate_score =
                aggregate_signal_score(number.span, &aggregate_signals, &non_file_units);
            let score = match (file_score, aggregate_score) {
                (Some(file), Some(aggregate)) => file.min(aggregate),
                (Some(file), None) => file,
                (None, Some(aggregate)) => aggregate,
                (None, None) => return None,
            };
            (score <= MAX_SIGNAL_DISTANCE).then_some((score, number))
        })
        .min_by_key(|(score, number)| (*score, usize::MAX.saturating_sub(number.value)))
        .map(|(_, number)| number)
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
        "lines",
        "section",
        "sections",
        "paragraph",
        "paragraphs",
        "chapter",
        "chapters",
        "child",
        "children",
        "scene",
        "scenes",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in [
        "文字",
        "単語",
        "ページ",
        "頁",
        "行",
        "節",
        "章",
        "場面",
        "シーン",
    ] {
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
