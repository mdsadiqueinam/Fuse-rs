/// Converts a match mask to an array of index pairs.
///
/// This function takes a boolean mask array where `true` represents a match at that position,
/// and converts it into a list of start-end index pairs representing contiguous matches.
/// Only matches that are at least `min_match_char_length` long are included.
///
/// # Arguments
///
/// * `match_mask` - Vector of booleans where `true` indicates a match at that position
/// * `min_match_char_length` - Minimum length required for a valid match
///
/// # Returns
///
/// * Vector of `[start, end]` index pairs representing contiguous matches
///
#[allow(dead_code)]
pub fn convert_mask_to_indices(
    match_mask: &[bool],
    min_match_char_length: usize,
) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    let mut start: isize = -1;
    let mut end: isize = -1;

    // Process each position in the match mask
    for i in 0..match_mask.len() {
        let is_match = match_mask[i];

        if is_match && start == -1 {
            // Start of a new match sequence
            start = i as isize;
        } else if !is_match && start != -1 {
            // End of a match sequence
            end = i as isize - 1;

            // Only include matches that meet the minimum length requirement
            if end - start + 1 >= min_match_char_length as isize {
                indices.push((start as usize, end as usize));
            }

            // Reset for next sequence
            start = -1;
        }
    }

    // Handle case where match extends to the end of the array
    if !match_mask.is_empty() && match_mask[match_mask.len() - 1] && start != -1 {
        if (match_mask.len() as isize - start) >= min_match_char_length as isize {
            indices.push((start as usize, (match_mask.len() - 1) as usize));
        }
    }

    indices
}

