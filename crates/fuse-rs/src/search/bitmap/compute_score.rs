use crate::core::options::config::FuseOptions;

/// Computes the score for a match with a given pattern.
///
/// # Arguments
///
/// * `pattern_length` - The length of the pattern being searched
/// * `errors` - Number of errors in the match
/// * `current_location` - Position of the current match
/// * `expected_location` - Position where the match was expected
/// * `distance` - How far to look for matches (from FuseOptions)
/// * `ignore_location` - Whether to ignore location matching (from FuseOptions)
///
/// # Returns
///
/// A score between 0.0 (perfect match) and 1.0 (completely different)
pub fn compute_score(
    pattern: &str,
    errors: usize,
    current_location: usize,
    expected_location: usize,
    options: &FuseOptions,
) -> f64 {
    // Calculate the score based on the error ratio
    let accuracy = errors as f64 / pattern.len() as f64;
    
    // If location is ignored, just return the accuracy score
    if options.ignore_location {
        return accuracy;
    }
    
    // Calculate how far the match is from its expected location
    let proximity = (expected_location as isize - current_location as isize).abs() as usize;
    
    // If distance is 0, avoid a divide by zero error
    if options.distance == 0 {
        return if proximity != 0 { 1.0 } else { accuracy };
    }
    
    // Calculate the final score as a combination of accuracy and proximity
    accuracy + (proximity as f64 / options.distance as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::config::FuseOptions;

    #[test]
    fn test_compute_score_with_exact_match() {
        let options = FuseOptions {
            distance: 100,
            ignore_location: false,
            ..Default::default()
        };
        
        let score = compute_score("aaaaa", 0, 10, 10, &options);
        assert_eq!(score, 0.0);
    }
    
    #[test]
    fn test_compute_score_with_errors() {
        let options = FuseOptions {
            distance: 100,
            ignore_location: false,
            ..Default::default()
        };
        
        let score = compute_score("aaaaaaaaaa", 2, 10, 10, &options);
        assert_eq!(score, 0.2);
    }
    
    #[test]
    fn test_compute_score_with_location_difference() {
        let options = FuseOptions {
            distance: 100,
            ignore_location: false,
            ..Default::default()
        };
        
        let score = compute_score("aaaaa", 0, 0, 10, &options);
        assert_eq!(score, 0.1);
    }
    
    #[test]
    fn test_compute_score_with_ignore_location() {
        let options = FuseOptions {
            distance: 100,
            ignore_location: true,
            ..Default::default()
        };
        
        let score = compute_score("aaaaa", 1, 0, 10, &options);
        assert_eq!(score, 0.2);
    }
    
    #[test]
    fn test_compute_score_with_zero_distance() {
        let options = FuseOptions {
            distance: 0,
            ignore_location: false,
            ..Default::default()
        };
        
        // When proximity is non-zero and distance is zero
        let score1 = compute_score("aaaaa", 1, 0, 10, &options);
        assert_eq!(score1, 1.0);
        
        // When proximity is zero and distance is zero
        let score2 = compute_score("aaaaa", 1, 10, 10, &options);
        assert_eq!(score2, 0.2);
    }
}