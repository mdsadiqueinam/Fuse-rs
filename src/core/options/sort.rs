use crate::core::results::match_result::FuseSortFunctionArg;

/// Function type definition for sort_fn
pub type FuseSortFunction = fn(&FuseSortFunctionArg, &FuseSortFunctionArg) -> i32;

/// Default implementation of the sort function
#[inline]
pub fn default_sort_fn(a: &FuseSortFunctionArg, b: &FuseSortFunctionArg) -> i32 {
    if (a.score - b.score).abs() < f64::EPSILON {
        if a.idx < b.idx { -1 } else { 1 }
    } else {
        if a.score < b.score { -1 } else { 1 }
    }
}

/// Wrapper for default_sort_fn to satisfy Serde's default attribute
pub fn default_sort_fn_wrapper() -> FuseSortFunction {
    default_sort_fn
}
