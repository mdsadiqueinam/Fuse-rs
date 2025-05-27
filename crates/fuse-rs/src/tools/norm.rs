//! Field-length normalization utilities for scoring
//!
//! This module provides functionality to normalize field lengths during
//! the scoring process of fuzzy search results. The normalization ensures
//! that field length is appropriately factored into relevance scoring.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

//----------------------------------------------------------------------
// Constants & Statics
//----------------------------------------------------------------------

/// Regular expression to split text into tokens by whitespace
static SPACE_REGEX: OnceLock<regex::Regex> = OnceLock::new();

//----------------------------------------------------------------------
// Normalization Implementation
//----------------------------------------------------------------------

/// Handles field-length normalization for scoring calculations.
///
/// The `Norm` struct calculates and caches normalization factors based on
/// text field lengths, which are used to adjust relevance scores during search.
///
/// # Example
///
/// ```no_run
/// // This is internal API, not meant to be used directly
/// // Code shown for illustration purposes only
/// 
/// // let normalizer = Norm::new(0.5, 3);
/// // let value = "hello world";
/// // let norm_factor = normalizer.get(value);
/// ```
#[derive(Debug)]
pub struct Norm {
    /// Influence weight of field length (higher = more influence)
    weight: f64,
    
    /// Precision control for calculations
    mantissa: u32,
    
    /// Cache of previously calculated normalization values by token count
    cache: Mutex<HashMap<usize, f64>>,
}

impl Norm {
    /// Creates a new field normalizer with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `weight` - How much field length should affect scoring (0.0-1.0)
    /// * `mantissa` - Number of decimal places for precision
    ///
    /// # Returns
    ///
    /// A new `Norm` instance ready for normalization calculations
    pub fn new(weight: f64, mantissa: u32) -> Self {
        Norm {
            weight,
            mantissa,
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Calculates the normalization factor for a given string value.
    ///
    /// This method counts the tokens in the input string and returns
    /// a normalization factor that can be used in scoring calculations.
    /// Results are cached for performance.
    ///
    /// # Arguments
    ///
    /// * `value` - The string to calculate normalization for
    ///
    /// # Returns
    ///
    /// A normalization factor as a float value
    pub fn get(&self, value: &str) -> f64 {
        // Count non-empty tokens in the string
        let space_regex = SPACE_REGEX.get_or_init(|| regex::Regex::new(r"\s+").unwrap());
        let num_tokens = space_regex
            .split(value.trim())
            .filter(|s| !s.is_empty())
            .count();
        
        // Check cache first
        let mut cache = self.cache.lock().unwrap();
        if let Some(&n) = cache.get(&num_tokens) {
            return n;
        }
        
        // Calculate normalization factor
        let m = 10f64.powi(self.mantissa as i32);
        let norm = 1.0 / (num_tokens as f64).powf(0.5 * self.weight);
        
        // Round to specified precision and cache result
        let n = ((norm * m).round() / m) as f64;
        cache.insert(num_tokens, n);
        n
    }

    /// Clears the internal cache of normalization values.
    ///
    /// This can be useful if memory usage is a concern or if
    /// normalization parameters have been changed.
    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_get_basic() {
        let norm = Norm::new(0.5, 3);
        let value = "foo bar baz";
        let n = norm.get(value);
        // For 3 tokens, weight=0.5, mantissa=3
        // norm = 1 / (3.0).powf(0.25) = 1 / 1.316... = ~0.759
        // m = 1000, so rounded to 0.76
        assert!((n - 0.76).abs() < 0.001);
    }

    #[test]
    fn test_norm_cache_and_clear() {
        let norm = Norm::new(1.0, 2);
        let value = "a b c d";
        let n1 = norm.get(value);
        let n2 = norm.get(value);
        assert_eq!(n1, n2); // Should be cached
        norm.clear();
        let n3 = norm.get(value);
        assert_eq!(n1, n3); // Should recompute but same value
    }

    #[test]
    fn test_norm_single_token() {
        let norm = Norm::new(1.0, 2);
        let value = "single";
        let n = norm.get(value);
        assert!((n - 1.0).abs() < 0.001);
    }
}
