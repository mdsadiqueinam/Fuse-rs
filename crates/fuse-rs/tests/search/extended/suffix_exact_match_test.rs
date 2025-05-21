use fuse_rs::search::extended::suffix_exact_match::SuffixExactMatch;
use fuse_rs::search::extended::base_match::BaseMatch;

#[test]
fn test_new_suffix_exact_match() {
    let pattern = "test";
    let suffix_match = SuffixExactMatch::new(pattern.to_string());
    
    assert_eq!(suffix_match.pattern(), pattern);
    assert_eq!(SuffixExactMatch::get_type(), "suffix-exact");
}

#[test]
fn test_is_multi_match() {
    // Test with a valid multi-match pattern
    let pattern = r#""test"$"#;
    let result = SuffixExactMatch::is_multi_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    
    // Test with an invalid multi-match pattern
    let pattern = "test";
    let result = SuffixExactMatch::is_multi_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_is_single_match() {
    // Test with a valid single-match pattern
    let pattern = "test$";
    let result = SuffixExactMatch::is_single_match(pattern);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), "test");
    
    // Test with an invalid single-match pattern
    let pattern = "test";
    let result = SuffixExactMatch::is_single_match(pattern);
    assert!(result.is_none());
}

#[test]
fn test_search_suffix_match() {
    let pattern = "world";
    let suffix_match = SuffixExactMatch::new(pattern.to_string());
    
    // Test exact suffix match
    let text = "hello world";
    let result = suffix_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    
    // The indices should point to the "world" part at the end
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(6, 10)]);
    
    // Test no match
    let text = "hello";
    let result = suffix_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
    
    // Test partial match (should not match)
    let text = "world of rust";
    let result = suffix_match.search(text);
    assert!(!result.is_match);
    assert_eq!(result.score, 1.0);
    assert!(result.indices.is_none());
}

#[test]
fn test_search_empty_pattern() {
    let pattern = "";
    let suffix_match = SuffixExactMatch::new(pattern.to_string());
    
    // Empty pattern should match any text (as a suffix)
    let text = "hello";
    let result = suffix_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    
    // For non-empty text, indices should point to the last character
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(4, 4)]);
    
    // Empty pattern should match empty text
    let text = "";
    let result = suffix_match.search(text);
    assert!(result.is_match);
    assert_eq!(result.score, 0.0);
    assert!(result.indices.is_some());
    
    // For empty text, indices should be (0, 0)
    let indices = result.indices.unwrap();
    assert_eq!(indices, vec![(0, 0)]);
}

#[test]
fn test_search_case_sensitivity() {
    let pattern = "World";
    let suffix_match = SuffixExactMatch::new(pattern.to_string());
    
    // Case-sensitive match
    let text = "Hello World";
    let result = suffix_match.search(text);
    assert!(result.is_match);
    
    // Case-sensitive non-match
    let text = "Hello world";
    let result = suffix_match.search(text);
    assert!(!result.is_match);
}