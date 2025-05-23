use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::Arc;

use crate::FuseOptions;
use crate::helpers::str_ext::StrExt;
use crate::search::search::SearchResult;
use super::base_match::{BaseMatch};
use super::fuzzy_match::FuzzyMatch;
use super::include_match::IncludeMatch;
use super::parse_query::{parse_query, MatcherBox};

// Set of match types that can return multiple matches
lazy_static::lazy_static! {
    static ref MULTI_MATCH_SET: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert(FuzzyMatch::get_type());
        set.insert(IncludeMatch::get_type());
        set
    };
}

/// Extended search implementation
///
/// Command-like searching
/// ======================
///
/// Given multiple search terms delimited by spaces.e.g. `^jscript .python$ ruby !java`,
/// search in a given text.
///
/// Search syntax:
///
/// | Token       | Match type                 | Description                            |
/// | ----------- | -------------------------- | -------------------------------------- |
/// | `jscript`   | fuzzy-match                | Items that fuzzy match `jscript`       |
/// | `=scheme`   | exact-match                | Items that are `scheme`                |
/// | `'python`   | include-match              | Items that include `python`            |
/// | `!ruby`     | inverse-exact-match        | Items that do not include `ruby`       |
/// | `^java`     | prefix-exact-match         | Items that start with `java`           |
/// | `!^earlang` | inverse-prefix-exact-match | Items that do not start with `earlang` |
/// | `.js$`      | suffix-exact-match         | Items that end with `.js`              |
/// | `!.go$`     | inverse-suffix-exact-match | Items that do not end with `.go`       |
///
/// A single pipe character acts as an OR operator. For example, the following
/// query matches entries that start with `core` and end with either`go`, `rb`,
/// or`py`.
///
/// ```
/// let pattern = "^core go$ | rb$ | py$";
/// // This pattern would match entries that start with `core` and end with either `go`, `rb`, or `py`
/// ```
pub struct ExtendedSearch<'a> {
    pattern: String,
    options: Cow<'a, FuseOptions<'a>>,
    query: Option<Vec<Vec<MatcherBox<'a>>>>,
}

impl<'a> ExtendedSearch<'a> {
    /// Create a new ExtendedSearch
    pub fn new(pattern: String, options: Cow<'a, FuseOptions<'a>>) -> Self {
        let mut pattern = if options.is_case_sensitive {
            pattern
        } else {
            pattern.to_lowercase()
        };

        pattern = if options.ignore_diacritics {
            pattern.strip_diacritics()
        } else {
            pattern
        };

        let query = parse_query(&pattern, &options);

        Self {
            pattern,
            options,
            query: Some(query),
        }
    }

    /// Check if extended search should be used
    pub fn condition(_: &str, options: &FuseOptions) -> bool {
        options.use_extended_search
    }

    /// Search for the pattern in the given text
    pub fn search_in(&self, text: &str) -> SearchResult {
        let query = match &self.query {
            Some(q) => q,
            None => {
                return SearchResult {
                    is_match: false,
                    score: 1.0,
                    indices: None,
                };
            }
        };

        let mut text = if self.options.is_case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        text = if self.options.ignore_diacritics {
            text.strip_diacritics()
        } else {
            text
        };

        let mut all_indices = Vec::new();
        let mut num_matches: usize = 0;
        let mut total_score = 0.0;

        // ORs
        for searchers in query {
            
            //reset indices and num_matches
            all_indices.clear();
            num_matches = 0;
            
            // ANDs
            for searcher in searchers {
                let result = searcher.search(&text);

                if result.is_match {
                    num_matches += 1;
                    total_score += result.score;

                    if self.options.include_matches {
                        // Get the searcher's type
                        let searcher_type = searcher.get_type();

                        if let Some(indices) = result.indices {
                            if MULTI_MATCH_SET.contains(searcher_type) {
                                all_indices.extend(indices);
                            } else if !indices.is_empty() {
                                all_indices.push(indices[0]);
                            }
                        }
                    }
                } else {
                    total_score = 0.0;
                    num_matches = 0;
                    all_indices.clear();
                    break;
                }
            }

            // OR condition, so if TRUE, return
            if num_matches > 0 {
                return SearchResult {
                    is_match: true,
                    score: total_score / num_matches as f64,
                    indices: if self.options.include_matches { 
                        Some(all_indices)
                    } else {
                        None
                    },
                };
            }
        }

        // Nothing was matched
        SearchResult {
            is_match: false,
            score: 1.0,
            indices: None,
        }
    }
}
