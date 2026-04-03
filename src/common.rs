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
struct Assert<const CHECK: bool>;
/// This trait is used to ensure that the compile-time check is true
trait IsTrue {}
impl IsTrue for Assert<true> {}

pub trait GreaterThan<const N: usize, const M: usize> {}
impl<const N: usize, const M: usize> GreaterThan<N, M> for () where Assert<{ N > M }>: IsTrue {}
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
//{{{ trait: Abs
pub trait Abs
{
    fn abs(self) -> Self;
}
//}}}
//{{{ collection: impl_abs implementations
impl Abs for f32
{
    fn abs(self) -> Self
    {
        self.abs()
    }
}

impl Abs for f64
{
    fn abs(self) -> Self
    {
        self.abs()
    }
}

macro_rules! impl_abs {
    ($type:ty) => {
        impl Abs for $type
        {
            fn abs(self) -> Self
            {
                self.abs()
            }
        }
    };
}

apply_for_all_integer_types!(impl_abs);
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

    fn sqrt(self) -> Self;
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

    fn sqrt(self) -> Self
    {
        self.sqrt()
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

    fn sqrt(self) -> Self
    {
        self.sqrt()
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

    fn transpose(&self) -> Self::TransposeType;
    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: Getrf + Float;
    fn trace(&self) -> Self::ScalarType;
}
//}}}
//{{{ trait: Shape
pub trait Shape
where
    Self: Sized,
{
    fn nrows(&self) -> usize;
    fn ncols(&self) -> usize;
    fn size(&self) -> (usize, usize)
    {
        (self.nrows(), self.ncols())
    }
}
//}}}
//{{{ impl: Shape for references
impl<T> Shape for &T
where
    T: Shape,
{
    fn nrows(&self) -> usize
    {
        (*self).nrows()
    }

    fn ncols(&self) -> usize
    {
        (*self).ncols()
    }
}

impl<T> Shape for &mut T
where
    T: Shape,
{
    fn nrows(&self) -> usize
    {
        (**self).nrows()
    }

    fn ncols(&self) -> usize
    {
        (**self).ncols()
    }
}
//}}}
//{{{ trait: MatMul
pub trait MatMul<Rhs = Self>
where
    Self: Sized,
{
    type Output;

    fn matmul(
        self,
        rhs: Rhs,
    ) -> Self::Output;
}
//}}}
//{{{ trait: VectorOps
#[allow(clippy::len_without_is_empty)]
pub trait VectorOps:
    Index<usize, Output = Self::ScalarType> + IndexMut<usize, Output = Self::ScalarType> + Sized + Clone
{
    type ScalarType: Field + Zero + One + Copy + Default + Float;

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
        out.sqrt()
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
//{{{ trait: TransformOps
pub trait TransformOps
where
    Self: Sized,
{
    type ScalarType: Field + Copy;

    /// Applies `f` to each element in-place.
    fn transform<F>(
        &mut self,
        f: F,
    ) where
        F: FnMut(Self::ScalarType) -> Self::ScalarType;

    /// Returns a transformed copy of `self`.
    fn transformed<F>(
        &self,
        f: F,
    ) -> Self
    where
        Self: Clone,
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        let mut out = self.clone();
        out.transform(f);
        out
    }

    /// Consumes `self`, transforms it in-place, and returns it.
    fn into_transformed<F>(
        mut self,
        f: F,
    ) -> Self
    where
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        self.transform(f);
        self
    }

    /// Shifts every element by `value`.
    fn shift(
        &mut self,
        value: Self::ScalarType,
    )
    {
        self.transform(|element| element + value);
    }

    fn shifted(
        &self,
        value: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|element| element + value)
    }

    fn into_shifted(
        self,
        value: Self::ScalarType,
    ) -> Self
    {
        self.into_transformed(|element| element + value)
    }

    /// Scales every element by `value`.
    fn scale(
        &mut self,
        value: Self::ScalarType,
    )
    {
        self.transform(|element| element * value);
    }

    fn scaled(
        &self,
        value: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|element| element * value)
    }

    fn into_scaled(
        self,
        value: Self::ScalarType,
    ) -> Self
    {
        self.into_transformed(|element| element * value)
    }

    /// Assigns every element to `value`.
    fn fill(
        &mut self,
        value: Self::ScalarType,
    )
    {
        self.transform(|_| value);
    }

    fn filled(
        &self,
        value: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|_| value)
    }

    fn into_filled(
        self,
        value: Self::ScalarType,
    ) -> Self
    {
        self.into_transformed(|_| value)
    }
}
//}}}
//{{{ trait: FloatTransformOps
pub trait FloatTransformOps: TransformOps
where
    Self::ScalarType: Float,
{
    fn acos(&self) -> Self
    where
        Self: Clone,
    {
        self.transformed(|value| value.acos())
    }

    fn clamp(
        &self,
        min: Self::ScalarType,
        max: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|value| value.clamp(min, max))
    }

    fn powi(
        &self,
        exp: i32,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|value| value.powi(exp))
    }

    fn sqrt(&self) -> Self
    where
        Self: Clone,
    {
        self.transformed(|value| value.sqrt())
    }
}
//}}}
//{{{ impl: FloatTransformOps for T
impl<T> FloatTransformOps for T
where
    T: TransformOps,
    T::ScalarType: Float,
{
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
//{{{ trait: ReduceOps
pub trait ReduceOps
{
    type Item: Copy;
    type Index: Copy;

    fn fold<B, F>(
        &self,
        init: B,
        f: F,
    ) -> B
    where
        F: FnMut(B, Self::Item) -> B;

    fn fold_indexed<B, F>(
        &self,
        init: B,
        f: F,
    ) -> B
    where
        F: FnMut(B, Self::Index, Self::Item) -> B;

    fn is_empty(&self) -> bool
    {
        !self.fold(false, |_, _| true)
    }

    fn reduce<F>(
        &self,
        mut f: F,
    ) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        self.fold(None, |acc, value| {
            Some(match acc
            {
                Some(current) => f(current, value),
                None => value,
            })
        })
    }

    fn sum(&self) -> Self::Item
    where
        Self::Item: Add<Output = Self::Item> + Zero,
    {
        self.fold(Self::Item::zero(), |acc, value| acc + value)
    }

    fn product(&self) -> Self::Item
    where
        Self::Item: Mul<Output = Self::Item> + One,
    {
        self.fold(Self::Item::one(), |acc, value| acc * value)
    }

    fn min(&self) -> Option<Self::Item>
    where
        Self::Item: PartialOrd,
    {
        self.reduce(|left, right| {
            if left <= right
            {
                left
            }
            else
            {
                right
            }
        })
    }

    fn max(&self) -> Option<Self::Item>
    where
        Self::Item: PartialOrd,
    {
        self.reduce(|left, right| {
            if left >= right
            {
                left
            }
            else
            {
                right
            }
        })
    }

    fn min_by_key<K, F>(
        &self,
        mut key: F,
    ) -> Option<Self::Item>
    where
        K: PartialOrd,
        F: FnMut(Self::Item) -> K,
    {
        self.fold(None, |acc, value| {
            let current_key = key(value);
            Some(match acc
            {
                Some((best_value, best_key)) if best_key <= current_key => (best_value, best_key),
                _ => (value, current_key),
            })
        })
        .map(|(value, _)| value)
    }

    fn max_by_key<K, F>(
        &self,
        mut key: F,
    ) -> Option<Self::Item>
    where
        K: PartialOrd,
        F: FnMut(Self::Item) -> K,
    {
        self.fold(None, |acc, value| {
            let current_key = key(value);
            Some(match acc
            {
                Some((best_value, best_key)) if best_key >= current_key => (best_value, best_key),
                _ => (value, current_key),
            })
        })
        .map(|(value, _)| value)
    }

    fn transform_min<F>(
        &self,
        transform: F,
    ) -> Option<Self::Item>
    where
        Self::Item: PartialOrd,
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.min_by_key(transform)
    }

    fn transform_max<F>(
        &self,
        transform: F,
    ) -> Option<Self::Item>
    where
        Self::Item: PartialOrd,
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.max_by_key(transform)
    }

    fn abs_min(&self) -> Option<Self::Item>
    where
        Self::Item: Abs + PartialOrd,
    {
        self.min_by_key(|value| value.abs())
    }

    fn abs_max(&self) -> Option<Self::Item>
    where
        Self::Item: Abs + PartialOrd,
    {
        self.max_by_key(|value| value.abs())
    }

    fn argmin(&self) -> Option<(Self::Index, Self::Item)>
    where
        Self::Item: PartialOrd,
    {
        self.fold_indexed(None, |acc, index, value| {
            Some(match acc
            {
                Some((best_index, best_value)) if best_value <= value => (best_index, best_value),
                _ => (index, value),
            })
        })
    }

    fn argmax(&self) -> Option<(Self::Index, Self::Item)>
    where
        Self::Item: PartialOrd,
    {
        self.fold_indexed(None, |acc, index, value| {
            Some(match acc
            {
                Some((best_index, best_value)) if best_value >= value => (best_index, best_value),
                _ => (index, value),
            })
        })
    }
}
//}}}
//{{{ enum
pub enum Dimension
{
    Rows,
    Cols,
    All,
}
//}}}
