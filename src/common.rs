//! Collection of crate-wide types, traits and definitions.
//!
//--------------------------------------------------------------------------------------------------

use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
//{{{ crate imports
//}}}
//{{{ std imports
use ::std::ops::{Add, Div, Mul, Sub, Index, Neg, IndexMut};
use std::cmp::PartialEq;

//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: Field
pub trait Field:
    Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
    + AddAssign + SubAssign + MulAssign + DivAssign + Neg + PartialEq{}
//}}}
//{{{ trait: IndexValue
pub trait IndexValue<I>
{
    type Output;

    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output;
}

//}}}
//{{{ macro: apply_for_all_types
#[macro_export]
macro_rules! apply_for_all_types {
    ($macro:ident) => {
        $macro!(f32);

        $macro!(f64);

        $macro!(i8);

        $macro!(i16);

        $macro!(i32);

        $macro!(i64);

        $macro!(i128);
    };
}

//}}}
//{{{ macro: apply_for_all_integer_types
#[macro_export]
macro_rules! apply_for_all_integer_types {
    ($macro:ident) => {
        $macro!(i8);

        $macro!(i16);

        $macro!(i32);

        $macro!(i64);

        $macro!(i128);

    };
}

//}}}
//{{{ macro: impl_field
macro_rules! impl_field {
    ($type:ty) => {
        impl Field for $type
        {}

        impl IndexValue<usize> for $type
        {
            type Output = Self;

            #[inline]

            fn index_value(
                &self,
                _index: usize,
            ) -> Self::Output
            {
                *self
            }
        }
    };
}

//}}}
//{{{ collection: impl_field implementations
apply_for_all_types!(impl_field);
//}}}
//{{{ trait: Zero
pub trait Zero
{
    fn zero() -> Self;
}
//}}}
//{{{ collectoin: impl_zero implementations
impl Zero for f32
{
    fn zero() -> Self
    {

        0.0
    }
}

impl Zero for f64
{
    fn zero() -> Self
    {

        0.0
    }
}

macro_rules! impl_zero {
    ($type:ty) => {
        impl Zero for $type
        {
            fn zero() -> Self
            {

                0
            }
        }
    };
}

apply_for_all_integer_types!(impl_zero);
//}}}
//{{{ trait: One
pub trait One
{
    fn one() -> Self;
}
//}}}
//{{{ collection: impl_one implementations

impl One for f32
{
    fn one() -> Self
    {

        1.0
    }
}

impl One for f64
{
    fn one() -> Self
    {

        1.0
    }
}

macro_rules! impl_zero {
    ($type:ty) => {
        impl One for $type
        {
            fn one() -> Self
            {

                1
            }
        }
    };
}

apply_for_all_integer_types!(impl_zero);
//}}}
//{{{ collection: re-exports
pub use num_complex::{Complex, Complex64, Complex32};
//}}}

trait VectorOps: Index<usize, Output = Self::T> + IndexMut<usize,Output = Self::T> + Sized + Clone {

    type T: Field + Zero + One + Copy + Default; 

    /// Computes the norm (magnitude) of the vector.
    ///
    /// # Returns
    ///
    /// The norm of the vector as a value of type `Self::T`.
    ///
    fn norm(&self) -> Self::T {

        let mut out = Self::T::zero();

        for i in 0..self.len() {
            out += self[i] * self[i]
        }

        // out = T::sq
        out
    }
    fn dot(&self, other: &Self) -> Self::T {
        let mut out = Self::T::zero();
        for i in 0..self.len() {
            out += self[i] * other[i]
        }
        out
    }
    fn normalize(&self) -> Self {
        let norm = self.norm();
        let mut out = self.clone();
        if norm != Self::T::zero() {
            for i in 0..self.len() {
                out[i] /= norm;
            }
        }
        out
    }
    fn cross(&self, other: &Self) -> Self;
    fn len(&self) -> usize;
    fn angle(&self, other: &Self) -> Self::T; 
}