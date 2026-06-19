use crate::count_profile::{DeliverableKind, Language};

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
    format!(
        "### 本文断片\n第{index}節では、要求「{anchor}」を、人物が選び直しを迫られる場面として描きます。前節から渡された不安を受け取り、場所の細部、相手の反応、ひとつの短い行動を順に置いて、読者が変化を追えるようにします。\n\n場面の終わりでは、解決ではなく次へ進む理由を残します。全{total}本の流れの中で、この節は小さな判断を確定し、その判断が次の節の圧力になるように結びます。"
    )
}

fn jp_guide(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### 本文断片\n第{index}節では、要求「{anchor}」を、読者がそのまま実行できる単位へ落とします。入力、判断基準、操作、観察結果を順に並べ、作業者が迷う地点をひとつ先回りして明示します。\n\n全{total}本の手順の中で、この節の完了条件は、次の節へ渡せる状態を名前で確認できることです。失敗した場合は、観察結果と戻り地点を記録してから再実行します。"
    )
}

fn jp_report(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### 本文断片\n第{index}節では、要求「{anchor}」を一つの論点として扱い、前提、根拠、判断、残る疑問を分けて記述します。主張だけで終えず、どの観察がその判断を支えるのかを本文中に残します。\n\n全{total}本の報告の中で、この節は次の比較や検証へ渡す材料を持ちます。結論は暫定でよく、未確認の条件を明示することで後続の節が同じ論点を繰り返さないようにします。"
    )
}

fn jp_general(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### 本文断片\n第{index}節では、要求「{anchor}」を独立して読める成果物単位にします。目的、前提、具体例、確認方法をひとまとまりにし、隣接する節と重ならないように焦点を一つへ絞ります。\n\n全{total}本の構成の中で、この節が渡すものは、用語、判断、未解決点のいずれかです。後続ファイルが迷わず続けられるように、最後に次の接続先を短く示します。"
    )
}

fn en_narrative(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### Draft Passage\nSegment {index} turns \"{anchor}\" into a scene where someone must choose again. It carries forward the prior pressure, names one concrete place detail, gives another person a visible reaction, and leaves one small action on the page so the reader can track change.\n\nThe segment closes with a reason to continue rather than a full resolution. Inside the {total}-file sequence, this part fixes one decision and lets that decision become pressure for the next segment."
    )
}

fn en_guide(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### Draft Passage\nSegment {index} turns \"{anchor}\" into an executable unit. It states the input, the decision rule, the action to take, and the result to observe, then names one point where the operator might otherwise hesitate.\n\nInside the {total}-file guide, this segment is complete when the next segment can receive a named state. If the result is wrong, record the observation and return point before repeating the step."
    )
}

fn en_report(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### Draft Passage\nSegment {index} treats \"{anchor}\" as one report point and separates premise, evidence, judgment, and remaining question. It does not stop at a claim; it leaves the observation that supports the judgment inside the text.\n\nInside the {total}-file report, this segment hands off material for comparison or validation. The conclusion can stay provisional as long as the next unknown is explicit."
    )
}

fn en_general(index: usize, total: usize, anchor: &str) -> String {
    format!(
        "### Draft Passage\nSegment {index} turns \"{anchor}\" into a standalone deliverable unit. It groups purpose, premise, example, and check method, then keeps the focus narrow enough that nearby segments do not repeat it.\n\nInside the {total}-file structure, this segment hands off a term, decision, or open issue. It ends by naming the next connection so later files can continue without guessing."
    )
}
