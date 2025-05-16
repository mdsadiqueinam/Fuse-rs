//! Utility tools for search indexing and scoring
//!
//! This module contains various utilities used by the fuzzy search implementation,
//! including key management, indexing, and normalization.

// Internal module structure
pub(crate) mod key_store;
pub(crate) mod norm;
pub(crate) mod fuse_index;
pub(crate) mod fuse_index_record;