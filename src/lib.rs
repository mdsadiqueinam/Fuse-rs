mod helpers;
mod core;
mod tools;

// Export the old modules for backward compatibility
pub use crate::core::fuse::Fuse;
pub use crate::core::options::config::FuseOptions;
pub use crate::core::options::keys::FuseOptionKey;
pub use crate::core::options::sort::FuseSortFunction;
pub use crate::core::results::search_result::SearchResult;
pub use crate::core::results::match_result::{
    FuseSortFunctionArg,
    FuseSortFunctionItem,
    FuseSortFunctionMatch,
    FuseSortFunctionMatchList, 
    FuseSortFunctionMatchType
};