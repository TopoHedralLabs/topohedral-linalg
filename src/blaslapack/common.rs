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
pub trait AsI32 {
    /// Returns the value cast to `i32`.
    fn as_i32(&self) -> i32;
}
//{{{ impl: AsI32 for f32
impl AsI32 for f32 {
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
//}}}
//{{{ impl: AsI32 for f64
impl AsI32 for f64 {
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
//}}}
//}}}

//{{{ collection: BLAS/LAPACK validation
/// Converts a Rust matrix dimension to the integer type used by BLAS/LAPACK.
///
/// # Panics
///
/// Panics when `value` exceeds the range supported by the linked BLAS/LAPACK ABI.
#[inline]
pub(crate) fn blas_dim(
    name: &str,
    value: usize,
) -> i32 {
    i32::try_from(value)
        .unwrap_or_else(|_| panic!("{name} exceeds the maximum BLAS/LAPACK dimension"))
}

/// Computes a matrix buffer length without allowing `usize` overflow.
///
/// # Panics
///
/// Panics when `nrows * ncols` cannot be represented by `usize`.
#[inline]
pub(crate) fn matrix_len(
    name: &str,
    nrows: usize,
    ncols: usize,
) -> usize {
    nrows
        .checked_mul(ncols)
        .unwrap_or_else(|| panic!("{name} dimensions overflow usize"))
}

/// Verifies that a column-major buffer has exactly the expected matrix length.
///
/// # Panics
///
/// Panics when the dimensions overflow or `actual` differs from the expected length.
#[inline]
pub(crate) fn assert_matrix_len(
    name: &str,
    actual: usize,
    nrows: usize,
    ncols: usize,
) {
    let expected = matrix_len(name, nrows, ncols);
    assert_eq!(
        actual, expected,
        "{name} buffer length does not match its dimensions"
    );
}

/// Validates a workspace size returned by a LAPACK workspace query.
///
/// # Panics
///
/// Panics when LAPACK returns a non-positive workspace size.
#[inline]
pub(crate) fn workspace_len<T>(value: &T) -> (usize, i32)
where
    T: AsI32,
{
    let lwork = value.as_i32();
    assert!(
        lwork > 0,
        "LAPACK workspace query returned a non-positive length"
    );
    (lwork as usize, lwork)
}
//}}}
