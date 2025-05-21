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
pub type MatcherBox<'a> = Arc<dyn BaseMatch + Send + Sync + 'a>;

pub type MatcherFactory<'a> = fn(&str, &FuseOptions<'a>) -> Option<MatcherBox<'a>>;



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
            let query: Vec<String> = SPACE_RE
                .split(item.trim())
                .filter(|item| !item.is_empty() && !item.trim().is_empty())
                .map(|s| s.to_string())
                .collect();

            let mut results: Vec<MatcherBox<'a>> = Vec::new();

            for query_item in query {
                // 1. Handle multiple query match (i.e, ones that are quoted, like `"hello world"`)
                let mut found = false;
                for idx in 0..8 {
                    let token = match idx {
                        0 => ExactMatch::is_multi_match(&query_item),
                        1 => IncludeMatch::is_multi_match(&query_item),
                        2 => PrefixExactMatch::is_multi_match(&query_item),
                        3 => InversePrefixExactMatch::is_multi_match(&query_item),
                        4 => InverseSuffixExactMatch::is_multi_match(&query_item),
                        5 => SuffixExactMatch::is_multi_match(&query_item),
                        6 => InverseExactMatch::is_multi_match(&query_item),
                        7 => FuzzyMatch::is_multi_match(&query_item),
                        _ => None,
                    };
                    if let Some(token) = token {
                        match idx {
                            0 => results.push(Arc::new(ExactMatch::new(token))),
                            1 => results.push(Arc::new(IncludeMatch::new(token))),
                            2 => results.push(Arc::new(PrefixExactMatch::new(token))),
                            3 => results.push(Arc::new(InversePrefixExactMatch::new(token))),
                            4 => results.push(Arc::new(InverseSuffixExactMatch::new(token))),
                            5 => results.push(Arc::new(SuffixExactMatch::new(token))),
                            6 => results.push(Arc::new(InverseExactMatch::new(token))),
                            7 => results.push(Arc::new(FuzzyMatch::new(token, std::borrow::Cow::Owned(options.clone())))),
                            _ => {},
                        }
                        found = true;
                        break;
                    }
                }
                if found {
                    continue;
                }

                // 2. Handle single query matches (i.e, ones that are *not* quoted)
                let mut matched = false;
                for idx in 0..8 {
                    let token = match idx {
                        0 => ExactMatch::is_single_match(&query_item),
                        1 => IncludeMatch::is_single_match(&query_item),
                        2 => PrefixExactMatch::is_single_match(&query_item),
                        3 => InversePrefixExactMatch::is_single_match(&query_item),
                        4 => InverseSuffixExactMatch::is_single_match(&query_item),
                        5 => SuffixExactMatch::is_single_match(&query_item),
                        6 => InverseExactMatch::is_single_match(&query_item),
                        7 => FuzzyMatch::is_single_match(&query_item),
                        _ => None,
                    };
                    if let Some(token) = token {
                        match idx {
                            0 => results.push(Arc::new(ExactMatch::new(token))),
                            1 => results.push(Arc::new(IncludeMatch::new(token))),
                            2 => results.push(Arc::new(PrefixExactMatch::new(token))),
                            3 => results.push(Arc::new(InversePrefixExactMatch::new(token))),
                            4 => results.push(Arc::new(InverseSuffixExactMatch::new(token))),
                            5 => results.push(Arc::new(SuffixExactMatch::new(token))),
                            6 => results.push(Arc::new(InverseExactMatch::new(token))),
                            7 => results.push(Arc::new(FuzzyMatch::new(token, std::borrow::Cow::Owned(options.clone())))),
                            _ => {},
                        }
                        matched = true;
                        break;
                    }
                }
                if matched {
                    continue;
                }
            }

            results
        })
        .collect()
}
