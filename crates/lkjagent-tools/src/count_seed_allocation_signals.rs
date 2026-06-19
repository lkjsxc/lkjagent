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
        "assessment rubric",
        "assessment rubrics",
        "evaluation rubric",
        "evaluation rubrics",
        "scoring rubric",
        "scoring rubrics",
        "design",
        "decision record",
        "decision records",
        "decision log",
        "decision logs",
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
        "timeline",
        "timelines",
        "lesson plan",
        "lesson plans",
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
        "character bio",
        "character bios",
        "character biography",
        "character biographies",
        "cast profile",
        "cast profiles",
        "cast bio",
        "cast bios",
        "cast biography",
        "cast biographies",
        "persona bio",
        "persona bios",
        "persona biography",
        "persona biographies",
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
