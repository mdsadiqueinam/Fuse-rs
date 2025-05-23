use regex::Regex;
use crate::search::search::SearchResult;

/// Base trait for all match types
pub trait BaseMatch: Send + Sync {
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
    fn search(&self, text: &str) -> SearchResult;
}

/// Helper function to extract matches from a regex
fn get_match(pattern: &str, regex: &Regex) -> Option<String> {
    regex.captures(pattern).and_then(|caps| {
        caps.get(1).map(|m| m.as_str().to_string())
    })
}
