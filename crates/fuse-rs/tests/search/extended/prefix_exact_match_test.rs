use fuse_rs::search::extended::prefix_exact_match::PrefixExactMatch;
use fuse_rs::search::extended::base_match::BaseMatch;

#[test]
fn test_new_prefix_exact_match() {
    let pattern = "test";
    let prefix_match = PrefixExactMatch::new(pattern.to_string());
    
    assert_eq!(prefix_match.pattern(), pattern);
    assert_eq!(PrefixExactMatch::get_type(), "prefix-exact");
}

#[test]
fn test_is_multi_match() {
    // Test with a valid multi-match pattern
    let pattern = r#"^"test""#;
    let result = PrefixExactMatch::is_multi_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    
    // Test with an invalid multi-match pattern
    let pattern = "test";
    let result = PrefixExactMatch::is_multi_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_is_single_match() {
    // Test with a valid single-match pattern
    let pattern = "^test";
    let result = PrefixExactMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    
    // Test with an invalid single-match pattern
    let pattern = "test";
    let result = PrefixExactMatch::is_single_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_search_prefix_match() {
    let pattern = "hello";
    let prefix_match = PrefixExactMatch::new(pattern.to_string());
    
    // Test exact prefix match
    let text = "hello world";
    let result = prefix_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    
    // The indices should point to the "hello" part at the beginning
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(0, 4)]);
    
    // Test no match
    let text = "world";
    let result = prefix_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
    
    // Test partial match (should not match)
    let text = "hi hello world";
    let result = prefix_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
}

#[test]
fn test_search_empty_pattern() {
    let pattern = "";
    let prefix_match = PrefixExactMatch::new(pattern.to_string());
    
    // Empty pattern should match any text (as a prefix)
    let text = "hello";
    let result = prefix_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    
    // For non-empty text, indices should point to the first character
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(0, 0)]);
    
    // Empty pattern should match empty text
    let text = "";
    let result = prefix_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    
    // For empty text, indices should be (0, 0)
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(0, 0)]);
}

#[test]
fn test_search_case_sensitivity() {
    let pattern = "Hello";
    let prefix_match = PrefixExactMatch::new(pattern.to_string());
    
    // Case-sensitive match
    let text = "Hello World";
    let result = prefix_match.search(text);
    assert!(result.is_match);
    
    // Case-sensitive non-match
    let text = "hello world";
    let result = prefix_match.search(text);
    assert!(!result.is_match);
}