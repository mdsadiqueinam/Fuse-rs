// Tests for IncludeMatch
use fuse_rs::search::extended::include_match::IncludeMatch;
use fuse_rs::search::extended::base_match::BaseMatch;

#[test]
fn test_new_include_match() {
    let pattern = "test";
    let include_match = IncludeMatch::new(pattern.to_string());
    assert_eq!(include_match.pattern(), pattern);
    assert_eq!(IncludeMatch::get_type(), "include");
}

#[test]
fn test_is_multi_match() {
    // The multi_regex for IncludeMatch is ^'"(.*)"$
    let pattern = "'\"test\"";
    let result = IncludeMatch::is_multi_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    let pattern = "test";
    let result = IncludeMatch::is_multi_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_is_single_match() {
    let pattern = "'test";
    let result = IncludeMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    let pattern = "test";
    let result = IncludeMatch::is_single_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_search_include_match() {
    let pattern = "world";
    let include_match = IncludeMatch::new(pattern.to_string());
    let text = "hello world";
    let result = include_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(6, 10)]);
    let text = "hello";
    let result = include_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.unwrap().is_empty());
}

#[test]
fn test_search_multiple_matches() {
    let pattern = "l";
    let include_match = IncludeMatch::new(pattern.to_string());
    let text = "hello";
    let result = include_match.search(text);
    assert!(result.is_match);
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(2, 2), (3, 3)]);
}
