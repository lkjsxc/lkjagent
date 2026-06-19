use crate::count_profile::{DeliverableKind, Language};
use crate::count_profile_variation::{variation, Variation};

pub(crate) fn passage_block(
    language: Language,
    kind: DeliverableKind,
    index: usize,
    total: usize,
    anchor: &str,
) -> String {
    match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => jp_narrative(index, total, anchor),
        (Language::Japanese, DeliverableKind::Guide) => jp_guide(index, total, anchor),
        (Language::Japanese, DeliverableKind::Report) => jp_report(index, total, anchor),
        (Language::Japanese, DeliverableKind::General) => jp_general(index, total, anchor),
        (Language::English, DeliverableKind::Narrative) => en_narrative(index, total, anchor),
        (Language::English, DeliverableKind::Guide) => en_guide(index, total, anchor),
        (Language::English, DeliverableKind::Report) => en_report(index, total, anchor),
        (Language::English, DeliverableKind::General) => en_general(index, total, anchor),
    }
}

fn jp_narrative(index: usize, total: usize, anchor: &str) -> String {
    let detail = jp_detail(variation(
        Language::Japanese,
        DeliverableKind::Narrative,
        index,
    ));
    format!(
        "{detail}\n\n### 本文断片\n第{index}節では、要求「{anchor}」を、人物が選び直しを迫られる場面として描きます。{detail_sentence}。\n\n場面の終わりでは、解決ではなく次へ進む理由を残します。全{total}本の流れの中で、この節は小さな判断を確定し、その判断が次の節の圧力になるように結びます。",
        detail_sentence = jp_sentence(variation(Language::Japanese, DeliverableKind::Narrative, index))
    )
}

fn jp_guide(index: usize, total: usize, anchor: &str) -> String {
    let detail = jp_detail(variation(Language::Japanese, DeliverableKind::Guide, index));
    format!(
        "{detail}\n\n### 本文断片\n第{index}節では、要求「{anchor}」を、読者がそのまま実行できる単位へ落とします。{detail_sentence}。\n\n全{total}本の手順の中で、この節の完了条件は、次の節へ渡せる状態を名前で確認できることです。失敗した場合は、観察結果と戻り地点を記録してから再実行します。",
        detail_sentence = jp_sentence(variation(Language::Japanese, DeliverableKind::Guide, index))
    )
}

fn jp_report(index: usize, total: usize, anchor: &str) -> String {
    let detail = jp_detail(variation(
        Language::Japanese,
        DeliverableKind::Report,
        index,
    ));
    format!(
        "{detail}\n\n### 本文断片\n第{index}節では、要求「{anchor}」を一つの論点として扱い、前提、根拠、判断、残る疑問を分けて記述します。{detail_sentence}。\n\n全{total}本の報告の中で、この節は次の比較や検証へ渡す材料を持ちます。結論は暫定でよく、未確認の条件を明示することで後続の節が同じ論点を繰り返さないようにします。",
        detail_sentence = jp_sentence(variation(Language::Japanese, DeliverableKind::Report, index))
    )
}

fn jp_general(index: usize, total: usize, anchor: &str) -> String {
    let detail = jp_detail(variation(
        Language::Japanese,
        DeliverableKind::General,
        index,
    ));
    format!(
        "{detail}\n\n### 本文断片\n第{index}節では、要求「{anchor}」を独立して読める成果物単位にします。{detail_sentence}。\n\n全{total}本の構成の中で、この節が渡すものは、用語、判断、未解決点のいずれかです。後続ファイルが迷わず続けられるように、最後に次の接続先を短く示します。",
        detail_sentence = jp_sentence(variation(Language::Japanese, DeliverableKind::General, index))
    )
}

fn en_narrative(index: usize, total: usize, anchor: &str) -> String {
    let detail = en_detail(variation(
        Language::English,
        DeliverableKind::Narrative,
        index,
    ));
    format!(
        "{detail}\n\n### Draft Passage\nSegment {index} turns \"{anchor}\" into a scene where someone must choose again. {detail_sentence}.\n\nThe segment closes with a reason to continue rather than a full resolution. Inside the {total}-file sequence, this part fixes one decision and lets that decision become pressure for the next segment.",
        detail_sentence = en_sentence(variation(Language::English, DeliverableKind::Narrative, index))
    )
}

fn en_guide(index: usize, total: usize, anchor: &str) -> String {
    let detail = en_detail(variation(Language::English, DeliverableKind::Guide, index));
    format!(
        "{detail}\n\n### Draft Passage\nSegment {index} turns \"{anchor}\" into an executable unit. {detail_sentence}.\n\nInside the {total}-file guide, this segment is complete when the next segment can receive a named state. If the result is wrong, record the observation and return point before repeating the step.",
        detail_sentence = en_sentence(variation(Language::English, DeliverableKind::Guide, index))
    )
}

fn en_report(index: usize, total: usize, anchor: &str) -> String {
    let detail = en_detail(variation(Language::English, DeliverableKind::Report, index));
    format!(
        "{detail}\n\n### Draft Passage\nSegment {index} treats \"{anchor}\" as one report point and separates premise, evidence, judgment, and remaining question. {detail_sentence}.\n\nInside the {total}-file report, this segment hands off material for comparison or validation. The conclusion can stay provisional as long as the next unknown is explicit.",
        detail_sentence = en_sentence(variation(Language::English, DeliverableKind::Report, index))
    )
}

fn en_general(index: usize, total: usize, anchor: &str) -> String {
    let detail = en_detail(variation(
        Language::English,
        DeliverableKind::General,
        index,
    ));
    format!(
        "{detail}\n\n### Draft Passage\nSegment {index} turns \"{anchor}\" into a standalone deliverable unit. {detail_sentence}.\n\nInside the {total}-file structure, this segment hands off a term, decision, or open issue. It ends by naming the next connection so later files can continue without guessing.",
        detail_sentence = en_sentence(variation(Language::English, DeliverableKind::General, index))
    )
}

fn en_detail(detail: Variation) -> String {
    format!(
        "### Specific Detail\n- Focus: {}\n- Context: {}\n- Action: {}\n- Result: {}",
        detail.focus, detail.context, detail.action, detail.result
    )
}

fn jp_detail(detail: Variation) -> String {
    format!(
        "### 固有要素\n- 焦点: {}\n- 文脈: {}\n- 動作: {}\n- 結果: {}",
        detail.focus, detail.context, detail.action, detail.result
    )
}

fn en_sentence(detail: Variation) -> String {
    format!(
        "This part follows {} inside {}; the segment must {}; visible result: {}",
        detail.focus, detail.context, detail.action, detail.result
    )
}

fn jp_sentence(detail: Variation) -> String {
    format!(
        "{}は{}で「{}」を実行し、{}",
        detail.focus, detail.context, detail.action, detail.result
    )
}
