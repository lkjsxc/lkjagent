use crate::count_profile::Language;

pub(crate) fn local_verification(language: Language) -> &'static str {
    match language {
        Language::Japanese => {
            "## ローカル検証\n\n- 設計担当の状態を記録し、編集前に確認します。\n- 連続性台帳は前・現在・次のパスを示します。\n- 本文は固有要素、本文断片、要求との接続を含みます。\n- 継続メモは後続ファイルが続けられる状態を示します。\n\n"
        }
        Language::English => {
            "## Local Verification\n\n- Design-owner status is recorded and checked before edits.\n- Sequence ledger names previous, current, and next paths.\n- Draft content includes concrete detail, passage, and requirement link.\n- Handoff names the state later files can continue from.\n\n"
        }
    }
}
