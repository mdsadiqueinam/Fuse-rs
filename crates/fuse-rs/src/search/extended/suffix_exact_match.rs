use lazy_static::lazy_static;
use regex::Regex;
use crate::search::search::SearchResult;
use super::base_match::{BaseMatch};

/// Suffix exact match implementation
/// Token: 'file$'
/// Match type: suffix-exact-match
/// Description: Items that end with `file`
pub struct SuffixExactMatch {
    pattern: String,
}

impl SuffixExactMatch {
    /// Create a new SuffixExactMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "suffix-exact"
    }
}

impl BaseMatch for SuffixExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^"(.*)"\$$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^(.*)\$$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let is_match = text.ends_with(&self.pattern);
        let indices = if is_match {
            if self.pattern.is_empty() {
                if text.is_empty() {
                    vec![(0, 0)]
                } else {
                    let last = text.len() - 1;
                    vec![(last, last)]
                }
            } else {
                let start = text.len().saturating_sub(self.pattern.len());
                let end = text.len().saturating_sub(1);
                vec![(start, end)]
            }
        } else {
            vec![]
        };
        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: if is_match { Some(indices) } else { None },
        }
    }
}
