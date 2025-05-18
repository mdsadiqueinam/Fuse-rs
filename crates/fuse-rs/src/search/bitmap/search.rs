use crate::FuseError;
use crate::FuseOptions;
use crate::helpers::str_ext::StrExt;
use std::collections::HashMap;

use super::compute_score::compute_score;
use super::constants::MAX_BITS;

pub struct SearchResult {
    /// Whether the pattern was found in the text
    pub is_match: bool,

    /// The match quality score (lower is better)
    pub score: f64,

    /// List of match position ranges as (start, end) tuples
    pub indices: Vec<(usize, usize)>,
}

pub fn search(
    text: &str,
    pattern: &str,
    pattern_alphabet: &HashMap<char, u64>,
    options: &FuseOptions,
) -> Result<SearchResult, FuseError> {
    if pattern.len() > MAX_BITS {
        return Err(FuseError::PatternLengthTooLarge(MAX_BITS));
    }

    let pattern_length = pattern.len();
    let text_length = text.len();
    let expected_location = options.location.min(text_length);
    let mut current_threshold = options.threshold;
    let mut best_location: isize = expected_location as isize;

    let compute_matches = options.min_match_char_length > 1 || options.include_matches;
    let mut match_mask = if compute_matches {
        vec![0; text_length]
    } else {
        Vec::new()
    };

    let mut final_score = 1.0;

    while let Some(i) = text.index_of(pattern, 0.max(best_location) as usize) {
        let score = compute_score(pattern, 0, i, expected_location, options);
        current_threshold = score.min(current_threshold);
        best_location = (i + pattern_length) as isize;

        if compute_matches {
            for j in 0..pattern_length {
                match_mask[i + j] = 1;
            }
        }
    }

    best_location = -1;
    let mut last_bit_arr = vec![0u64; 0];
    let mut bin_max = pattern_length + text_length;
    let mask = 1 << (pattern_length - 1);

    for i in 0..pattern_length {
        let mut bin_min = 0;
        let mut bin_mid = bin_max;

        while bin_min < bin_mid {
            let mid = bin_min + (bin_max - bin_min) / 2;
            let score = compute_score(pattern, i, expected_location + mid, expected_location, options);

            if score <= current_threshold {
                bin_min = mid;
            } else {
                bin_max = mid;
            }

            bin_mid = bin_min + (bin_max - bin_min) / 2;
        }

        bin_max = bin_mid;

        let mut start = 1.max(expected_location as isize - bin_mid as isize + 1) as usize;
        let finish = if options.find_all_matches {
            text_length
        } else {
            (expected_location + bin_mid).min(text_length) + pattern_length
        };

        let mut bit_arr = vec![0u64; finish + 2];
        bit_arr[finish + 1] = (1 << i) - 1;

        for j in (start..=finish).rev() {
            let current_location = j - 1;
            let char_match = *pattern_alphabet.get(&text.chars().nth(current_location).unwrap_or('\0')).unwrap_or(&0);

            if compute_matches {
                match_mask[current_location] = if char_match > 0 { 1 } else { 0 };
            }

            bit_arr[j] = ((bit_arr[j + 1] << 1) | 1) & char_match;

            if i > 0 {
                bit_arr[j] |= ((last_bit_arr.get(j + 1).copied().unwrap_or(0)
                    | last_bit_arr.get(j).copied().unwrap_or(0))
                    << 1)
                    | 1
                    | last_bit_arr.get(j + 1).copied().unwrap_or(0);
            }

            if (bit_arr[j] & mask) != 0 {
                final_score = compute_score(pattern, i, current_location, expected_location, options);

                if final_score <= current_threshold {
                    current_threshold = final_score;
                    best_location = current_location as isize;

                    if best_location <= expected_location as isize {
                        break;
                    }

                    start = (1.max(2 * expected_location as isize - best_location) as usize).max(1);
                }
            }
        }

        let score = compute_score(pattern, i + 1, expected_location, expected_location, options);
        if score > current_threshold {
            break;
        }

        last_bit_arr = bit_arr;
    }

    let mut result = SearchResult {
        is_match: best_location >= 0,
        score: final_score.max(0.001),
        indices: vec![],
    };

    if compute_matches {
        let mut indices = Vec::new();
        let mut start: Option<usize> = None;
        for (i, &bit) in match_mask.iter().enumerate() {
            if bit == 1 {
                if start.is_none() {
                    start = Some(i);
                }
            } else if let Some(s) = start {
                if i - s >= options.min_match_char_length {
                    indices.push((s, i - 1));
                }
                start = None;
            }
        }
        if let Some(s) = start {
            if text_length - s >= options.min_match_char_length {
                indices.push((s, text_length - 1));
            }
        }

        if indices.is_empty() {
            result.is_match = false;
        } else if options.include_matches {
            result.indices = indices;
        }
    }

    Ok(result)
}
