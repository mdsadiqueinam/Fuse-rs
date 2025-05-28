//! # fuse-rs
//! 
//! A lightweight fuzzy-search library for Rust.
//! This is a Rust port of the popular [Fuse.js](https://fusejs.io/) JavaScript library.
//!
//! ## Overview
//!
//! Fuse-rs provides fuzzy searching capability with tunable options for pattern matching,
//! scoring, and result sorting.

// Internal module structure
pub mod helpers;
pub mod core;
pub mod tools;
pub mod search;
mod transform;
//----------------------------------------------------------------------
// Public API Exports
//----------------------------------------------------------------------

// Main functionality
pub use crate::core::fuse::Fuse;
pub use crate::core::options::config::FuseOptions;
pub use crate::core::options::keys::FuseOptionKey;
pub use crate::core::options::sort::FuseSortFunction;

// Error types
pub use crate::core::error_messages::FuseError;

// Query parser types
pub use crate::core::query_parser::{
    Expression, 
    LeafNode, 
    LogicalNode, 
    ParsedExpression, 
    ParseOptions, 
    parse
};

// Search results
pub use crate::core::results::search_result::{
    RangeTuple,
    FuseResultMatch,
    FuseSearchOptions,
    FuseResult
};
pub use crate::core::results::match_result::{
    FuseSortFunctionArg,
    FuseSortFunctionItem,
    FuseSortFunctionMatch,
    FuseSortFunctionMatchList, 
    FuseSortFunctionMatchType
};
