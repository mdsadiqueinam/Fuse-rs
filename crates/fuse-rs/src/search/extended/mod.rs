pub mod base_match;
pub mod extended_search;
pub mod exact_match;
pub mod fuzzy_match;
pub mod include_match;
pub mod inverse_exact_match;
pub mod inverse_prefix_exact_match;
pub mod inverse_suffix_exact_match;
pub mod parse_query;
pub mod prefix_exact_match;
pub mod suffix_exact_match;

// Re-export the ExtendedSearch struct
pub use extended_search::ExtendedSearch;
