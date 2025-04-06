//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, Float, One, Zero};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


pub trait SMatrixConstructors<T, const N: usize, const M: usize>
where
    T: Field + Zero + One + Copy + Default,
{
    /// Creates a new matrix with the given dimensions and initializes all elements to zero.
    fn zeros() -> Self;

    /// Creates a new matrix with the given dimensions and initializes all elements to one.
    fn ones() -> Self;

    /// Creates a new matrix with the given dimensions and initializes all elements to the specified value.
    fn from_value(value: T) -> Self;

    /// Creates a new matrix from a slice of values.
    fn from_row_slice(values: &[T]) -> Self;

    /// Creates a new matrix from a slice of values.
    fn from_col_slice(values: &[T]) -> Self;
}

pub trait DMatrixConstructors<T>
where
    T: Field + Copy + Default,
{
    /// Creates a new matrix with the given dimensions and initializes all elements to zero.
    fn zeros(rows: usize, cols: usize) -> Self where T: Zero;

    /// Creates a new matrix with the given dimensions and initializes all elements to one.
    fn ones(rows: usize, cols: usize) -> Self where T: One;

    /// Creates a new matrix with the given dimensions and initializes all elements to the specified value.
    fn from_value(rows: usize, cols: usize, value: T) -> Self;

    /// Creates a new matrix from a slice of values.
    fn from_row_slice(rows: usize, cols: usize, values: &[T]) -> Self;

    /// Creates a new matrix from a slice of values.
    fn from_col_slice(rows: usize, cols: usize, values: &[T]) -> Self;
}