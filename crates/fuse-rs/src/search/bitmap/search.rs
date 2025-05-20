use crate::FuseError;
use crate::FuseOptions;
use crate::helpers::str_ext::StrExt;
use std::collections::HashMap;

use super::compute_score::compute_score;
use super::constants::MAX_BITS;
use super::convert_mask_to_indices::convert_mask_to_indices;

pub struct SearchResult {
    /// Whether the pattern was found in the text
    pub is_match: bool,

    /// The match quality score (lower is better)
    pub score: f64,

    /// List of match position ranges as (start, end) tuples
    pub indices: Option<Vec<(usize, usize)>>,
}

pub fn search(
    text: &str,
    pattern: &str,
    pattern_alphabet: &HashMap<char, u64>,
    options: &FuseOptions,
) -> Result<SearchResult, FuseError> {
    // Check pattern length against maximum allowed
    if pattern.len() > MAX_BITS {
        return Err(FuseError::PatternLengthTooLarge(MAX_BITS));
    }

    let pattern_length = pattern.len();
    // Set starting location at beginning text and initialize the alphabet.
    let text_length = text.len();
    // Handle the case when location > text.length
    let expected_location = 0.max(options.location.min(text_length));
    // Highest score beyond which we give up.
    let mut current_threshold = options.threshold;
    // Is there a nearby exact match? (speedup)
    let mut best_location = expected_location;

    // Performance: only computer matches when the minMatchCharLength > 1
    // OR if includeMatches is true.
    let compute_matches = options.min_match_char_length > 1 || options.include_matches;
    // A mask of the matches, used for building the indices
    let mut match_mask = if compute_matches {
        vec![0; text_length]
    } else {
        Vec::new()
    };

    while let Some(index) = text.index_of(pattern, Some(best_location)) {
        let score = compute_score(pattern, 0, index, expected_location, options);

        current_threshold = score.min(current_threshold);
        best_location = index + pattern_length;

        if compute_matches {
            for i in 0..pattern_length {
                match_mask[index + i] = 1;
            }
        }
    }

    // reset the best location
    best_location = usize::MAX; // -1 equivalent in Rust

    let mut last_bit_arr: Vec<u64> = Vec::new();
    let mut final_score = 1.0;
    let mut bin_max = pattern_length + text_length;

    let mask = 1 << (pattern_length - 1);

    for i in 0..pattern_length {
        let mut bin_min = 0;
        let mut bin_mid = bin_max;

        while bin_min < bin_mid {
            let score = compute_score(
                pattern,
                i,
                expected_location + bin_mid,
                expected_location,
                options,
            );

            if score <= current_threshold {
                bin_min = bin_mid;
            } else {
                bin_max = bin_mid;
            }

            bin_mid = ((bin_max - bin_min) / 2) + bin_min;
        }

        // Use the result from this iteration as the maximum for the next.
        bin_max = bin_mid;
        
        let mut start = match expected_location.checked_sub(bin_mid) {
            Some(val) => val + 1,
            None => 1,
        };

        let finish = if options.find_all_matches {
            text_length
        } else {
            (expected_location + bin_mid).min(text_length) + pattern_length
        };
        
        let mut bit_arr = vec![0; finish + 2];

        bit_arr[finish + 1] = (1 << i) - 1;

        for j in (start..=finish).rev() {
            let current_location = j - 1;

            let char_match = match text.chars().nth(current_location) {
                Some(c) => pattern_alphabet.get(&c),
                None => None,
            };

            if compute_matches {
                // Speed up: quick bool to int conversion (i.e, `charMatch ? 1 : 0`)
                match_mask[current_location] = if char_match.is_some() { 1 } else { 0 };
            }

            // First pass: exact match
            bit_arr[j] = ((bit_arr[j + 1] << 1) | 1) & char_match.unwrap_or(&0);

            // Subsequent passes: fuzzy match
            if i > 0 {
                bit_arr[j] |=
                    ((last_bit_arr[j + 1] | last_bit_arr[j]) << 1) | 1 | last_bit_arr[j + 1]
            }

            if bit_arr[j] & mask != 0 {
                final_score =
                    compute_score(pattern, i, current_location, expected_location, options);

                // This match will almost certainly be better than any existing match.
                // But check anyway.
                if final_score <= current_threshold {
                    // Indeed it is
                    current_threshold = final_score;
                    best_location = current_location;

                    // Already passed `loc`, downhill from here on in.
                    if best_location <= expected_location {
                        break;
                    }

                    // When passing `bestLocation`, don't exceed our current distance from `expectedLocation`.
                    start = match expected_location.checked_sub(best_location) {
                        Some(val) => val * 2,
                        None => 1,
                    };
                }
            }
        }

        let score = compute_score(
            pattern,
            i + 1,
            expected_location,
            expected_location,
            options,
        );

        if score > current_threshold {
            break;
        }

        last_bit_arr = bit_arr;
    }

    let mut result = SearchResult {
        is_match: best_location != usize::MAX,
        // Count exact matches (those with a score of 0) to be "almost" exact
        score: (0.001f64).max(final_score),

        indices: None,
    };

    if compute_matches {
        let bool_match_mask = match_mask
            .iter()
            .map(|&x| x != 0)
            .collect::<Vec<_>>();
        let indicies = convert_mask_to_indices(&bool_match_mask, options.min_match_char_length);

        if indicies.is_empty() {
            result.is_match = false;
        } else if options.include_matches {
            result.indices = Some(indicies);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Helper function to create pattern alphabet
    fn create_pattern_alphabet(pattern: &str) -> HashMap<char, u64> {
        let pattern_len = pattern.len();
        let mut mask: HashMap<char, u64> = HashMap::new();
        
        for (i, c) in pattern.chars().enumerate() {
            mask.entry(c)
                .and_modify(|e| *e |= 1 << (pattern_len - i - 1))
                .or_insert(1 << (pattern_len - i - 1));
        }
        
        mask
    }

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

    #[test]
    fn test_pattern_too_large() {
        let text = "hello world";
        // Create a pattern that exceeds MAX_BITS
        let pattern = "a".repeat(MAX_BITS + 1);
        let pattern_alphabet = create_pattern_alphabet(&pattern);
        let options = default_options();
        
        match search(text, &pattern, &pattern_alphabet, &options) {
            Err(FuseError::PatternLengthTooLarge(_)) => {
                // This is the expected error
                assert!(true);
            },
            _ => {
                // Any other result is unexpected
                panic!("Expected PatternLengthTooLarge error");
            }
        }
    }

    #[test]
    fn test_find_all_matches() {
        let text = "abcabc";
        let pattern = "abc";
        let pattern_alphabet = create_pattern_alphabet(pattern);
        
        // Test without find_all_matches (should only find the first occurrence)
        let mut options = default_options();
        options.include_matches = true;
        
        let result = search(text, pattern, &pattern_alphabet, &options).unwrap();
        let indices = result.indices.unwrap();
        assert_eq!(indices.len(), 1);
        assert_eq!(indices, vec![(0, 5)]);
    }
}
