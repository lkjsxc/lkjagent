#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManuscriptFields {
    pub title: String,
    pub root: String,
    pub chapter_count: usize,
    pub total_words: usize,
    pub glob: String,
    pub note: Option<String>,
}

pub fn extract(objective: &str) -> ManuscriptFields {
    let lower = objective.to_ascii_lowercase();
    let title = title(&lower);
    let root = root(&lower, &title);
    let chapter_count = number_before(&lower, "chapter")
        .or_else(|| kanji_number_before(objective, '章'))
        .unwrap_or(10);
    let (total_words, note) = match word_target(objective, &lower) {
        Some(words) => (words, None),
        None => (10_000, Some("word target defaulted".to_string())),
    };
    ManuscriptFields {
        title,
        root: root.clone(),
        chapter_count,
        total_words,
        glob: format!("{root}/manuscript/*.md"),
        note,
    }
}

fn title(lower: &str) -> String {
    if lower.contains("aurora ledger") {
        "aurora-ledger".to_string()
    } else if lower.contains("iwanna") {
        "iwanna".to_string()
    } else {
        "manuscript".to_string()
    }
}

fn root(lower: &str, title: &str) -> String {
    lower
        .split_whitespace()
        .find(|word| word.starts_with("stories/") || word.starts_with("manuscripts/"))
        .map(clean)
        .unwrap_or_else(|| format!("stories/{title}"))
}

fn word_target(original: &str, lower: &str) -> Option<usize> {
    number_before(lower, "word").or_else(|| {
        original
            .split_whitespace()
            .find_map(|word| word.strip_suffix('語').and_then(kanji_number))
    })
}

fn number_before(lower: &str, marker: &str) -> Option<usize> {
    let words = lower.split_whitespace().collect::<Vec<_>>();
    words.windows(2).find_map(|pair| {
        if pair[1].starts_with(marker) {
            digits(pair[0])
        } else {
            None
        }
    })
}

fn kanji_number_before(text: &str, marker: char) -> Option<usize> {
    text.split(marker)
        .next()
        .and_then(|before| before.split_whitespace().last())
        .and_then(kanji_number)
}

fn digits(text: &str) -> Option<usize> {
    text.chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}

fn kanji_number(text: &str) -> Option<usize> {
    match text {
        "十" => Some(10),
        "一万" => Some(10_000),
        "二千" => Some(2_000),
        "千" => Some(1_000),
        _ => digits(text),
    }
}

fn clean(word: &str) -> String {
    word.trim_matches(|ch: char| ch == ',' || ch == '.' || ch == ':' || ch == ';')
        .to_string()
}
