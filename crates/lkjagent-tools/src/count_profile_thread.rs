use crate::count_profile::{DeliverableKind, Language};

pub(crate) fn segment_brief(
    language: Language,
    kind: DeliverableKind,
    index: usize,
    total: usize,
    anchor: &str,
) -> String {
    let stage = stage_name(language, index, total);
    let focus = focus_label(language, kind, index);
    let proof = proof_label(language, kind, index);
    match language {
        Language::Japanese => format!(
            "## セグメント概要\n\n- ファイル役割: {stage} / {focus}\n- 局所目的: 要求「{anchor}」をこのファイル固有の成果に変換します。\n- 検証手がかり: {proof}\n\n"
        ),
        Language::English => format!(
            "## Segment Brief\n\n- File role: {stage} / {focus}\n- Local objective: Turn \"{anchor}\" into this file's distinct contribution.\n- Verification cue: {proof}\n\n"
        ),
    }
}

pub(crate) fn segment_role(
    language: Language,
    kind: DeliverableKind,
    index: usize,
    total: usize,
) -> String {
    format!(
        "{} / {}",
        stage_name(language, index, total),
        focus_label(language, kind, index)
    )
}

fn focus_label(language: Language, kind: DeliverableKind, index: usize) -> &'static str {
    match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => pick(JP_NARRATIVE, index),
        (Language::Japanese, DeliverableKind::Guide) => pick(JP_GUIDE, index),
        (Language::Japanese, DeliverableKind::Report) => pick(JP_REPORT, index),
        (Language::Japanese, DeliverableKind::General) => pick(JP_GENERAL, index),
        (Language::English, DeliverableKind::Narrative) => pick(EN_NARRATIVE, index),
        (Language::English, DeliverableKind::Guide) => pick(EN_GUIDE, index),
        (Language::English, DeliverableKind::Report) => pick(EN_REPORT, index),
        (Language::English, DeliverableKind::General) => pick(EN_GENERAL, index),
    }
}

fn proof_label(language: Language, kind: DeliverableKind, index: usize) -> &'static str {
    match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => pick(JP_NARRATIVE_PROOF, index),
        (Language::Japanese, DeliverableKind::Guide) => pick(JP_GUIDE_PROOF, index),
        (Language::Japanese, DeliverableKind::Report) => pick(JP_REPORT_PROOF, index),
        (Language::Japanese, DeliverableKind::General) => pick(JP_GENERAL_PROOF, index),
        (Language::English, DeliverableKind::Narrative) => pick(EN_NARRATIVE_PROOF, index),
        (Language::English, DeliverableKind::Guide) => pick(EN_GUIDE_PROOF, index),
        (Language::English, DeliverableKind::Report) => pick(EN_REPORT_PROOF, index),
        (Language::English, DeliverableKind::General) => pick(EN_GENERAL_PROOF, index),
    }
}

fn stage_name(language: Language, index: usize, total: usize) -> &'static str {
    let slot = index
        .saturating_sub(1)
        .saturating_mul(6)
        .checked_div(total.max(1))
        .unwrap_or(0)
        .min(5);
    match language {
        Language::Japanese => JP_STAGES[slot],
        Language::English => EN_STAGES[slot],
    }
}

fn pick(values: &[&'static str], index: usize) -> &'static str {
    values[index.saturating_sub(1) % values.len()]
}

const EN_STAGES: [&str; 6] = [
    "opening",
    "exploration",
    "rising conflict",
    "midpoint reversal",
    "crisis",
    "resolution",
];
const JP_STAGES: [&str; 6] = ["導入", "探索", "対立拡大", "中盤反転", "危機", "収束"];

const EN_NARRATIVE: &[&str] = &[
    "inciting pressure",
    "threshold crossing",
    "first bargain",
    "hidden cost",
    "public consequence",
    "private doubt",
    "reversal seed",
    "choice under pressure",
];
const EN_GUIDE: &[&str] = &[
    "setup boundary",
    "input shaping",
    "decision rule",
    "operation path",
    "validation step",
    "repair route",
    "handoff state",
    "repeatable close",
];
const EN_REPORT: &[&str] = &[
    "claim setup",
    "evidence slice",
    "comparison point",
    "risk note",
    "implication",
    "counterpoint",
    "decision effect",
    "next question",
];
const EN_GENERAL: &[&str] = &[
    "scope unit",
    "premise unit",
    "example unit",
    "constraint unit",
    "decision unit",
    "handoff unit",
    "check unit",
    "summary unit",
];

const JP_NARRATIVE: &[&str] = &[
    "発端の圧力",
    "境界を越える選択",
    "最初の取引",
    "隠れた代償",
    "公的な影響",
    "私的な迷い",
    "反転の種",
    "圧力下の決断",
];
const JP_GUIDE: &[&str] = &[
    "準備境界",
    "入力整形",
    "判断規則",
    "操作経路",
    "検証手順",
    "修復経路",
    "引き継ぎ状態",
    "再現可能な完了",
];
const JP_REPORT: &[&str] = &[
    "主張の前提",
    "証拠の切片",
    "比較点",
    "リスク注記",
    "含意",
    "反対観点",
    "判断への影響",
    "次の疑問",
];
const JP_GENERAL: &[&str] = &[
    "範囲単位",
    "前提単位",
    "具体例単位",
    "制約単位",
    "判断単位",
    "引き継ぎ単位",
    "確認単位",
    "要約単位",
];

const EN_NARRATIVE_PROOF: &[&str] = &["choice", "place detail", "reaction", "turn"];
const EN_GUIDE_PROOF: &[&str] = &["input", "action", "observed state", "fallback"];
const EN_REPORT_PROOF: &[&str] = &["premise", "evidence", "judgment", "unknown"];
const EN_GENERAL_PROOF: &[&str] = &["purpose", "premise", "example", "check"];
const JP_NARRATIVE_PROOF: &[&str] = &["選択", "場所の細部", "反応", "転換"];
const JP_GUIDE_PROOF: &[&str] = &["入力", "操作", "観察状態", "戻り地点"];
const JP_REPORT_PROOF: &[&str] = &["前提", "根拠", "判断", "未確認点"];
const JP_GENERAL_PROOF: &[&str] = &["目的", "前提", "具体例", "確認"];
