use crate::count_profile::Language;

pub(crate) fn restart_guide(
    language: Language,
    _docs: usize,
    main: usize,
    index_files: usize,
) -> String {
    if index_files > 0 && main > 0 {
        return match language {
            Language::Japanese => "## 再開ガイド\n\n- README.md から監査を始め、記録済み合計ファイル数を保ちます。\n- docs/README.md で設計範囲を確認し、main/README.md で本編台帳を確認します。\n- 任意の main/part-xxx.md を開いたら、設計担当と連続性台帳を確認してから編集します。\n".to_string(),
            Language::English => "## Restart Guide\n\n- Start audit at README.md and preserve the recorded total file count.\n- Use docs/README.md for design ranges and main/README.md for the part ledger.\n- When opening any main/part-xxx.md, follow its Design owner and Sequence Ledger before editing.\n".to_string(),
        };
    }
    match language {
        Language::Japanese => "## 再開ガイド\n\n- README.md から監査を始め、記録済み合計ファイル数を保ちます。\n- 本編ファイルはありません。docs/ または main/ を追加する前に目標ファイル数の変更を記録します。\n".to_string(),
        Language::English => "## Restart Guide\n\n- Start audit at README.md and preserve the recorded total file count.\n- No main files exist; record a changed target before adding docs/ or main/ files.\n".to_string(),
    }
}
