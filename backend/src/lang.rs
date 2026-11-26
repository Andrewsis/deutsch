use serde::Deserialize;

/// Translation languages supported by `verb_translations.lang` (ISO 639-1).
pub const SUPPORTED: &[&str] = &["uk", "en"];
pub const DEFAULT: &str = "uk";

/// `?lang=` query parameter. Unknown or missing values fall back to `DEFAULT`.
#[derive(Deserialize, Debug, Default)]
pub struct LangQuery {
    pub lang: Option<String>,
}

impl LangQuery {
    pub fn code(&self) -> &'static str {
        resolve(self.lang.as_deref())
    }
}

pub fn resolve(lang: Option<&str>) -> &'static str {
    lang.and_then(|l| SUPPORTED.iter().find(|s| s.eq_ignore_ascii_case(l)))
        .copied()
        .unwrap_or(DEFAULT)
}
