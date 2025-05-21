use crate::FuseError;
use crate::FuseOptions;
use crate::helpers::str_ext::StrExt;
use std::collections::HashMap;
use crate::search::search_result::SearchResult;
use super::compute_score::compute_score;
use super::constants::MAX_BITS;
use super::convert_mask_to_indices::convert_mask_to_indices;


pub fn search(
    text: &str,
    pattern: &str,
    pattern_alphabet: &HashMap<char, u32>,
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

    let mut last_bit_arr: Vec<u32> = Vec::new();
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

