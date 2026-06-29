pub(crate) fn knowledge_request(lower: &str, content: &str) -> bool {
    lower.contains("knowledge")
        || lower.contains("wiki")
        || lower.contains("encyclopedia")
        || content.contains("百科事典")
        || content.contains("知識ベース")
}

pub(crate) fn documentation_request(lower: &str, content: &str) -> bool {
    lower.contains("doc")
        || lower.contains("readme")
        || lower.contains("markdown")
        || content.contains("文書")
        || content.contains("ドキュメント")
        || counted_content_request(lower, content)
        || long_content_request(lower, content)
}

pub(crate) fn priority_counted_content_request(lower: &str, content: &str) -> bool {
    counted_content_request(lower, content) && creation_request(lower, content)
}

pub(crate) fn priority_long_content_request(lower: &str, content: &str) -> bool {
    content_artifact_request(lower, content)
}

pub(crate) fn content_artifact_request(lower: &str, content: &str) -> bool {
    (long_content_request(lower, content) || counted_story_request(lower))
        && creation_request(lower, content)
}

pub(crate) fn operational_compaction_request(unquoted: &str) -> bool {
    unquoted.contains("context pressure")
        || unquoted.contains("compaction")
        || (unquoted.contains("compact")
            && contains_any(unquoted, &["context", "prompt", "token", "memory"]))
}

pub(crate) fn unquoted_lower(content: &str) -> String {
    let mut quote = None;
    let mut out = String::with_capacity(content.len());
    for ch in content.chars() {
        if quote == Some(ch) {
            quote = None;
            out.push(' ');
        } else if quote.is_none() && matches!(ch, '"' | '\'') {
            quote = Some(ch);
            out.push(' ');
        } else if quote.is_none() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(' ');
        }
    }
    out
}

fn counted_content_request(lower: &str, content: &str) -> bool {
    file_signal(lower, content)
        && content_signal(lower, content)
        && !code_change_action(lower, content)
}

fn long_content_request(lower: &str, content: &str) -> bool {
    large_content_signal(lower, content)
        && content_signal(lower, content)
        && !code_change_action(lower, content)
}

pub(crate) fn counted_story_request(lower: &str) -> bool {
    lower
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|word| matches!(word, "chapter" | "chapters" | "scene" | "scenes"))
        && lower.chars().any(|ch| ch.is_ascii_digit())
}

fn creation_request(lower: &str, content: &str) -> bool {
    contains_any(
        lower,
        &["build", "create", "generate", "make", "produce", "write"],
    ) || contains_any(content, &["作", "生成", "構築"])
}

fn code_change_action(lower: &str, content: &str) -> bool {
    lower
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(code_change_word)
        || contains_any(content, &["修正", "実装", "デバッグ", "リファクタ"])
}

fn code_change_word(word: &str) -> bool {
    matches!(
        word,
        "debug"
            | "debugging"
            | "fix"
            | "fixing"
            | "implement"
            | "implementing"
            | "patch"
            | "patching"
            | "refactor"
            | "refactoring"
    )
}

fn file_signal(lower: &str, content: &str) -> bool {
    lower.contains("file")
        || lower.contains(".md")
        || content.contains("ファイル")
        || content.contains("文書")
        || content.contains("ドキュメント")
}

fn content_signal(lower: &str, content: &str) -> bool {
    contains_any(
        lower,
        &[
            "article",
            "artifact",
            "book",
            "body",
            "chapter",
            "collection",
            "content",
            "corpus",
            "deliverable",
            "dictionary",
            "draft",
            "essay",
            "glossary",
            "guide",
            "lesson",
            "lexicon",
            "manual",
            "manuscript",
            "narrative",
            "novel",
            "report",
            "scene",
            "story",
            "tutorial",
        ],
    ) || contains_any(
        content,
        &[
            "本文",
            "本編",
            "章",
            "教材",
            "物語",
            "小説",
            "成果物",
            "手引き",
            "記事",
            "報告書",
        ],
    )
}

pub(crate) fn large_content_signal(lower: &str, content: &str) -> bool {
    contains_any(
        lower,
        &[
            "big",
            "comprehensive",
            "detailed",
            "full",
            "large",
            "long",
            "many",
            "multi-file",
            "structured",
        ],
    ) || contains_any(content, &["長編", "大規模", "構造化"])
}

pub(crate) fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
