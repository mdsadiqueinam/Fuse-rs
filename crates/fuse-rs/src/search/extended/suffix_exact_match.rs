use regex::Regex;
use std::sync::OnceLock;
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

static MULTI_REGEX: OnceLock<Regex> = OnceLock::new();
static SINGLE_REGEX: OnceLock<Regex> = OnceLock::new();

impl BaseMatch for SuffixExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        MULTI_REGEX.get_or_init(|| Regex::new(r#"^"(.*)"\$$"#).unwrap())
    }

    fn single_regex() -> &'static Regex {
        SINGLE_REGEX.get_or_init(|| Regex::new(r"^(.*)\$$").unwrap())
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
