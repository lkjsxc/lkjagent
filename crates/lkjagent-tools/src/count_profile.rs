use crate::count_profile_data::{EN_DESIGN_FOCUSES, JP_DESIGN_FOCUSES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DeliverableProfile {
    language: Language,
    kind: DeliverableKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    English,
    Japanese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeliverableKind {
    Narrative,
    Guide,
    Report,
    General,
}

impl DeliverableProfile {
    pub(crate) fn from_objective(objective: &str) -> Self {
        Self {
            language: detect_language(objective),
            kind: detect_kind(objective),
        }
    }

    pub(crate) fn root_readme(self, docs: usize, main: usize, objective: &str) -> String {
        match self.language {
            Language::Japanese => format!(
                "# 構造化成果物\n\n## 目的\n\n次の依頼に対応する複数ファイル成果物です。\n\n{objective}\n\n## 目次\n\n- [docs/](docs/): 設計、継続性、検証のための設計メモ {docs} 件。\n- [main/](main/): 順序付き本編ファイル {main} 件。\n\n## 検証\n\nこの成果物は指定ファイル数に合わせて生成されています。依頼が変わらない限り、合計ファイル数を保ってください。\n"
            ),
            Language::English => format!(
                "# Structured Output\n\n## Purpose\n\nA generated multi-file deliverable for this objective:\n\n{objective}\n\n## Table of Contents\n\n- [docs/](docs/): {docs} design files for planning, continuity, and verification.\n- [main/](main/): {main} ordered main content files.\n\n## Verification\n\nThe scaffold was generated as an exact counted deliverable. Keep the total file count stable unless the owner changes the target.\n"
            ),
        }
    }

    pub(crate) fn doc_page(self, index: usize, objective: &str) -> String {
        match self.language {
            Language::Japanese => format!(
                "# 設計メモ {index:03}\n\n## 焦点\n\n{}\n\n## 依頼文\n\n{objective}\n\n## 設計ノート\n\nこのメモは、成果物全体の順序、継続性、検証可能性を保つために使います。対応する本編ファイルへ反映すべき判断を記録します。\n",
                self.design_focus(index)
            ),
            Language::English => format!(
                "# Design Memo {index:03}\n\n## Focus\n\n{}\n\n## Objective Context\n\n{objective}\n\n## Notes\n\nUse this memo to keep the generated file set coherent, ordered, and verifiable. Record decisions that should shape the corresponding main content files.\n",
                self.design_focus(index)
            ),
        }
    }

    pub(crate) fn main_page(self, index: usize, total: usize, objective: &str) -> String {
        let arc = index.saturating_sub(1) / 10 + 1;
        let slot = index.saturating_sub(1) % 10 + 1;
        match self.language {
            Language::Japanese => format!(
                "# {}\n\n## 位置\n\n- 幕: {arc}\n- 節: {slot}\n\n## 依頼文\n\n{objective}\n\n## 本文\n\n{}\n\n## 継続メモ\n\n{}\n",
                self.main_title(index),
                self.body_text(index, total),
                self.handoff_text(index, total)
            ),
            Language::English => format!(
                "# {}\n\n## Position\n\n- Arc: {arc}\n- Segment: {slot}\n\n## Objective Context\n\n{objective}\n\n## Draft Content\n\n{}\n\n## Continuity Hand-Off\n\n{}\n",
                self.main_title(index),
                self.body_text(index, total),
                self.handoff_text(index, total)
            ),
        }
    }

    fn main_title(self, index: usize) -> String {
        match (self.language, self.kind) {
            (Language::Japanese, DeliverableKind::Narrative) => format!("本編 {index:03}"),
            (Language::Japanese, _) => format!("本文 {index:03}"),
            (Language::English, DeliverableKind::Narrative) => {
                format!("Narrative Segment {index:03}")
            }
            (Language::English, _) => format!("Main Content {index:03}"),
        }
    }

    fn design_focus(self, index: usize) -> &'static str {
        let slot = index.saturating_sub(1).min(11);
        let (focuses, fallback) = match self.language {
            Language::Japanese => (&JP_DESIGN_FOCUSES, "補足設計メモ"),
            Language::English => (&EN_DESIGN_FOCUSES, "supplemental planning notes"),
        };
        focuses.get(slot).copied().unwrap_or(fallback)
    }

    fn body_text(self, index: usize, total: usize) -> String {
        match (self.language, self.kind) {
            (Language::Japanese, DeliverableKind::Narrative) => format!(
                "第{index}節は、依頼された大きな物語を一つの場面として進めます。状況、選択、変化を明確に置き、全{total}本の流れの中で次の節へ進む理由を残します。"
            ),
            (Language::Japanese, DeliverableKind::Guide) => format!(
                "この節では、依頼内容を実行可能な手順へ分解します。前提、操作、確認結果を明確にし、全{total}本の手順が重複せず積み上がるようにします。"
            ),
            (Language::Japanese, DeliverableKind::Report) => format!(
                "この節では、依頼内容に関する観点、根拠、示唆を整理します。全{total}本の報告が同じ論点を繰り返さず、次の分析へつながる形でまとめます。"
            ),
            (Language::Japanese, DeliverableKind::General) => format!(
                "この節では、依頼内容を独立して読める単位へ展開します。全{total}本の成果物が順序、目的、継続性を持つように、具体的な内容と次の接続点を残します。"
            ),
            (Language::English, DeliverableKind::Narrative) => format!(
                "Segment {index} advances the requested narrative as one concrete scene. It establishes a situation, a choice, and a visible change while leaving a reason for the next segment in the {total}-file sequence."
            ),
            (Language::English, DeliverableKind::Guide) => format!(
                "Segment {index} turns the objective into an actionable procedure. It states the premise, the operation, and the expected check so the {total}-file guide accumulates without repeating itself."
            ),
            (Language::English, DeliverableKind::Report) => format!(
                "Segment {index} records one perspective, its supporting basis, and its implication. It keeps the {total}-file report ordered and prepares the next analysis point."
            ),
            (Language::English, DeliverableKind::General) => format!(
                "Segment {index} develops the objective as a standalone unit. It keeps the {total}-file deliverable ordered, purposeful, and ready for later expansion."
            ),
        }
    }

    fn handoff_text(self, index: usize, total: usize) -> String {
        if index >= total {
            return match self.language {
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
        match self.language {
            Language::Japanese => {
                format!("- 前節までの用語、判断、未解決点を引き継ぎます。\n- 次の接続先: {next:03}")
            }
            Language::English => format!(
                "- Carry forward terms, decisions, and open questions from earlier parts.\n- Next segment: {next:03}"
            ),
        }
    }
}

fn detect_language(objective: &str) -> Language {
    if objective.chars().any(|ch| {
        ('\u{3040}'..='\u{30ff}').contains(&ch) || ('\u{4e00}'..='\u{9fff}').contains(&ch)
    }) {
        Language::Japanese
    } else {
        Language::English
    }
}

fn detect_kind(objective: &str) -> DeliverableKind {
    let lower = objective.to_lowercase();
    if contains_any(
        &lower,
        &["story", "novel", "fiction", "narrative", "物語", "小説"],
    ) {
        DeliverableKind::Narrative
    } else if contains_any(
        &lower,
        &["guide", "manual", "tutorial", "procedure", "手順", "説明書"],
    ) {
        DeliverableKind::Guide
    } else if contains_any(
        &lower,
        &["report", "analysis", "research", "調査", "分析", "報告"],
    ) {
        DeliverableKind::Report
    } else {
        DeliverableKind::General
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}
