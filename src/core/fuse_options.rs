use crate::helpers::get::{self, GetFn};
use crate::core::types::{FuseSortFunction, default_sort_fn, default_sort_fn_wrapper};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;


fn default_get_fn_wrapper() -> fn(&Value, &Vec<String>) -> Option<get::GetValue> {
    get::get
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuseOptionKeyName<'a> {
    String(Cow<'a, str>),
    StringArray(Vec<Cow<'a, str>>),
}

pub type FuseKeyValueGetter = Option<fn(&Value) -> &str>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseOptionKeyObject<'a> {
    pub name: Cow<'a, FuseOptionKeyName<'a>>, // Use Cow to avoid cloning
    pub weight: Option<f64>,

    #[serde(skip)]
    pub get_fn: FuseKeyValueGetter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseOptionKey<'a> {
    KeyObject(FuseOptionKeyObject<'a>),
    String(Cow<'a, str>),
    StringArray(Vec<Cow<'a, str>>),
}

impl<'a> Default for FuseOptionKey<'a> {
    fn default() -> Self {
        Self::StringArray(Vec::new())
    }
}

/// Represents a search result item
#[derive(Debug, Default)]
pub struct SearchResult {
    pub score: f64,
    pub idx: usize,
}

/// Configuration options for Fuse.js
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseOptions<'a> {
    #[serde(default)]
    pub is_case_sensitive: bool,
    #[serde(default)]
    pub ignore_diacritics: bool,
    #[serde(default)]
    pub include_score: bool,
    #[serde(default)]
    pub keys: FuseOptionKey<'a>,
    #[serde(default)]
    pub should_sort: bool,

    #[serde(skip, default = "default_sort_fn_wrapper")]
    pub sort_fn: FuseSortFunction,

    #[serde(default)]
    pub include_matches: bool,
    #[serde(default)]
    pub find_all_matches: bool,
    #[serde(default)]
    pub min_match_char_length: usize,

    #[serde(default)]
    pub location: usize,
    #[serde(default)]
    pub threshold: f64,
    #[serde(default)]
    pub distance: usize,

    #[serde(default)]
    pub use_extended_search: bool,

    #[serde(skip, default = "default_get_fn_wrapper")]
    pub get_fn: GetFn<Vec<String>>,

    #[serde(default)]
    pub ignore_location: bool,
    #[serde(default)]
    pub ignore_field_norm: bool,
    #[serde(default)]
    pub field_norm_weight: usize,
}

impl<'a> Default for FuseOptions<'a> {
    fn default() -> Self {
        Self {
            is_case_sensitive: false,
            ignore_diacritics: false,
            include_score: false,
            keys: FuseOptionKey::default(),
            should_sort: true,
            sort_fn: default_sort_fn,
            include_matches: false,
            find_all_matches: false,
            min_match_char_length: 1,
            location: 0,
            threshold: 0.6,
            distance: 100,
            use_extended_search: false,
            get_fn: get::get,
            ignore_location: false,
            ignore_field_norm: false,
            field_norm_weight: 1,
        }
    }
}
