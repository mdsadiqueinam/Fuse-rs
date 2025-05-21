use regex::Regex;
use crate::search::search_result::SearchResult;

/// Base trait for all match types
pub trait BaseMatch {
    /// Get the pattern being matched
    fn pattern(&self) -> &str;

    /// Check if the pattern is a multi-match
    fn is_multi_match(pattern: &str) -> Option<String> where Self: Sized {
        get_match(pattern, Self::multi_regex())
    }

    /// Check if the pattern is a single match
    fn is_single_match(pattern: &str) -> Option<String> where Self: Sized {
        get_match(pattern, Self::single_regex())
    }

    /// Get the regex for multi-matches
    fn multi_regex() -> &'static Regex where Self: Sized;

    /// Get the regex for single matches
    fn single_regex() -> &'static Regex where Self: Sized;

    /// Get the match type
    fn get_type(&self) -> &'static str;

    /// Search for the pattern in the given text
    fn search(&self, text: &str) -> SearchResult {
        let is_match = text.starts_with(self.pattern());

        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: if is_match {
                if self.pattern().is_empty() {
                    Some(vec![(0, 0)])
                } else {
                    Some(vec![(0, self.pattern().len() - 1)])
                }
            } else {
                None
            },
        }
    }
}

/// Helper function to extract matches from a regex
fn get_match(pattern: &str, regex: &Regex) -> Option<String> {
    regex.captures(pattern).and_then(|caps| {
        caps.get(1).map(|m| m.as_str().to_string())
    })
}
