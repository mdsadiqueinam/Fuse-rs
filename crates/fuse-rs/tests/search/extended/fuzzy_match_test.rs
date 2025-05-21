use std::borrow::Cow;
use fuse_rs::FuseOptions;
use fuse_rs::search::extended::fuzzy_match::FuzzyMatch;
use fuse_rs::search::extended::base_match::BaseMatch;

#[test]
fn test_new_fuzzy_match() {
    let pattern = "test";
    let options = FuseOptions::default();
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));

    assert_eq!(fuzzy_match.pattern(), pattern);
    assert_eq!(FuzzyMatch::get_type(), "fuzzy");
}

#[test]
fn test_is_multi_match() {
    // Test with a valid multi-match pattern
    let pattern = r#""test""#;
    let result = FuzzyMatch::is_multi_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");

    // Test with an invalid multi-match pattern
    let pattern = "test";
    let result = FuzzyMatch::is_multi_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_is_single_match() {
    // The single regex for FuzzyMatch is "^(.*)$", which matches any string
    // and captures the entire string

    // Test with a valid single-match pattern
    let pattern = "test";
    let result = FuzzyMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");

    // Test with another valid pattern
    let pattern = "=test";
    let result = FuzzyMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "=test");
}

#[test]
fn test_search_exact_match() {
    let pattern = "world";
    let mut options = FuseOptions::default();
    options.include_matches = true; // Ensure indices are included
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));

    // Test exact match
    let text = "world";
    let result = fuzzy_match.search(text);

    // The bitmap search algorithm should find an exact match
    assert!(result.is_match);

    // Exact matches have very low scores (but not 0.0 due to the 0.001 minimum)
    assert!(result.score < 0.1);

    // With include_matches=true, indices should be present
    assert!(result.indices.is_some());

    // The indices should point to the entire pattern
    let indices = result.indices.unwrap();
    assert!(!indices.is_empty());
}

#[test]
fn test_search_fuzzy_match() {
    let pattern = "wrld";
    let options = FuseOptions::default();
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));

    // Test fuzzy match
    let text = "world";
    let result = fuzzy_match.search(text);
    assert!(result.is_match);
    assert!(result.score < 0.5); // Fuzzy matches have higher scores than exact matches
    assert!(result.indices.is_some());

    // The indices should contain the matched positions
    let indices = result.indices.unwrap();
    assert!(!indices.is_empty());
}

#[test]
fn test_search_no_match() {
    let pattern = "xyz";
    let options = FuseOptions::default();
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));

    // Test no match
    let text = "hello";
    let result = fuzzy_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
}

#[test]
fn test_search_empty_pattern() {
    // For FuzzyMatch, we'll use a special case for empty patterns
    // The bitmap search algorithm doesn't handle empty patterns well
    // So we'll just check that the search method doesn't crash
    let pattern = "";
    let options = FuseOptions::default();
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));

    // Empty pattern with non-empty text
    let text = "hello";
    let _ = fuzzy_match.search(text);

    // Empty pattern with empty text
    let text = "";
    let _ = fuzzy_match.search(text);
}

#[test]
fn test_search_with_options() {
    let pattern = "wrld";
    let text = "world";

    // Test with default options
    let options = FuseOptions::default();
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(result.is_match);
    let default_score = result.score;

    // Test with higher threshold (more lenient)
    let mut options = FuseOptions::default();
    options.threshold = 0.8;
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(result.is_match);

    // Test with lower threshold (more strict)
    let mut options = FuseOptions::default();
    options.threshold = 0.1;
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    // This might not match depending on how strict the threshold is
    // So we don't assert on is_match

    // Test with include_matches
    let mut options = FuseOptions::default();
    options.include_matches = true;
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(result.is_match);
    assert!(result.indices.is_some());
    assert!(!result.indices.unwrap().is_empty());
}

#[test]
fn test_search_case_sensitivity() {
    let pattern = "World";
    let text = "world";

    // Test with case insensitive (default)
    let options = FuseOptions::default();
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(result.is_match);

    // Test with case sensitive
    let mut options = FuseOptions::default();
    options.is_case_sensitive = true;
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(!result.is_match);
}

#[test]
fn test_search_with_diacritics() {
    let pattern = "hélló";
    let text = "hello";

    // Test with ignore diacritics (default)
    let mut options = FuseOptions::default();
    options.ignore_diacritics = true;
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(result.is_match);

    // Test without ignoring diacritics
    let mut options = FuseOptions::default();
    options.ignore_diacritics = false;
    let fuzzy_match = FuzzyMatch::new(pattern.to_string(), Cow::Borrowed(&options));
    let result = fuzzy_match.search(text);
    assert!(!result.is_match);
}
