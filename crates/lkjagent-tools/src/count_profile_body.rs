use crate::count_profile::{DeliverableKind, Language};
use crate::count_profile_detail::detail_block;
use crate::count_profile_passage::passage_block;
use crate::count_profile_paths::main_path;
use crate::count_profile_stage::stage_name;

pub(crate) fn main_title(language: Language, kind: DeliverableKind, index: usize) -> String {
    match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => format!("本編 {index:03}"),
        (Language::Japanese, _) => format!("本文 {index:03}"),
        (Language::English, DeliverableKind::Narrative) => {
            format!("Narrative Segment {index:03}")
        }
        (Language::English, _) => format!("Main Content {index:03}"),
    }
}

pub(crate) fn body_text(
    language: Language,
    kind: DeliverableKind,
    index: usize,
    total: usize,
    anchor: &str,
) -> String {
    let body = match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => jp_narrative(index, total),
        (Language::Japanese, DeliverableKind::Guide) => jp_guide(index, total),
        (Language::Japanese, DeliverableKind::Report) => jp_report(index, total),
        (Language::Japanese, DeliverableKind::General) => jp_general(index, total),
        (Language::English, DeliverableKind::Narrative) => en_narrative(index, total),
        (Language::English, DeliverableKind::Guide) => en_guide(index, total),
        (Language::English, DeliverableKind::Report) => en_report(index, total),
        (Language::English, DeliverableKind::General) => en_general(index, total),
    };
    let detail = detail_block(language, kind, index, total, anchor);
    let passage = passage_block(language, kind, index, total, anchor);
    format!(
        "{body}\n\n{detail}\n\n{passage}\n\n{}",
        anchor_link(language, anchor)
    )
}

pub(crate) fn sequence_text(language: Language, index: usize, total: usize) -> String {
    let previous = previous_label(language, index);
    let next = next_label(language, index, total);
    let current = main_path(index);
    match language {
        Language::Japanese => format!(
            "- 前: {previous}\n- 現在: {current}\n- 次: {next}\n- 現在の段階: {}",
            stage_name(language, index, total)
        ),
        Language::English => format!(
            "- Previous: {previous}\n- Current: {current}\n- Next: {next}\n- Current stage: {}",
            stage_name(language, index, total)
        ),
    }
}

pub(crate) fn handoff_text(language: Language, index: usize, total: usize) -> String {
    if index >= total {
        return match language {
            Language::Japanese => {
                "- 最終節として、未解決の論点と完成条件を明確にします。".to_string()
            }
            Language::English => {
                "- Close the sequence by naming unresolved points and completion checks."
                    .to_string()
            }
        };
    }
    let next = index.saturating_add(1);
    let next_path = main_path(next);
    match language {
        Language::Japanese => {
            format!("- 前節までの用語、判断、未解決点を引き継ぎます。\n- 次の接続先: {next_path}")
        }
        Language::English => format!(
            "- Carry forward terms, decisions, and open questions from earlier parts.\n- Next segment: {next_path}"
        ),
    }
}

fn previous_label(language: Language, index: usize) -> String {
    if index <= 1 {
        return match language {
            Language::Japanese => "なし".to_string(),
            Language::English => "none".to_string(),
        };
    }
    main_path(index.saturating_sub(1))
}

fn next_label(language: Language, index: usize, total: usize) -> String {
    if index >= total {
        return match language {
            Language::Japanese => "なし".to_string(),
            Language::English => "none".to_string(),
        };
    }
    main_path(index.saturating_add(1))
}

fn jp_narrative(index: usize, total: usize) -> String {
    let stage = stage_name(Language::Japanese, index, total);
    let turn = if index >= total {
        "最終節では、残された対立を整理し、完了後に残る余韻を明確にします。".to_string()
    } else {
        format!(
            "この節の終わりでは、人物の理解か状況の均衡を一つ変え、全{total}本の流れの中で次節へ進む理由を残します。"
        )
    };
    format!(
        "### 場面の役割\n第{index}節は「{stage}」の段階として、状況、願い、障害を一つの場面に集約します。\n\n### 展開\n登場する視点、場所、対立の焦点を明確にし、読者が次の変化を期待できるように小さな決断を置きます。\n\n### 転換点\n{turn}"
    )
}

fn jp_guide(index: usize, total: usize) -> String {
    format!(
        "### 手順の役割\n第{index}節は全{total}本の手順の一部として、前提、操作、確認方法を一つの実行単位にまとめます。\n\n### 実行内容\n読者が同じ結果を再現できるように、入力、判断基準、完了条件を明確にします。\n\n### 確認\n次の節へ進む前に観察すべき状態と、失敗した場合に戻る地点を記録します。"
    )
}

fn jp_report(index: usize, total: usize) -> String {
    format!(
        "### 論点の役割\n第{index}節は全{total}本の報告の中で、一つの観点、根拠、示唆を担当します。\n\n### 分析\n同じ論点を繰り返さず、前節までの整理に新しい判断材料を加えます。\n\n### 含意\n次に検証すべき疑問、比較対象、または意思決定への影響を明確にします。"
    )
}

fn jp_general(index: usize, total: usize) -> String {
    format!(
        "### 節の役割\n第{index}節は全{total}本の成果物の一部として、依頼内容を独立して読める単位へ展開します。\n\n### 内容\n目的、前提、具体的な展開を分けて書き、周辺ファイルとの重複を避けます。\n\n### 接続\n前後の節とつながる未解決点、用語、次の作業を残します。"
    )
}

fn en_narrative(index: usize, total: usize) -> String {
    let stage = stage_name(Language::English, index, total);
    let turn = if index >= total {
        "Close the final segment with the public consequence, remaining cost, and completion state."
            .to_string()
    } else {
        format!(
            "End the segment with one changed understanding or unstable condition that justifies the next part in the {total}-file sequence."
        )
    };
    format!(
        "### Scene Role\nSegment {index} serves the {stage} stage by concentrating a situation, a desire, and an obstacle into one readable scene.\n\n### Development\nName the viewpoint, place, and conflict pressure clearly enough that the reader can track the next change.\n\n### Turn\n{turn}"
    )
}

fn en_guide(index: usize, total: usize) -> String {
    format!(
        "### Procedure Role\nSegment {index} is one executable unit in the {total}-file guide: premise, operation, and check.\n\n### Action\nState the input, decision rule, and expected result so the reader can repeat the step.\n\n### Check\nRecord the state to observe before continuing and the point to revisit if the step fails."
    )
}

fn en_report(index: usize, total: usize) -> String {
    format!(
        "### Analysis Role\nSegment {index} owns one perspective, basis, and implication inside the {total}-file report.\n\n### Evidence\nAdd one useful judgment without repeating earlier sections.\n\n### Implication\nName the next question, comparison, or decision impact that follows from this segment."
    )
}

fn en_general(index: usize, total: usize) -> String {
    format!(
        "### Section Role\nSegment {index} develops one standalone unit inside the {total}-file deliverable.\n\n### Content\nSeparate purpose, premise, and concrete development while avoiding overlap with nearby files.\n\n### Link\nLeave terms, open questions, and next work that connect this segment to the sequence."
    )
}

fn anchor_link(language: Language, anchor: &str) -> String {
    match language {
        Language::Japanese => {
            format!("### 要求との接続\nこの節では「{anchor}」を具体化する材料を本文内に置きます。")
        }
        Language::English => {
            format!(
                "### Requirement Link\nThis segment gives concrete form to \"{anchor}\" inside the main content."
            )
        }
    }
}
