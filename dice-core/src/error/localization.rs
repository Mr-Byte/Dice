use crate::error::{codes::ErrorCode, Tags};
use fluent_templates::{fluent_bundle::FluentValue, loader::langid, LanguageIdentifier, Loader};
use std::collections::HashMap;

use super::context::ContextMsgId;

#[derive(Default, Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Locale(LanguageIdentifier);

impl Locale {
    pub const US_ENGLISH: Locale = Locale(langid!("en-US"));
}

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "../data/locales",
        fallback_language: "en-US",
        // TODO: Make this configurable (most likely via Cargo features).
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub fn localize_error_code(error_code: ErrorCode, tags: &Tags, locale: &Locale) -> String {
    let args = tags
        .0
        .iter()
        .map(|(key, value)| (key.to_string(), FluentValue::String(std::borrow::Cow::Borrowed(value))))
        .collect::<HashMap<_, _>>();

    LOCALES.lookup_with_args(&locale.0, error_code, &args)
}

pub fn localize_context_msg_id(context_msg_id: ContextMsgId, tags: &Tags, locale: &Locale) -> String {
    let args = tags
        .0
        .iter()
        .map(|(key, value)| (key.to_string(), FluentValue::String(std::borrow::Cow::Borrowed(value))))
        .collect::<HashMap<_, _>>();

    LOCALES.lookup_with_args(&locale.0, context_msg_id, &args)
}
