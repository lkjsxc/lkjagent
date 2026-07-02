pub fn count_words(text: &str) -> usize {
    let latin = text
        .split_whitespace()
        .filter(|token| token.chars().any(|c| c.is_ascii_alphanumeric()))
        .count();
    let cjk = text.chars().filter(|c| is_cjk(*c)).count();
    latin + cjk
}

fn is_cjk(c: char) -> bool {
    matches!(
        c as u32,
        0x3400..=0x9fff | 0x3040..=0x30ff | 0xf900..=0xfaff
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_latin_cjk_and_mixed_text() {
        assert_eq!(count_words("Aurora Ledger opened"), 3);
        assert_eq!(count_words("\u{661f}\u{304c}\u{5149}\u{308b}"), 4);
        assert_eq!(count_words("Chapter 1 \u{661f}"), 3);
    }
}
