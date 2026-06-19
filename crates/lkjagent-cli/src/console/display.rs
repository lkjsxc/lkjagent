pub fn visible_width(text: &str) -> usize {
    text.chars().map(char_width).sum()
}

pub fn wrap(text: &str, prefix: &str, width: usize) -> Vec<String> {
    let available = width.saturating_sub(visible_width(prefix)).max(1);
    let mut out = Vec::new();
    for raw in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        wrap_line(raw, prefix, available, &mut out);
    }
    if out.is_empty() {
        out.push(format!("{prefix}none"));
    }
    out
}

pub fn preview(text: &str, width: usize) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    truncate(&collapsed, width)
}

pub fn truncate(text: &str, width: usize) -> String {
    if visible_width(text) <= width {
        return text.to_string();
    }
    let marker = "..";
    let limit = width.saturating_sub(marker.len());
    let mut out = String::new();
    let mut used: usize = 0;
    for ch in text.chars() {
        let next = char_width(ch);
        if used.saturating_add(next) > limit {
            break;
        }
        out.push(ch);
        used = used.saturating_add(next);
    }
    out.push_str(marker);
    out
}

fn wrap_line(raw: &str, prefix: &str, available: usize, out: &mut Vec<String>) {
    let mut current = String::new();
    let mut used: usize = 0;
    for word in raw.split_whitespace() {
        let width = visible_width(word);
        if width > available {
            flush(prefix, &mut current, &mut used, out);
            wrap_long_word(word, prefix, available, out);
            continue;
        }
        let gap = usize::from(!current.is_empty());
        if used.saturating_add(gap).saturating_add(width) > available {
            flush(prefix, &mut current, &mut used, out);
        }
        if !current.is_empty() {
            current.push(' ');
            used += 1;
        }
        current.push_str(word);
        used += width;
    }
    flush(prefix, &mut current, &mut used, out);
}

fn wrap_long_word(word: &str, prefix: &str, available: usize, out: &mut Vec<String>) {
    let mut current = String::new();
    let mut used: usize = 0;
    for ch in word.chars() {
        let width = char_width(ch);
        if used > 0 && used.saturating_add(width) > available {
            out.push(format!("{prefix}{current}"));
            current.clear();
            used = 0;
        }
        current.push(ch);
        used = used.saturating_add(width);
    }
    if !current.is_empty() {
        out.push(format!("{prefix}{current}"));
    }
}

fn flush(prefix: &str, current: &mut String, used: &mut usize, out: &mut Vec<String>) {
    if !current.is_empty() {
        out.push(format!("{prefix}{current}"));
        current.clear();
        *used = 0;
    }
}

fn char_width(ch: char) -> usize {
    let code = ch as u32;
    if ch == '\t' {
        4
    } else if ch.is_control() || combining(code) {
        0
    } else if wide(code) {
        2
    } else {
        1
    }
}

fn combining(code: u32) -> bool {
    matches!(code, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF)
}

fn wide(code: u32) -> bool {
    matches!(
        code,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1FAFF
    )
}
