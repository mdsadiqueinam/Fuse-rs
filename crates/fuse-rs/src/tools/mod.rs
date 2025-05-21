//! Utility tools for search indexing and scoring
//!
//! This module contains various utilities used by the fuzzy search implementation,
//! including key management, indexing, and normalization.

// Internal module structure
pub mod key_store;
pub mod norm;
pub mod fuse_index;
pub mod fuse_index_record;
