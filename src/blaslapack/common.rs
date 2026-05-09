//! Shared types and conversions for the BLAS/LAPACK wrapper layer.
//!
//! Provides the [`AsI32`] conversion trait used across the BLAS/LAPACK wrappers to convert Rust
//! floating-point dimension and parameter values into the `i32` integers that LAPACK routines
//! expect. Implementations are provided for `f32` and `f64`. Keeping this in a dedicated module
//! avoids repetition and provides a single place to adjust the conversion strategy if the
//! underlying BLAS/LAPACK ABI ever changes.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: AsI32
/// Conversion trait for casting a floating-point value to `i32` for use in BLAS/LAPACK calls.
pub trait AsI32
{
    /// Returns the value cast to `i32`.
    fn as_i32(&self) -> i32;
}
//{{{ impl: AsI32 for f32
impl AsI32 for f32
{
    fn as_i32(&self) -> i32
    {
        *self as i32
    }
}
//}}}
//{{{ impl: AsI32 for f64
impl AsI32 for f64
{
    fn as_i32(&self) -> i32
    {
        *self as i32
    }
}
//}}}
//}}}
