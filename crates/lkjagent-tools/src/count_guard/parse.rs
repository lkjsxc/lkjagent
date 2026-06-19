use super::{CountGuard, CountKind, CountMode};

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
        spans.extend(matches(lower, needle));
    }
    for needle in ["ファイル", "文書", "ドキュメント", "マークダウン"] {
        spans.extend(matches(content, needle));
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
        || content.contains("ぐらい")
        || content.contains("くらい")
        || content.contains("程度")
        || content.contains("約")
}

fn exact_signal(lower: &str, content: &str) -> bool {
    lower.contains("exact")
        || lower.contains("exactly")
        || lower.contains("precisely")
        || content.contains("ちょうど")
        || content.contains("ぴったり")
        || content.contains("正確")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NumberSpan {
    value: usize,
    span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Span {
    start: usize,
    end: usize,
}

fn number_spans(text: &str) -> Vec<NumberSpan> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut start = 0_usize;
    let mut end = 0_usize;
    for (index, ch) in text.char_indices() {
        if let Some(digit) = digit_char(ch) {
            if current.is_empty() {
                start = index;
            }
            current.push(digit);
            end = index.saturating_add(ch.len_utf8());
        } else if number_separator(ch) && !current.is_empty() {
            continue;
        } else {
            save_number(&mut values, &mut current, start, end);
        }
    }
    save_number(&mut values, &mut current, start, end);
    values
}

fn digit_char(ch: char) -> Option<char> {
    if ch.is_ascii_digit() {
        return Some(ch);
    }
    if ('０'..='９').contains(&ch) {
        return char::from_digit(ch as u32 - '０' as u32, 10);
    }
    None
}

fn number_separator(ch: char) -> bool {
    matches!(ch, ',' | '_' | '，')
}

fn save_number(values: &mut Vec<NumberSpan>, current: &mut String, start: usize, end: usize) {
    if current.is_empty() {
        return;
    }
    if let Ok(value) = current.parse() {
        values.push(NumberSpan {
            value,
            span: Span { start, end },
        });
    }
    current.clear();
}

fn matches(text: &str, needle: &str) -> Vec<Span> {
    text.match_indices(needle)
        .map(|(start, found)| Span {
            start,
            end: start.saturating_add(found.len()),
        })
        .collect()
}

fn span_distance(left: Span, right: Span) -> usize {
    if left.end <= right.start {
        right.start.saturating_sub(left.end)
    } else if right.end <= left.start {
        left.start.saturating_sub(right.end)
    } else {
        0
    }
}
