// Tests for InverseExactMatch
use fuse_rs::search::extended::inverse_exact_match::InverseExactMatch;
use fuse_rs::search::extended::base_match::BaseMatch;

#[test]
fn test_new_inverse_exact_match() {
    let pattern = "test";
    let inv_match = InverseExactMatch::new(pattern.to_string());
    assert_eq!(inv_match.pattern(), pattern);
    assert_eq!(InverseExactMatch::get_type(), "inverse-exact");
}

#[test]
fn test_is_multi_match() {
    let pattern = r#"!"test""#;
    let result = InverseExactMatch::is_multi_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    let pattern = "test";
    let result = InverseExactMatch::is_multi_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_is_single_match() {
    let pattern = "!test";
    let result = InverseExactMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    let pattern = "test";
    let result = InverseExactMatch::is_single_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_search_inverse_exact_match() {
    let pattern = "world";
    let inv_match = InverseExactMatch::new(pattern.to_string());
    let text = "hello world";
    let result = inv_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    let text = "hello";
    let result = inv_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
}
