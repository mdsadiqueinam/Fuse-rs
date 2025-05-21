use lazy_static::lazy_static;
use regex::Regex;
use crate::helpers::str_ext::StrExt;
use crate::search::search_result::SearchResult;
use super::base_match::{BaseMatch};

/// Include match implementation
/// Token: ''file'
/// Match type: include-match
/// Description: Items that include `file`
pub struct IncludeMatch {
    pattern: String,
}

impl IncludeMatch {
    /// Create a new IncludeMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "include"
    }
}

impl BaseMatch for IncludeMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^'"(.*)"$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^'(.*)$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let mut location: usize = 0;
        let mut indices = Vec::new();
        let pattern_len = self.pattern.len();

        while let Some(index) = text.index_of(self.pattern(), Some(location)) {
            location = index + pattern_len;
            indices.push((index, location.saturating_sub(1)));
        }
        
        let is_match = !indices.is_empty();
        
        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: Some(indices),
        }
    }
}
