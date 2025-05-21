use fuse_rs::search::bitmap::compute_score::compute_score;
use fuse_rs::core::options::config::FuseOptions;

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