use crate::count_profile::{DeliverableKind, Language};
use crate::count_profile_variation_en::english_variation;
use crate::count_profile_variation_jp::japanese_variation;

#[derive(Clone, Copy)]
pub(crate) struct Variation {
    pub(crate) focus: &'static str,
    pub(crate) context: &'static str,
    pub(crate) action: &'static str,
    pub(crate) result: &'static str,
}

pub(crate) fn variation(language: Language, kind: DeliverableKind, index: usize) -> Variation {
    match language {
        Language::English => english_variation(kind, index),
        Language::Japanese => japanese_variation(kind, index),
    }
}
