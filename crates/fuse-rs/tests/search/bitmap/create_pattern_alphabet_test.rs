use fuse_rs::search::bitmap::create_pattern_alphabet::create_pattern_alphabet;

#[test]
fn test_create_pattern_alphabet() {
    let pattern = "abc";
    let alphabet = create_pattern_alphabet(pattern);
    
    assert_eq!(alphabet.get(&'a'), Some(&4)); // 100 in binary (bit at position 2)
    assert_eq!(alphabet.get(&'b'), Some(&2)); // 010 in binary (bit at position 1)
    assert_eq!(alphabet.get(&'c'), Some(&1)); // 001 in binary (bit at position 0)
    assert_eq!(alphabet.get(&'d'), None);     // Not in pattern
}

#[test]
fn test_create_pattern_alphabet_with_repeating_chars() {
    let pattern = "hello";
    let alphabet = create_pattern_alphabet(pattern);
    
    // 'h' is at position 0, so bit at position (5-0-1) = 4 should be set
    assert_eq!(alphabet.get(&'h'), Some(&(1 << 4)));
    
    // 'e' is at position 1, so bit at position (5-1-1) = 3 should be set
    assert_eq!(alphabet.get(&'e'), Some(&(1 << 3)));
    
    // 'l' is at positions 2 and 3, so bits at positions (5-2-1) = 2 and (5-3-1) = 1 should be set
    assert_eq!(alphabet.get(&'l'), Some(&((1 << 2) | (1 << 1))));
    
    // 'o' is at position 4, so bit at position (5-4-1) = 0 should be set
    assert_eq!(alphabet.get(&'o'), Some(&(1 << 0)));
}