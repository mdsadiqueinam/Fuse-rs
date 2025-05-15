// Implements field-length normalization similar to the provided JS code.
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref SPACE_REGEX: regex::Regex = regex::Regex::new(r"\s+").unwrap();
}

pub struct Norm {
    weight: f64,
    mantissa: u32,
    cache: Mutex<HashMap<usize, f64>>,
}

impl Norm {
    pub fn new(weight: f64, mantissa: u32) -> Self {
        Norm {
            weight,
            mantissa,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, value: &str) -> f64 {
        let num_tokens = SPACE_REGEX.split(value.trim()).filter(|s| !s.is_empty()).count();
        let mut cache = self.cache.lock().unwrap();
        if let Some(&n) = cache.get(&num_tokens) {
            return n;
        }
        let m = 10f64.powi(self.mantissa as i32);
        let norm = 1.0 / (num_tokens as f64).powf(0.5 * self.weight);
        let n = ((norm * m).round() / m) as f64;
        cache.insert(num_tokens, n);
        n
    }

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
