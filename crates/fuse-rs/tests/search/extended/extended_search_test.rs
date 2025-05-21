// Tests for ExtendedSearch
use fuse_rs::search::extended::extended_search::ExtendedSearch;
use fuse_rs::FuseOptions;

#[test]
fn test_extended_search_basic() {
    let options = FuseOptions::default();
    let pattern = "^core go$ | rb$ | py$ xy$";
    let search = ExtendedSearch::new(pattern.to_string(), std::borrow::Cow::Borrowed(&options));
    let text = "corelib.go";
    let result = search.search_in(text);
    assert!(result.is_match);
    let text = "corelib.rb";
    let result = search.search_in(text);
    assert!(result.is_match);
    let text = "corelib.py";
    let result = search.search_in(text);
    assert!(result.is_match);
    let text = "corelib.rs";
    let result = search.search_in(text);
    assert!(!result.is_match);
}
