//! Core fuzzy search implementation
//!
//! This module contains the main functionality of the fuzzy search engine,
//! including configuration options, result handling, and the primary search algorithm.

// Configuration options
pub(crate) mod options;

// Search result types and handlers
pub(crate) mod results;

// Scoring functions
pub(crate) mod compute_score;

// Main search implementation
pub(crate) mod fuse;