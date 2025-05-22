//! Core fuzzy search implementation
//!
//! This module contains the main functionality of the fuzzy search engine,
//! including configuration options, result handling, and the primary search algorithm.

// Configuration options
pub mod options;

// Search result types and handlers
pub mod results;

// Scoring functions
pub mod compute_score;

// Error messages
pub mod error_messages;

// Main search implementation
pub mod fuse;
pub mod query_parser;
mod register;
