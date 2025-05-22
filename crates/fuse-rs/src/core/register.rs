use std::borrow::Cow;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::FuseOptions;
use crate::search::bitmap::bitmap_search::BitmapSearch;
use crate::search::extended::ExtendedSearch;
use crate::search::search_result::SearchResult;

pub trait Searcher {
    fn search_in(&self, text: &str) -> SearchResult;
}

impl Searcher for BitmapSearch<'static> {
    fn search_in(&self, text: &str) -> SearchResult {
        BitmapSearch::search_in(self, text)
    }
}

impl Searcher for ExtendedSearch<'static> {
    fn search_in(&self, text: &str) -> SearchResult {
        ExtendedSearch::search_in(self, text)
    }
}

type SearcherFactory = fn(pattern: String, options: FuseOptions<'static>) -> Box<dyn Searcher>;

static REGISTERED_SEARCHERS: Lazy<Mutex<Vec<(fn(&str, &FuseOptions) -> bool, SearcherFactory)>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register(
    condition: fn(&str, &FuseOptions) -> bool,
    factory: SearcherFactory,
) {
    let mut reg = REGISTERED_SEARCHERS.lock().unwrap();
    reg.push((condition, factory));
}

pub fn create_searcher(
    pattern: String,
    options: FuseOptions<'static>,
) -> Box<dyn Searcher> {
    let reg = REGISTERED_SEARCHERS.lock().unwrap();
    for (condition, factory) in reg.iter() {
        if condition(&pattern, &options) {
            return factory(pattern.clone(), options.clone());
        }
    }
    Box::new(BitmapSearch::new(Cow::Owned(pattern), Cow::Owned(options)))
}
