// Tests for BaseMatch trait helper functions
use fuse_rs::search::extended::base_match::BaseMatch;
use once_cell::sync::Lazy;
use regex::Regex;

struct Dummy;

impl BaseMatch for Dummy {
    fn pattern(&self) -> &str { "" }
    fn multi_regex() -> &'static Regex {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^=(.*)$").unwrap());
        &REGEX
    }
    fn single_regex() -> &'static Regex {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^=(.*)$").unwrap());
        &REGEX
    }
    fn get_type(&self) -> &'static str { "dummy" }
    fn search(&self, _: &str) -> fuse_rs::search::search_result::SearchResult { unimplemented!() }
}

#[test]
fn test_get_match() {
    let pattern = "=file";
    assert_eq!(<Dummy as BaseMatch>::is_single_match(pattern), Some("file".to_string()));
    let pattern = "file";
    assert_eq!(<Dummy as BaseMatch>::is_single_match(pattern), None);
}
