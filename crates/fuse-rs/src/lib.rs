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
mod helpers;
mod core;
mod tools;
mod search;

//----------------------------------------------------------------------
// Public API Exports
//----------------------------------------------------------------------

// Main functionality
pub use crate::core::fuse::Fuse;
pub use crate::core::options::config::FuseOptions;
pub use crate::core::options::keys::FuseOptionKey;
pub use crate::core::options::sort::FuseSortFunction;

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