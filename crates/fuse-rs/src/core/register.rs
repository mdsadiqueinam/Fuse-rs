use std::borrow::Cow;
use std::sync::{Mutex, OnceLock};
use crate::FuseOptions;
use crate::search::bitmap::bitmap_search::BitmapSearch;
use crate::search::search::Searcher;

// Constructor type for a searcher
pub type SearcherConstructor = fn(String, FuseOptions<'static>) -> Box<dyn Searcher>;

// Condition type for a searcher
pub type SearcherCondition = fn(&str, &FuseOptions) -> bool;

// Registry: Vec of (condition, constructor)
static REGISTERED_SEARCHERS: OnceLock<Mutex<Vec<(SearcherCondition, SearcherConstructor)>>> = OnceLock::new();

pub fn register(condition: SearcherCondition, constructor: SearcherConstructor) {
    let reg = REGISTERED_SEARCHERS.get_or_init(|| Mutex::new(Vec::new()));
    let mut reg_mut = reg.lock().unwrap();
    reg_mut.push((condition, constructor));
}

pub fn create_searcher(
    pattern: String,
    options: FuseOptions<'static>,
) -> Box<dyn Searcher> {
    let reg = REGISTERED_SEARCHERS.get_or_init(|| Mutex::new(Vec::new()));
    let reg = reg.lock().unwrap();
    for (condition, constructor) in reg.iter() {
        if condition(&pattern, &options) {
            return constructor(pattern.clone(), options.clone());
        }
    }
    Box::new(BitmapSearch::new(Cow::Owned(pattern), Cow::Owned(options)))
}
