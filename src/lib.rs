//! # i18n-currency
//!
//! A library to create a

#![allow(non_camel_case_types)]

pub enum Currency {
	EUR,
	USD,
}

pub enum Language {
}

/// Translates the currency into the target language.
/// If there is no translation, returns `None`.
pub fn translate_currency(currency: Currency, language: Language) -> Option<&'static str> {
    use self::Language::*;
    match language {
    }
}
