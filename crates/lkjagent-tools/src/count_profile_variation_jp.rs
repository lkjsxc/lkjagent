use crate::count_profile::DeliverableKind;
use crate::count_profile_variation::Variation;

pub(crate) fn japanese_variation(kind: DeliverableKind, index: usize) -> Variation {
    let values = match kind {
        DeliverableKind::Narrative => JP_NARRATIVE,
        DeliverableKind::Guide => JP_GUIDE,
        DeliverableKind::Report => JP_REPORT,
        DeliverableKind::General => JP_GENERAL,
    };
    if matches!(kind, DeliverableKind::Narrative) {
        return spread_variation(values, index);
    }
    values[index.saturating_sub(1) % values.len()]
}

fn spread_variation(values: &[Variation], index: usize) -> Variation {
    let position = index.saturating_sub(1);
    let slot = position
        .saturating_mul(5)
        .saturating_add(position / values.len())
        % values.len();
    values[slot]
}

const JP_NARRATIVE: &[Variation] = &[
    Variation {
        focus: "記録係",
        context: "資料庫の階段",
        action: "偽の時刻印を示す",
        result: "未完成の地図へ信頼が移る",
    },
    Variation {
        focus: "伝令",
        context: "冠水した駅台",
        action: "割れた印を隠す",
        result: "移動経路が公の危険になる",
    },
    Variation {
        focus: "管理者",
        context: "静かな機関室",
        action: "安易な修理を拒む",
        result: "安全と忠誠が分かれる",
    },
    Variation {
        focus: "見習い",
        context: "施錠された観測室",
        action: "失われた欄外注を読む",
        result: "約束に代償が生まれる",
    },
    Variation {
        focus: "目撃者",
        context: "市場の橋",
        action: "誤った受取人を名指す",
        result: "群衆の立場が変わる",
    },
    Variation {
        focus: "塔の守り手",
        context: "冷えた信号塔",
        action: "遅れた警告を送る",
        result: "助けが負債を連れて来る",
    },
];

const JP_GUIDE: &[Variation] = &[
    Variation {
        focus: "作業境界",
        context: "生の依頼",
        action: "入力を一つの作業単位に整える",
        result: "次工程が安定した前提を受け取る",
    },
    Variation {
        focus: "引き継ぎ包み",
        context: "途中成果",
        action: "編集前に必要状態を名付ける",
        result: "後続作業が推測なしに再開できる",
    },
    Variation {
        focus: "検証針",
        context: "候補結果",
        action: "主張を証明する最小確認を走らせる",
        result: "失敗時の戻り地点が残る",
    },
    Variation {
        focus: "修復分岐",
        context: "観測された不一致",
        action: "境界を一つずつ変える",
        result: "修正が監査可能なまま残る",
    },
    Variation {
        focus: "作業者メモ",
        context: "曖昧な指示",
        action: "採用した解釈を記録する",
        result: "後続手順が同じ契約を継承する",
    },
    Variation {
        focus: "完了記録",
        context: "完了単位",
        action: "移動前に証拠を保存する",
        result: "完了状態が復元できる",
    },
];

const JP_REPORT: &[Variation] = &[
    Variation {
        focus: "利用証拠",
        context: "現在の挙動",
        action: "観察事実と推論を分ける",
        result: "主張に見える根拠が残る",
    },
    Variation {
        focus: "リスク証拠",
        context: "失敗形態",
        action: "発火条件と影響範囲を名付ける",
        result: "判断に防護線が付く",
    },
    Variation {
        focus: "比較証拠",
        context: "代替経路",
        action: "退けた交換条件を書く",
        result: "採用理由が読める",
    },
    Variation {
        focus: "費用証拠",
        context: "運用負荷",
        action: "費用を測定可能な圧力へ結び付ける",
        result: "優先順位を付けられる",
    },
    Variation {
        focus: "品質証拠",
        context: "受入条件",
        action: "証明を具体物へつなげる",
        result: "結論が検査可能になる",
    },
    Variation {
        focus: "未知証拠",
        context: "未解決の問い",
        action: "未証明の範囲を区切る",
        result: "次の検討対象が定まる",
    },
];

const JP_GENERAL: &[Variation] = &[
    Variation {
        focus: "範囲片",
        context: "依頼目的",
        action: "最小の完了単位を定義する",
        result: "隣接ファイルとの重複が減る",
    },
    Variation {
        focus: "具体例片",
        context: "抽象要求",
        action: "一つの具体例に落とす",
        result: "主張を点検できる",
    },
    Variation {
        focus: "制約片",
        context: "利用可能な限界",
        action: "出力を形作る規則を書く",
        result: "後続作業が境界を守る",
    },
    Variation {
        focus: "判断片",
        context: "分岐選択",
        action: "一つの道を選んで理由を示す",
        result: "連続性が保たれる",
    },
    Variation {
        focus: "検証片",
        context: "完了主張",
        action: "証明する具体物を名付ける",
        result: "単位ごとに検査できる",
    },
    Variation {
        focus: "引継片",
        context: "残作業",
        action: "次の用語と未解決点を残す",
        result: "継続が構造化される",
    },
];
