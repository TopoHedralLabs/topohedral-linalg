//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, Float, One, Zero};
//}}}
//{{{ std imports
use core::ops::{Index, IndexMut};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------



pub trait SVectorConstructors<T, const N: usize>
where
    T: Field + Zero + One + Copy + Default,
{
    /// Creates a new vector with the given dimensions and initializes all elements to zero.
    fn zeros() -> Self;

    /// Creates a new vector with the given dimensions and initializes all elements to one.
    fn ones() -> Self;

    /// Creates a new vector with the given dimensions and initializes all elements to the specified value.
    fn from_value(value: f64) -> Self;

    /// Creates a new vector from a slice of values.
    fn from_slice(values: &[T]) -> Self;
}

pub trait DVectorConstructors<T>
{
    /// Creates a new vector with the given dimensions and initializes all elements to zero.
    fn zeros(nelem: usize) -> Self where T: Zero;

    /// Creates a new vector with the given dimensions and initializes all elements to one.
    fn ones(nelem: usize) -> Self where T: One;

    /// Creates a new vector with the given dimensions and initializes all elements to the specified value.
    fn from_value(nelem: usize, value: T) -> Self;

    /// Creates a new vector from a slice of values.
    fn from_slice(values: &[T]) -> Self;
}


