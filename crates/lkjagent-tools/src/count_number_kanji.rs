pub(crate) fn number_char(ch: char) -> bool {
    digit(ch).is_some() || unit(ch)
}

pub(crate) fn boundary_allows(next: Option<char>) -> bool {
    match next {
        Some(ch) if cjk_ideograph(ch) => count_suffix(ch),
        _ => true,
    }
}

pub(crate) fn parse(text: &str) -> Option<usize> {
    if !text.chars().any(unit) {
        return parse_digits(text);
    }
    let mut total = 0_usize;
    let mut section = 0_usize;
    let mut current = 0_usize;
    let mut saw = false;
    for ch in text.chars() {
        if let Some(digit) = digit(ch) {
            current = digit;
            saw = true;
        } else if let Some(unit) = small_unit(ch) {
            let amount = if current == 0 { 1 } else { current };
            section = section.saturating_add(amount.saturating_mul(unit));
            current = 0;
            saw = true;
        } else if ch == '万' {
            let amount = section.saturating_add(current).max(1);
            total = total.saturating_add(amount.saturating_mul(10_000));
            section = 0;
            current = 0;
            saw = true;
        } else {
            return None;
        }
    }
    saw.then_some(total.saturating_add(section).saturating_add(current))
}

fn parse_digits(text: &str) -> Option<usize> {
    let mut digits = String::new();
    for ch in text.chars() {
        digits.push(char::from_digit(digit(ch)? as u32, 10)?);
    }
    digits.parse().ok()
}

fn digit(ch: char) -> Option<usize> {
    match ch {
        '零' | '〇' => Some(0),
        '一' => Some(1),
        '二' => Some(2),
        '三' => Some(3),
        '四' => Some(4),
        '五' => Some(5),
        '六' => Some(6),
        '七' => Some(7),
        '八' => Some(8),
        '九' => Some(9),
        _ => None,
    }
}

fn unit(ch: char) -> bool {
    matches!(ch, '十' | '百' | '千' | '万')
}

fn small_unit(ch: char) -> Option<usize> {
    match ch {
        '十' => Some(10),
        '百' => Some(100),
        '千' => Some(1000),
        _ => None,
    }
}

fn cjk_ideograph(ch: char) -> bool {
    ('\u{4e00}'..='\u{9fff}').contains(&ch)
}

fn count_suffix(ch: char) -> bool {
    matches!(
        ch,
        '章' | '件'
            | '個'
            | '本'
            | '枚'
            | '冊'
            | '頁'
            | '行'
            | '字'
            | '文'
            | '項'
            | '節'
            | '部'
            | '巻'
            | '話'
            | '設'
            | '観'
    )
}
