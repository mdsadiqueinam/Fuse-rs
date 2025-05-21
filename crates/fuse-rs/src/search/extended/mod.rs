pub(crate) mod base_match;
pub(crate) mod extended_search;
pub(crate) mod exact_match;
pub(crate) mod fuzzy_match;
pub(crate) mod include_match;
pub(crate) mod inverse_exact_match;
pub(crate) mod inverse_prefix_exact_match;
pub(crate) mod inverse_suffix_exact_match;
pub(crate) mod parse_query;
pub(crate) mod prefix_exact_match;
pub(crate) mod suffix_exact_match;

// Re-export the ExtendedSearch struct
pub use extended_search::ExtendedSearch;
