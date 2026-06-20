use crate::count_profile::{DeliverableKind, Language};

pub(crate) fn acceptance_audit(
    language: Language,
    kind: DeliverableKind,
    docs: usize,
    main: usize,
) -> String {
    match language {
        Language::Japanese => jp_audit(kind, docs, main),
        Language::English => en_audit(kind, docs, main),
    }
}

fn jp_audit(kind: DeliverableKind, docs: usize, main: usize) -> String {
    format!(
        "## 受入監査\n\n- 構造: README.md、docs/README.md、main/README.md が成果物の入口です。\n- 設計範囲: {docs} 件の設計メモが main ファイル範囲を分担します。\n- 本編範囲: {main} 件の main ファイルは再帰的な arc 配下で管理します。\n- 内容契約: 各 main ファイルはセグメント概要、連続性台帳、本文、継続メモを持ちます。\n- 種別契約: この成果物は {} として監査します。\n",
        jp_kind(kind)
    )
}

fn en_audit(kind: DeliverableKind, docs: usize, main: usize) -> String {
    format!(
        "## Acceptance Audit\n\n- Structure: README.md, docs/README.md, and main/README.md are the entry points.\n- Design coverage: {docs} design memos divide the main-file range.\n- Main coverage: {main} main files live under recursive arc directories.\n- Content contract: every main file carries a segment brief, sequence ledger, body, and handoff notes.\n- Kind contract: audit this deliverable as {}.\n",
        en_kind(kind)
    )
}

fn jp_kind(kind: DeliverableKind) -> &'static str {
    match kind {
        DeliverableKind::Narrative => "物語",
        DeliverableKind::Guide => "手順書",
        DeliverableKind::Report => "報告書",
        DeliverableKind::General => "汎用成果物",
    }
}

fn en_kind(kind: DeliverableKind) -> &'static str {
    match kind {
        DeliverableKind::Narrative => "a narrative",
        DeliverableKind::Guide => "a guide",
        DeliverableKind::Report => "a report",
        DeliverableKind::General => "a general deliverable",
    }
}
