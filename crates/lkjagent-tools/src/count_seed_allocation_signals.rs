use crate::count_number::{span_matches, Span};

pub(crate) fn design_signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in [
        "brief",
        "briefs",
        "blueprint",
        "blueprints",
        "checklist",
        "checklists",
        "design",
        "appendix",
        "appendices",
        "appendix note",
        "appendix notes",
        "memo",
        "memos",
        "outline",
        "outlines",
        "plan file",
        "plan files",
        "planning note",
        "planning notes",
        "planning",
        "spec",
        "specs",
        "viewpoint",
        "viewpoints",
        "lore",
        "worldbuilding",
        "world bible",
        "story bible",
        "character profile",
        "character profiles",
        "character sheet",
        "character sheets",
        "cast profile",
        "cast profiles",
    ] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in [
        "設計",
        "観点",
        "メモ",
        "構成案",
        "章立て",
        "計画",
        "設定資料",
        "参考資料",
        "背景資料",
    ] {
        spans.extend(span_matches(content, needle));
    }
    spans
}

pub(crate) fn file_signal_spans(lower: &str, content: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for needle in ["file", "files", "document", "documents", "docs", ".md"] {
        spans.extend(span_matches(lower, needle));
    }
    for needle in ["ファイル", "文書", "ドキュメント", "マークダウン"] {
        spans.extend(span_matches(content, needle));
    }
    spans
}
