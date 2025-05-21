use fuse_rs::tools::norm::Norm;

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