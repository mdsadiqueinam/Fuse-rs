use lazy_static::lazy_static;
use regex::Regex;
use crate::helpers::str_ext::StrExt;
use crate::search::search::SearchResult;
use super::base_match::{BaseMatch};

/// Inverse exact match implementation
/// Token: '!file'
/// Match type: inverse-exact-match
/// Description: Items that do not include `file`
pub struct InverseExactMatch {
    pattern: String,
}

impl InverseExactMatch {
    /// Create a new InverseExactMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "inverse-exact"
    }
}

impl BaseMatch for InverseExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^!"(.*)"$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^!(.*)$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let index = text.index_of(self.pattern(), None);
        let is_match = index.is_none();
        
        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: Some(vec![(0, text.len().saturating_sub(1))]),
        }
    }
}
