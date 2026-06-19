use crate::count_profile::{DeliverableKind, Language};

pub(crate) fn detail_block(
    language: Language,
    kind: DeliverableKind,
    index: usize,
    total: usize,
    anchor: &str,
) -> String {
    match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => jp_narrative(index, anchor),
        (Language::Japanese, DeliverableKind::Guide) => jp_guide(index, total, anchor),
        (Language::Japanese, DeliverableKind::Report) => jp_report(index, anchor),
        (Language::Japanese, DeliverableKind::General) => jp_general(index, total, anchor),
        (Language::English, DeliverableKind::Narrative) => en_narrative(index, anchor),
        (Language::English, DeliverableKind::Guide) => en_guide(index, total, anchor),
        (Language::English, DeliverableKind::Report) => en_report(index, anchor),
        (Language::English, DeliverableKind::General) => en_general(index, total, anchor),
    }
}

fn jp_narrative(index: usize, anchor: &str) -> String {
    format!(
        "### 具体化メモ\n- 視点: {}が要求「{anchor}」を場面内の選択として受け止めます。\n- 圧力: {}。\n- 固有の変化: {}を次節へ残します。",
        pick(JP_VIEWPOINTS, index, 1),
        pick(JP_PRESSURES, index, 2),
        pick(JP_SHIFTS, index, 3)
    )
}

fn jp_guide(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### 実行メモ\n- 入力: 要求「{anchor}」を{}として扱います。\n- 操作: 全{total}本の中で{}を具体化します。\n- 失敗時: {}を確認してから次へ進みます。",
        pick(JP_INPUTS, index, 1),
        pick(JP_OPERATIONS, index, 2),
        pick(JP_CHECKS, index, 3)
    )
}

fn jp_report(index: usize, anchor: &str) -> String {
    format!(
        "### 分析メモ\n- 観点: 要求「{anchor}」を{}から見ます。\n- 比較: {}との差分を明確にします。\n- 含意: {}を次の検討材料として残します。",
        pick(JP_LENSES, index, 1),
        pick(JP_COMPARISONS, index, 2),
        pick(JP_IMPLICATIONS, index, 3)
    )
}

fn jp_general(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### 成果物メモ\n- 担当: 要求「{anchor}」を全{total}本の中の{}として展開します。\n- 証拠: {}を本文に残します。\n- 接続: {}を後続ファイルへ渡します。",
        pick(JP_UNITS, index, 1),
        pick(JP_EVIDENCE, index, 2),
        pick(JP_HANDOFFS, index, 3)
    )
}

fn en_narrative(index: usize, anchor: &str) -> String {
    format!(
        "### Concrete Commitments\n- Viewpoint: {} treats \"{anchor}\" as an in-scene choice.\n- Pressure: {}.\n- Change: carry {} into the next segment.",
        pick(EN_VIEWPOINTS, index, 1),
        pick(EN_PRESSURES, index, 2),
        pick(EN_SHIFTS, index, 3)
    )
}

fn en_guide(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### Execution Commitments\n- Input: Treat \"{anchor}\" as {}.\n- Action: In the {total}-file sequence, make {} concrete.\n- Failure case: Check {} before moving on.",
        pick(EN_INPUTS, index, 1),
        pick(EN_OPERATIONS, index, 2),
        pick(EN_CHECKS, index, 3)
    )
}

fn en_report(index: usize, anchor: &str) -> String {
    format!(
        "### Analysis Commitments\n- Lens: Read \"{anchor}\" through {}.\n- Comparison: Name the difference from {}.\n- Implication: Leave {} for the next review point.",
        pick(EN_LENSES, index, 1),
        pick(EN_COMPARISONS, index, 2),
        pick(EN_IMPLICATIONS, index, 3)
    )
}

