//! Collection of crate-wide types, traits and definitions.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::getrf::Getrf;
//}}}
//{{{ std imports
use ::std::ops::{Add, Div, Mul, Neg, Sub};
use std::cmp::PartialEq;
use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, SubAssign};

//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: compile-time checks
/// Assertion struct for compile-time checks
pub struct Assert<const CHECK: bool>;
/// This trait is used to ensure that the compile-time check is true
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

pub type AssertGreaterThan<const N: usize, const M: usize> = Assert<{ N > M }>;


//}}}
//{{{ trait: Field
pub trait Field:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
{
}
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
        impl Field for $type {}

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
pub use num_complex::Complex;
//}}}
//{{{ trait: Float
pub trait Float: Field
{
    fn acos(self) -> Self;
    fn clamp(
        self,
        min: Self,
        max: Self,
    ) -> Self;
    fn small() -> Self;
    fn powi(
        self,
        exp: i32,
    ) -> Self;
}
//}}}
//{{{ impl: Float for f32
impl Float for f32
{
    fn acos(self) -> Self
    {
        self.acos()
    }
    fn clamp(
        self,
        min: Self,
        max: Self,
    ) -> Self
    {
        f32::clamp(self, min, max)
    }
    fn small() -> Self
    {
        f32::EPSILON
    }
    fn powi(
        self,
        exp: i32,
    ) -> Self
    {
        self.powi(exp)
    }
}
//}}}
//{{{ impl: Float for f64
impl Float for f64
{
    fn acos(self) -> Self
    {
        self.acos()
    }
    fn clamp(
        self,
        min: Self,
        max: Self,
    ) -> Self
    {
        f64::clamp(self, min, max)
    }
    fn small() -> Self
    {
        f64::EPSILON
    }
    fn powi(
        self,
        exp: i32,
    ) -> Self
    {
        self.powi(exp)
    }
}
//}}}
//{{{ trait: MatrixOps
pub trait MatrixOps
where
    Self: Sized,
{
    type ScalarType: Field + Zero + One + Copy;
    type TransposeType;

    fn size(&self) -> (usize, usize);
    fn transpose(&self) -> Self::TransposeType;
    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: Getrf + Float;
    fn trace(&self) -> Self::ScalarType;
}
//}}}
//{{{ trait: VectorOps
#[allow(clippy::len_without_is_empty)]
pub trait VectorOps:
    Index<usize, Output = Self::ScalarType> + IndexMut<usize, Output = Self::ScalarType> + Sized + Clone
{
    type ScalarType: Field + Zero + One + Copy + Default;

    //{{{ fn: len
    fn len(&self) -> usize;
    //}}}
    //{{{ fn: norm
    /// Computes the norm (magnitude) of the vector.
    ///
    /// # Returns
    ///
    /// The norm of the vector as a value of type `Self::T`.
    ///
    fn norm(&self) -> Self::ScalarType
    {
        let mut out = Self::ScalarType::zero();

        for i in 0..self.len()
        {
            out += self[i] * self[i]
        }
        out
    }
    //}}}
    //{{{ fn: dot
    fn dot(
        &self,
        other: &Self,
    ) -> Self::ScalarType
    {
        if self.len() != other.len()
        {
            panic!("Vectors must be of the same length");
        }

        let mut out = Self::ScalarType::zero();
        for i in 0..self.len()
        {
            out += self[i] * other[i]
        }
        out
    }
    //}}}
    //{{{ fn: normalize
    fn normalize(&self) -> Self
    {
        let norm = self.norm();
        let mut out = self.clone();
        if norm != Self::ScalarType::zero()
        {
            for i in 0..self.len()
            {
                out[i] /= norm;
            }
        }
        out
    }
    //}}}
    //{{{ fn: cross
    fn cross(
        &self,
        other: &Self,
    ) -> Self
    {
        if self.len() != 3
        {
            panic!("Cross product is only defined for 2D and 3D vectors");
        }

        if self.len() != other.len()
        {
            panic!("Vectors must be of the same length");
        }

        let mut out = other.clone();
        out[0] = self[1] * other[2] - self[2] * other[1];
        out[1] = self[2] * other[0] - self[0] * other[2];
        out[2] = self[0] * other[1] - self[1] * other[0];
        out
    }
    //}}}
}
//}}}
//{{{ trait: FloatVectorOps
pub trait FloatVectorOps: VectorOps
where
    Self::ScalarType: Float + Zero + One + Copy + Default,
{
    //{{{ fn: angle
    fn angle(
        &self,
        other: &Self,
    ) -> Self::ScalarType
    {
        if self.len() != other.len()
        {
            panic!("Vectors must be of the same length");
        }

        if self.len() != 2 && self.len() != 3
        {
            panic!("Angle is only defined for 2D and 3D vectors");
        }

        if self.norm() < Self::ScalarType::small() || other.norm() < Self::ScalarType::small()
        {
            panic!("Cannot compute angle with zero vector");
        }

        let a = self.normalize();
        let b = other.normalize();
        let dot = (a.dot(&b)).clamp(-Self::ScalarType::one(), Self::ScalarType::one());
        dot.acos()
    }
    //}}}
}
//}}}
//{{{ fun: lin_index
#[inline]
pub fn lin_index(
    idx: (usize, usize),
    n: usize,
) -> usize
{
    idx.0 + idx.1 * n
}
//}}}
//{{{ fun: tuple_index
#[inline]
pub fn tuple_index(
    idx: usize,
    nrows: usize,
) -> (usize, usize)
{
    (idx % nrows, idx / nrows)
}

//}}}
