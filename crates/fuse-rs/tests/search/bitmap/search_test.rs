use fuse_rs::search::bitmap::create_pattern_alphabet::create_pattern_alphabet;
use fuse_rs::search::bitmap::search::search;
use fuse_rs::FuseOptions;

// Helper to create default options
fn default_options() -> FuseOptions<'static> {
    FuseOptions::new()
}

#[test]
fn test_exact_match() {
    let text = "hello world";
    let pattern = "world";
    let pattern_alphabet = create_pattern_alphabet(pattern);
    let options = default_options();
    
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    
    assert!(result.is_match);
    assert!(result.score < 0.1); // Exact matches have very low scores
}

#[test]
fn test_no_match() {
    let text = "hello world";
    let pattern = "xyz";
    let pattern_alphabet = create_pattern_alphabet(pattern);
    let options = default_options();
    
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    
    assert!(!result.is_match);
}

#[test]
fn test_fuzzy_match() {
    let text = "hello world";
    let pattern = "helo wrld"; // Fuzzy version with missing characters
    let pattern_alphabet = create_pattern_alphabet(pattern);
    let options = default_options();
    
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    
    // This should be a match because it's close enough
    assert!(result.is_match);
}

#[test]
fn test_include_matches() {
    let text = "hello world";
    let pattern = "world";
    let pattern_alphabet = create_pattern_alphabet(pattern);
    let mut options = default_options();
    options.include_matches = true;
    
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    
    assert!(result.is_match);
    assert!(result.indices.is_some());
    
    // Expected match indices for "world" in "hello world" (starting from positions 6-10)
    let expected_indices = vec![(2, 4), (6, 10)];
    assert_eq!(result.indices, Some(expected_indices));
}

#[test]
fn test_min_match_char_length() {
    let text = "hello world";
    let pattern = "world";
    let pattern_alphabet = create_pattern_alphabet(pattern);
    
    // Test with min length 3
    let mut options = default_options();
    options.min_match_char_length = 3;
    options.include_matches = true;
    
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    
    assert!(result.is_match);
    assert!(result.indices.is_some());
    
    // Test with min length that's too long
    let mut options = default_options();
    options.min_match_char_length = 10;
    options.include_matches = true;
    
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    
    // Should not match since we require 10 consecutive characters
    assert!(!result.is_match);
}

#[test]
fn test_threshold_effect() {
    let text = "hello world";
    let pattern = "helo wrld"; // Fuzzy match
    let pattern_alphabet = create_pattern_alphabet(pattern);
    
    // With default threshold
    let options = default_options();
    let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
    assert!(result.is_match);
    
    // With stricter threshold that should fail
    let mut strict_options = default_options();
    strict_options.threshold = 0.2;
    
    let result = search(text, pattern, &pattern_alphabet, &strict_options).unwrap();
    assert!(!result.is_match);
}