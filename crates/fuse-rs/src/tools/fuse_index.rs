//! Search index implementation for improved search performance
//!
//! This module provides the indexing functionality that speeds up
//! fuzzy searches by pre-processing the data collection.

use crate::{helpers::get::GetFn, FuseOptions};
use super::norm::Norm;

//----------------------------------------------------------------------
// Types & Constants
//----------------------------------------------------------------------

/// Default size for the n-gram indexing
const DEFAULT_NGRAM_SIZE: usize = 3;

/// Search index for fast fuzzy search operations
///
/// This structure maintains an inverted index mapping tokens to document IDs,
/// which allows for faster search operations compared to linear scanning.
#[derive(Debug)]
pub struct FuseIndex {
    norm: Norm,
    get_fn: GetFn,
    is_created: bool,
}

//----------------------------------------------------------------------
// Implementation
//----------------------------------------------------------------------

impl FuseIndex {
   pub fn new(options: &FuseOptions) -> Self {
        FuseIndex {
            norm: Norm::new(options.field_norm_weight, 3),
            get_fn: options.get_fn,
            is_created: false,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index_creation() {
        // TODO: Implement tests when the actual index is implemented
    }
}