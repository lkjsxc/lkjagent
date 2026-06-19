const MAX_DESIGN_SIGNAL_DISTANCE: usize = 72;

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
        spans.extend(matches(lower, needle));
    }
    for needle in ["設計", "観点", "メモ"] {
        spans.extend(matches(content, needle));
    }
    spans
}

fn file_signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in ["file", "files", "document", "documents", "docs", ".md"] {
        spans.extend(matches(lower, needle));
    }
    for needle in ["ファイル", "文書", "ドキュメント", "マークダウン"] {
        spans.extend(matches(content, needle));
    }
    spans
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
