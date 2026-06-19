use crate::count_number_kanji as kanji;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NumberSpan {
    pub(crate) value: usize,
    pub(crate) span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub(crate) fn number_spans(text: &str) -> Vec<NumberSpan> {
    let mut values = Vec::new();
    let mut digits = String::new();
    let mut digit_start = 0_usize;
    let mut digit_end = 0_usize;
    let mut kanji = String::new();
    let mut kanji_start = 0_usize;
    let mut kanji_end = 0_usize;
    for (index, ch) in text.char_indices() {
        if let Some(digit) = digit_char(ch) {
            save_kanji_number(&mut values, &mut kanji, kanji_start, kanji_end, Some(ch));
            if digits.is_empty() {
                digit_start = index;
            }
            digits.push(digit);
            digit_end = index.saturating_add(ch.len_utf8());
        } else if number_separator(ch) && !digits.is_empty() {
            continue;
        } else if kanji::number_char(ch) {
            save_digit_number(&mut values, &mut digits, digit_start, digit_end);
            if kanji.is_empty() {
                kanji_start = index;
            }
            kanji.push(ch);
            kanji_end = index.saturating_add(ch.len_utf8());
        } else {
            save_digit_number(&mut values, &mut digits, digit_start, digit_end);
            save_kanji_number(&mut values, &mut kanji, kanji_start, kanji_end, Some(ch));
        }
    }
    save_digit_number(&mut values, &mut digits, digit_start, digit_end);
    save_kanji_number(&mut values, &mut kanji, kanji_start, kanji_end, None);
    values
}

pub(crate) fn span_matches(text: &str, needle: &str) -> Vec<Span> {
    text.match_indices(needle)
        .map(|(start, found)| Span {
            start,
            end: start.saturating_add(found.len()),
        })
        .collect()
}

pub(crate) fn span_distance(left: Span, right: Span) -> usize {
    if left.end <= right.start {
        right.start.saturating_sub(left.end)
    } else if right.end <= left.start {
        left.start.saturating_sub(right.end)
    } else {
        0
    }
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

fn save_digit_number(values: &mut Vec<NumberSpan>, current: &mut String, start: usize, end: usize) {
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

fn save_kanji_number(
    values: &mut Vec<NumberSpan>,
    current: &mut String,
    start: usize,
    end: usize,
    next: Option<char>,
) {
    if current.is_empty() {
        return;
    }
    if kanji::boundary_allows(next) {
        if let Some(value) = kanji::parse(current) {
            values.push(NumberSpan {
                value,
                span: Span { start, end },
            });
        }
    }
    current.clear();
}