fn en_general(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### Deliverable Commitments\n- Ownership: Develop \"{anchor}\" as {} in the {total}-file sequence.\n- Evidence: Leave {} in the text.\n- Handoff: Pass {} to later files.",
        pick(EN_UNITS, index, 1),
        pick(EN_EVIDENCE, index, 2),
        pick(EN_HANDOFFS, index, 3)
    )
}

fn pick(values: &[&'static str], index: usize, stride: usize) -> &'static str {
    if values.is_empty() {
        return "";
    }
    let position = index.saturating_sub(1).saturating_div(stride.max(1));
    let slot = position
        .saturating_mul(5)
        .saturating_add(position / values.len())
        % values.len();
    values[slot]
}

const JP_VIEWPOINTS: &[&str] = &[
    "記録係の視点",
    "現場の当事者",
    "外部から来た協力者",
    "疑念を抱く管理者",
    "次世代を担う人物",
    "失われた約束を知る人物",
];
const JP_PRESSURES: &[&str] = &[
    "時間制限が判断を狭める",
    "信頼できる情報が食い違う",
    "小さな成功が新しい負債を生む",
    "安全策と前進が衝突する",
    "隠していた前提が表に出る",
    "助けを求めるほど責任が重くなる",
];
const JP_SHIFTS: &[&str] = &["理解の反転", "関係の変化", "場所の意味", "約束の重さ"];
const JP_INPUTS: &[&str] = &["前提", "制約", "作業単位", "確認対象"];
const JP_OPERATIONS: &[&str] = &["準備", "判断", "実行", "検証", "調整"];
const JP_CHECKS: &[&str] = &["入力の完全性", "手順の再現性", "観察結果", "戻り地点"];
const JP_LENSES: &[&str] = &["利用者価値", "構造", "リスク", "運用", "長期影響"];
const JP_COMPARISONS: &[&str] = &["前節の整理", "代替案", "既知の制約", "未検証の仮説"];
const JP_IMPLICATIONS: &[&str] = &["次の疑問", "判断材料", "未解決点", "検証条件"];
const JP_UNITS: &[&str] = &["独立した単位", "接続点", "詳細化", "検証対象"];
const JP_EVIDENCE: &[&str] = &["目的", "前提", "具体例", "完了条件"];
const JP_HANDOFFS: &[&str] = &["用語", "未解決点", "判断", "次の作業"];

const EN_VIEWPOINTS: &[&str] = &[
    "the recorder",
    "the person on the ground",
    "an outside ally",
    "a skeptical steward",
    "the next generation",
    "the keeper of a lost promise",
];
const EN_PRESSURES: &[&str] = &[
    "time narrows the available choice",
    "trusted facts disagree",
    "a small success creates new debt",
    "safety and progress collide",
    "a hidden premise becomes visible",
    "asking for help increases responsibility",
];
const EN_SHIFTS: &[&str] = &[
    "a reversed understanding",
    "a changed relationship",
    "a changed place",
    "a heavier promise",
];
const EN_INPUTS: &[&str] = &["a premise", "a constraint", "a work unit", "a check target"];
const EN_OPERATIONS: &[&str] = &["setup", "judgment", "execution", "validation", "adjustment"];
const EN_CHECKS: &[&str] = &[
    "input completeness",
    "repeatability",
    "observed result",
    "the return point",
];
const EN_LENSES: &[&str] = &[
    "user value",
    "structure",
    "risk",
    "operation",
    "long-term effect",
];
const EN_COMPARISONS: &[&str] = &[
    "the prior section",
    "an alternative",
    "a known constraint",
    "an untested assumption",
];
const EN_IMPLICATIONS: &[&str] = &[
    "the next question",
    "decision material",
    "an open issue",
    "a validation condition",
];
const EN_UNITS: &[&str] = &[
    "a standalone unit",
    "a connection point",
    "a detail expansion",
    "a validation target",
];
const EN_EVIDENCE: &[&str] = &[
    "purpose",
    "premise",
    "a concrete example",
    "completion criteria",
];
const EN_HANDOFFS: &[&str] = &["terms", "open questions", "decisions", "next work"];
