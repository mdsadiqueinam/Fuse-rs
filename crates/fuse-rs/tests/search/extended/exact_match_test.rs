use fuse_rs::search::extended::exact_match::ExactMatch;
use fuse_rs::search::extended::base_match::BaseMatch;

#[test]
fn test_new_exact_match() {
    let pattern = "test";
    let exact_match = ExactMatch::new(pattern.to_string());

    assert_eq!(exact_match.pattern(), pattern);
    assert_eq!(ExactMatch::get_type(), "exact");
}

#[test]
fn test_is_multi_match() {
    // Test with a valid multi-match pattern
    let pattern = r#"="test""#;
    let result = ExactMatch::is_multi_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");

    // Test with an invalid multi-match pattern
    let pattern = "test";
    let result = ExactMatch::is_multi_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_is_single_match() {
    // Test with a valid single-match pattern
    let pattern = "=test";
    let result = ExactMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");

    // Test with an invalid single-match pattern
    let pattern = "test";
    let result = ExactMatch::is_single_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_search_exact_match() {
    let pattern = "world";
    let exact_match = ExactMatch::new(pattern.to_string());

    // Test exact match
    let text = "world";
    let result = exact_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    assert_eq!(result.indices.unwrap(), vec![(0, pattern.len() - 1)]);

    // Test no match
    let text = "hello";
    let result = exact_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());

    // Test partial match (should not match)
    let text = "world of rust";
    let result = exact_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
}

#[test]
fn test_search_empty_pattern() {
    let pattern = "";
    let exact_match = ExactMatch::new(pattern.to_string());

    // Empty pattern should match empty text
    let text = "";
    let result = exact_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    assert_eq!(result.indices.unwrap(), vec![(0, 0)]);

    // Empty pattern should not match non-empty text
    let text = "hello";
    let result = exact_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
}

#[test]
fn test_search_case_sensitivity() {
    let pattern = "World";
    let exact_match = ExactMatch::new(pattern.to_string());

    // Case-sensitive match
    let text = "World";
    let result = exact_match.search(text);
    assert!(result.is_match);

    // Case-sensitive non-match
    let text = "world";
    let result = exact_match.search(text);
    assert!(!result.is_match);
}
