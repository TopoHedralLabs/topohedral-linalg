//! Collection of crate-wide types, traits and definitions.
//!
//--------------------------------------------------------------------------------------------------

use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
//{{{ crate imports
//}}}
//{{{ std imports
use ::std::ops::{Add, Div, Mul, Sub};

//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: Field
pub trait Field:
    Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
    + AddAssign + SubAssign + MulAssign + DivAssign {}
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

        $macro!(u8);

        $macro!(u16);

        $macro!(u32);

        $macro!(u64);

        $macro!(u128);
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

        $macro!(u8);

        $macro!(u16);

        $macro!(u32);

        $macro!(u64);

        $macro!(u128);
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
                index: usize,
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
