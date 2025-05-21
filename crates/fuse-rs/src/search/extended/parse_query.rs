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
    static ref MULTI_MATCHERS: [fn(&str) -> Option<String>; 8] = [
        ExactMatch::is_multi_match,
        IncludeMatch::is_multi_match,
        PrefixExactMatch::is_multi_match,
        InversePrefixExactMatch::is_multi_match,
        InverseSuffixExactMatch::is_multi_match,
        SuffixExactMatch::is_multi_match,
        InverseExactMatch::is_multi_match,
        FuzzyMatch::is_multi_match,
    ];
    static ref SINGLE_MATCHERS: [fn(&str) -> Option<String>; 8] = [
        ExactMatch::is_single_match,
        IncludeMatch::is_single_match,
        PrefixExactMatch::is_single_match,
        InversePrefixExactMatch::is_single_match,
        InverseSuffixExactMatch::is_single_match,
        SuffixExactMatch::is_single_match,
        InverseExactMatch::is_single_match,
        FuzzyMatch::is_single_match,
    ];
}

const OR_TOKEN: &str = "|";



/// A trait object representing a matcher
pub type MatcherBox<'a> = Arc<dyn BaseMatch + Send + Sync + 'a>;

pub type MatcherFactory<'a> = fn(&str, &FuseOptions<'a>) -> Option<MatcherBox<'a>>;

/// Replace unsupported look-ahead regex with a manual split function
fn split_pattern(pattern: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for c in pattern.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                current.push(c);
            },
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
            },
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        result.push(current);
    }
    result
}

/// Parse a query string into a structured format for searching
///
/// Returns a 2D array representation of the query, for simpler parsing.
/// Example:
/// "^core go$ | rb$ | py$ xy$" => [["^core", "go$"], ["rb$"], ["py$", "xy$"]]
pub fn parse_query<'a>(pattern: &str, options: &FuseOptions<'a>) -> Vec<Vec<MatcherBox<'a>>> {
    pattern
        .split(OR_TOKEN)
        .map(|item| {
            let options = options.clone();
            let query: Vec<String> = split_pattern(item.trim());

            let mut results: Vec<MatcherBox<'a>> = Vec::new();

            for query_item in query {
                let constructors: [Box<dyn Fn(String, &FuseOptions<'a>) -> MatcherBox<'a>>; 8] = [
                    Box::new(|token, _| Arc::new(ExactMatch::new(token))),
                    Box::new(|token, _| Arc::new(IncludeMatch::new(token))),
                    Box::new(|token, _| Arc::new(PrefixExactMatch::new(token))),
                    Box::new(|token, _| Arc::new(InversePrefixExactMatch::new(token))),
                    Box::new(|token, _| Arc::new(InverseSuffixExactMatch::new(token))),
                    Box::new(|token, _| Arc::new(SuffixExactMatch::new(token))),
                    Box::new(|token, _| Arc::new(InverseExactMatch::new(token))),
                    Box::new(|token, opts| Arc::new(FuzzyMatch::new(token, std::borrow::Cow::Owned(opts.clone())))),
                ];

                // Multi-match
                let mut found = false;
                for (idx, matcher) in (*MULTI_MATCHERS).iter().enumerate() {
                    if let Some(token) = matcher(&query_item) {
                        results.push(constructors[idx](token, &options));
                        found = true;
                        break;
                    }
                }

                if found {
                    continue;
                }

                // Single-match
                for (idx, matcher) in (*SINGLE_MATCHERS).iter().enumerate() {
                    if let Some(token) = matcher(&query_item) {
                        results.push(constructors[idx](token, &options));
                        break;
                    }
                }
            }

            results
        })
        .collect()
}
