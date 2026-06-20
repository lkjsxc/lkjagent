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
    long_content_request(lower, content) && creation_request(lower, content)
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
            "draft",
            "essay",
            "guide",
            "lesson",
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

fn large_content_signal(lower: &str, content: &str) -> bool {
    contains_any(
        lower,
        &["big", "large", "long", "many", "multi-file", "structured"],
    ) || contains_any(content, &["長編", "大規模", "構造化"])
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}
