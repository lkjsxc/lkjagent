use crate::count_number::{NumberSpan, Span};

struct Token {
    word: String,
    start: usize,
    end: usize,
}

pub(crate) fn number_spans(text: &str) -> Vec<NumberSpan> {
    let tokens = word_tokens(text);
    let mut spans = Vec::new();
    let mut index = 0_usize;
    while index < tokens.len() {
        if let Some((end, value)) = parse_from(&tokens, index) {
            spans.push(NumberSpan {
                value,
                span: Span {
                    start: tokens[index].start,
                    end: tokens[end].end,
                },
            });
            index = end.saturating_add(1);
        } else {
            index = index.saturating_add(1);
        }
    }
    spans
}

fn parse_from(tokens: &[Token], start: usize) -> Option<(usize, usize)> {
    let mut index = start;
    let mut total = 0_usize;
    let mut current = 0_usize;
    let mut consumed = false;
    let mut scaled = false;
    while let Some(token) = tokens.get(index) {
        if bare_scale_start(token) && !consumed {
            current = scale_value(&token.word)?;
            consumed = true;
            scaled = true;
            index = index.saturating_add(1);
            continue;
        }
        if article_scale_start(tokens, index) && !consumed {
            current = 1;
            consumed = true;
            index = index.saturating_add(1);
            continue;
        }
        if token.word == "and" && consumed && next_is_number(tokens, index.saturating_add(1)) {
            index = index.saturating_add(1);
            continue;
        }
        if let Some(value) = small_number(&token.word) {
            current = current.saturating_add(value);
            consumed = true;
        } else if let Some(value) = tens_number(&token.word) {
            current = current.saturating_add(value);
            consumed = true;
        } else if token.word == "hundred" && consumed {
            current = current.max(1).saturating_mul(100);
            scaled = true;
        } else if token.word == "thousand" && consumed {
            total = total.saturating_add(current.max(1).saturating_mul(1000));
            current = 0;
            scaled = true;
        } else {
            break;
        }
        index = index.saturating_add(1);
    }
    if !consumed {
        return None;
    }
    let end = index.checked_sub(1)?;
    let value = total.saturating_add(current);
    let multi_word = end > start;
    (scaled || multi_word || value <= 20 || value.is_multiple_of(10)).then_some((end, value))
}

fn bare_scale_start(token: &Token) -> bool {
    matches!(
        token.word.as_str(),
        "hundred" | "hundreds" | "thousand" | "thousands"
    )
}

fn scale_value(word: &str) -> Option<usize> {
    match word {
        "hundred" | "hundreds" => Some(100),
        "thousand" | "thousands" => Some(1000),
        _ => None,
    }
}

fn article_scale_start(tokens: &[Token], index: usize) -> bool {
    tokens.get(index).is_some_and(|token| {
        matches!(token.word.as_str(), "a" | "an")
            && tokens
                .get(index.saturating_add(1))
                .is_some_and(|next| matches!(next.word.as_str(), "hundred" | "thousand"))
    })
}

fn next_is_number(tokens: &[Token], index: usize) -> bool {
    tokens.get(index).is_some_and(|token| {
        small_number(&token.word).is_some()
            || tens_number(&token.word).is_some()
            || token.word == "hundred"
            || token.word == "thousand"
    })
}

fn word_tokens(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut start = 0_usize;
    let mut end = 0_usize;
    for (index, ch) in text.char_indices() {
        if ch.is_ascii_alphabetic() {
            if current.is_empty() {
                start = index;
            }
            current.push(ch.to_ascii_lowercase());
            end = index.saturating_add(ch.len_utf8());
        } else {
            save_token(&mut tokens, &mut current, start, end);
        }
    }
    save_token(&mut tokens, &mut current, start, end);
    tokens
}

fn save_token(tokens: &mut Vec<Token>, current: &mut String, start: usize, end: usize) {
    if current.is_empty() {
        return;
    }
    tokens.push(Token {
        word: std::mem::take(current),
        start,
        end,
    });
}

fn small_number(word: &str) -> Option<usize> {
    match word {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        "eleven" => Some(11),
        "twelve" => Some(12),
        "thirteen" => Some(13),
        "fourteen" => Some(14),
        "fifteen" => Some(15),
        "sixteen" => Some(16),
        "seventeen" => Some(17),
        "eighteen" => Some(18),
        "nineteen" => Some(19),
        _ => None,
    }
}

fn tens_number(word: &str) -> Option<usize> {
    match word {
        "twenty" => Some(20),
        "thirty" => Some(30),
        "forty" => Some(40),
        "fifty" => Some(50),
        "sixty" => Some(60),
        "seventy" => Some(70),
        "eighty" => Some(80),
        "ninety" => Some(90),
        _ => None,
    }
}
