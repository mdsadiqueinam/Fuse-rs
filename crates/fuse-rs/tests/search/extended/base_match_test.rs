// Tests for BaseMatch trait helper functions
use fuse_rs::search::extended::base_match::BaseMatch;
use std::sync::OnceLock;
use regex::Regex;

struct Dummy;

impl BaseMatch for Dummy {
    fn pattern(&self) -> &str { "" }
    fn multi_regex() -> &'static Regex {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        REGEX.get_or_init(|| Regex::new(r"^=(.*)$").unwrap())
    }
    fn single_regex() -> &'static Regex {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        REGEX.get_or_init(|| Regex::new(r"^=(.*)$").unwrap())
    }
    fn get_type(&self) -> &'static str { "dummy" }
    fn search(&self, _: &str) -> fuse_rs::search::search::SearchResult { unimplemented!() }
}

#[test]
fn test_get_match() {
    let pattern = "=file";
    assert_eq!(<Dummy as BaseMatch>::is_single_match(pattern), Some("file".to_string()));
    let pattern = "file";
    assert_eq!(<Dummy as BaseMatch>::is_single_match(pattern), None);
}
