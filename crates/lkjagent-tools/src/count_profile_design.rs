use crate::count_profile::{DeliverableKind, Language};
use crate::count_profile_paths::main_path;

pub(crate) fn design_text(
    language: Language,
    kind: DeliverableKind,
    index: usize,
    docs: usize,
    main: usize,
) -> String {
    let coverage = coverage_text(language, index, docs, main);
    let task = design_task(language, kind);
    match language {
        Language::Japanese => format!(
            "## 対象範囲\n\n{coverage}\n\n## 設計タスク\n\n{task}\n\n## 検証観点\n\n- 担当範囲の前後関係が連続していること。\n- 本編ファイルが依頼文、段階、次の接続先を保っていること。\n- 追加や削除で規模目安を崩さないこと。\n"
        ),
        Language::English => format!(
            "## Coverage\n\n{coverage}\n\n## Design Task\n\n{task}\n\n## Verification Checks\n\n- The covered range preserves sequence continuity.\n- Main files retain objective context, stage, and next-link information.\n- Revisions do not change the total file count.\n"
        ),
    }
}

fn coverage_text(language: Language, index: usize, docs: usize, main: usize) -> String {
    let Some((start, end)) = coverage_range(index, docs, main) else {
        return match language {
            Language::Japanese => {
                "本編ファイルはありません。全体構成だけを確認します。".to_string()
            }
            Language::English => {
                "No main files exist; review only the overall structure.".to_string()
            }
        };
    };
    match language {
        Language::Japanese if start == end => {
            format!("- 担当本編: {}", main_path(start))
        }
        Language::Japanese => {
            format!("- 担当本編: {} から {}", main_path(start), main_path(end))
        }
        Language::English if start == end => {
            format!("- Covered main file: {}", main_path(start))
        }
        Language::English => {
            format!(
                "- Covered main files: {} through {}",
                main_path(start),
                main_path(end)
            )
        }
    }
}

fn coverage_range(index: usize, docs: usize, main: usize) -> Option<(usize, usize)> {
    if docs == 0 || main == 0 {
        return None;
    }
    let slot = index.saturating_sub(1).min(docs.saturating_sub(1));
    let start = slot.saturating_mul(main) / docs + 1;
    let mut end = (slot.saturating_add(1)).saturating_mul(main) / docs;
    if end < start {
        end = start;
    }
    Some((start.min(main), end.min(main)))
}

fn design_task(language: Language, kind: DeliverableKind) -> &'static str {
    match (language, kind) {
        (Language::Japanese, DeliverableKind::Narrative) => {
            "担当範囲の場面目的、対立、転換点が前後の節と矛盾しないように確認します。"
        }
        (Language::Japanese, DeliverableKind::Guide) => {
            "担当範囲の手順が再現可能で、前後の準備や確認条件と矛盾しないように確認します。"
        }
        (Language::Japanese, DeliverableKind::Report) => {
            "担当範囲の論点、根拠、含意が重複せず、報告全体の流れを補強するように確認します。"
        }
        (Language::Japanese, DeliverableKind::General) => {
            "担当範囲の目的、前提、接続点が成果物全体の順序を保つように確認します。"
        }
        (Language::English, DeliverableKind::Narrative) => {
            "Check that scene purpose, conflict, and turn points align with neighboring segments."
        }
        (Language::English, DeliverableKind::Guide) => {
            "Check that procedures are repeatable and align with nearby setup and validation steps."
        }
        (Language::English, DeliverableKind::Report) => {
            "Check that perspective, basis, and implication strengthen the report without repetition."
        }
        (Language::English, DeliverableKind::General) => {
            "Check that purpose, premise, and linking points preserve the ordered deliverable."
        }
    }
}
