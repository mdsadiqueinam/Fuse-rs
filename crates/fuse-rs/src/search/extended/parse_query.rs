use std::sync::Arc;
use regex::Regex;
use lazy_static::lazy_static;

use crate::FuseOptions;

use super::base_match::BaseMatch;
use super::exact_match::ExactMatch;
use super::include_match::IncludeMatch;
use super::prefix_exact_match::PrefixExactMatch;
use super::inverse_prefix_exact_match::InversePrefixExactMatch;
use super::inverse_suffix_exact_match::InverseSuffixExactMatch;
use super::suffix_exact_match::SuffixExactMatch;
use super::inverse_exact_match::InverseExactMatch;
use super::fuzzy_match::FuzzyMatch;

// Regex to split by spaces, but keep anything in quotes together
lazy_static! {
    static ref SPACE_RE: Regex = Regex::new(r#" +(?=(?:[^"]*"[^"]*")*[^"]*$)"#).unwrap();
}

const OR_TOKEN: &str = "|";

/// A trait object representing a matcher
pub type MatcherBox = Arc<dyn BaseMatch + Send + Sync>;

/// Parse a query string into a structured format for searching
///
/// Returns a 2D array representation of the query, for simpler parsing.
/// Example:
/// "^core go$ | rb$ | py$ xy$" => [["^core", "go$"], ["rb$"], ["py$", "xy$"]]
pub fn parse_query(pattern: &str, options: FuseOptions) -> Vec<Vec<MatcherBox>> {
    pattern
        .split(OR_TOKEN)
        .map(|item| {
            let query: Vec<String> = item
                .trim()
                .split(SPACE_RE)
                .filter(|item| !item.is_empty() && !item.trim().is_empty())
                .map(|s| s.to_string())
                .collect();

            let mut results: Vec<MatcherBox> = Vec::new();
            
            for query_item in query {
                // 1. Handle multiple query match (i.e, ones that are quoted, like `"hello world"`)
                let mut found = false;
                
                // Try ExactMatch
                if let Some(token) = ExactMatch::is_multi_match(&query_item) {
                    results.push(Arc::new(ExactMatch::new(token)));
                    found = true;
                }
                
                // Try IncludeMatch
                if !found {
                    if let Some(token) = IncludeMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(IncludeMatch::new(token)));
                        found = true;
                    }
                }
                
                // Try PrefixExactMatch
                if !found {
                    if let Some(token) = PrefixExactMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(PrefixExactMatch::new(token)));
                        found = true;
                    }
                }
                
                // Try InversePrefixExactMatch
                if !found {
                    if let Some(token) = InversePrefixExactMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(InversePrefixExactMatch::new(token)));
                        found = true;
                    }
                }
                
                // Try InverseSuffixExactMatch
                if !found {
                    if let Some(token) = InverseSuffixExactMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(InverseSuffixExactMatch::new(token)));
                        found = true;
                    }
                }
                
                // Try SuffixExactMatch
                if !found {
                    if let Some(token) = SuffixExactMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(SuffixExactMatch::new(token)));
                        found = true;
                    }
                }
                
                // Try InverseExactMatch
                if !found {
                    if let Some(token) = InverseExactMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(InverseExactMatch::new(token)));
                        found = true;
                    }
                }
                
                // Try FuzzyMatch
                if !found {
                    if let Some(token) = FuzzyMatch::is_multi_match(&query_item) {
                        results.push(Arc::new(FuzzyMatch::new(token, options.clone())));
                        found = true;
                    }
                }
                
                if found {
                    continue;
                }
                
                // 2. Handle single query matches (i.e, ones that are *not* quoted)
                
                // Try ExactMatch
                if let Some(token) = ExactMatch::is_single_match(&query_item) {
                    results.push(Arc::new(ExactMatch::new(token)));
                    continue;
                }
                
                // Try IncludeMatch
                if let Some(token) = IncludeMatch::is_single_match(&query_item) {
                    results.push(Arc::new(IncludeMatch::new(token)));
                    continue;
                }
                
                // Try PrefixExactMatch
                if let Some(token) = PrefixExactMatch::is_single_match(&query_item) {
                    results.push(Arc::new(PrefixExactMatch::new(token)));
                    continue;
                }
                
                // Try InversePrefixExactMatch
                if let Some(token) = InversePrefixExactMatch::is_single_match(&query_item) {
                    results.push(Arc::new(InversePrefixExactMatch::new(token)));
                    continue;
                }
                
                // Try InverseSuffixExactMatch
                if let Some(token) = InverseSuffixExactMatch::is_single_match(&query_item) {
                    results.push(Arc::new(InverseSuffixExactMatch::new(token)));
                    continue;
                }
                
                // Try SuffixExactMatch
                if let Some(token) = SuffixExactMatch::is_single_match(&query_item) {
                    results.push(Arc::new(SuffixExactMatch::new(token)));
                    continue;
                }
                
                // Try InverseExactMatch
                if let Some(token) = InverseExactMatch::is_single_match(&query_item) {
                    results.push(Arc::new(InverseExactMatch::new(token)));
                    continue;
                }
                
                // Try FuzzyMatch (default)
                if let Some(token) = FuzzyMatch::is_single_match(&query_item) {
                    results.push(Arc::new(FuzzyMatch::new(token, options.clone())));
                }
            }
            
            results
        })
        .collect()
}