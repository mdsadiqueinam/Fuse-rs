use regex::Regex;
use std::sync::OnceLock;
use crate::helpers::str_ext::StrExt;
use crate::search::search::SearchResult;
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

static MULTI_REGEX: OnceLock<Regex> = OnceLock::new();
static SINGLE_REGEX: OnceLock<Regex> = OnceLock::new();

impl BaseMatch for IncludeMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        MULTI_REGEX.get_or_init(|| Regex::new(r#"^'"(.*)"$"#).unwrap())
    }

    fn single_regex() -> &'static Regex {
        SINGLE_REGEX.get_or_init(|| Regex::new(r"^'(.*)$").unwrap())
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
