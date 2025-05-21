/// Creates a pattern alphabet for bitap algorithm.
///
/// This function generates a bitmap mask for each character in the pattern.
/// Each mask is a 32-bit integer where each bit represents the presence of
/// the character at a specific position in the pattern.
///
/// # Arguments
///
/// * `pattern` - The search pattern to create an alphabet for
///
/// # Returns
///
/// A HashMap where keys are characters and values are bitmasks
pub fn create_pattern_alphabet(pattern: &str) -> std::collections::HashMap<char, u32> {
    let mut mask = std::collections::HashMap::new();
    let len = pattern.len();

    // Create a bit mask for each character in the pattern
    for (i, c) in pattern.chars().enumerate() {
        // Get the existing mask for this character, or 0 if not found
        let entry = mask.entry(c).or_insert(0);

        // Set the bit corresponding to the position in the pattern
        // For example, if the character is at position 0 in a 3-character pattern,
        // we set the bit at position 2 (len - i - 1 = 3 - 0 - 1 = 2)
        *entry |= 1 << (len - i - 1);
    }

    mask
}

