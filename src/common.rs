//! Crate-wide foundational traits, types, and index utilities.
//!
//! Defines the primitive abstractions shared by all matrix types in the crate. The [`Field`]
//! trait bounds element types to those supporting arithmetic operations. [`Zero`] and [`One`]
//! supply additive and multiplicative identities. [`Shape`] exposes runtime matrix dimensions.
//! [`IndexValue`] and [`EvalInto`] power the lazy expression-template pipeline. [`LazyExpr`]
//! marks a type as a deferred computation. Compile-time predicates such as [`GreaterThan`] and
//! [`IsTrue`] enable dimension-checking assertions on static matrices.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::blaslapack::Getrf;
use crate::float::Float;
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

/// Compile-time predicate that asserts `N > M`, used to enforce static dimension constraints.
pub trait GreaterThan<const N: usize, const M: usize> {}
/// Blanket implementation that satisfies [`GreaterThan`] whenever the compile-time check `N > M` holds.
impl<const N: usize, const M: usize> GreaterThan<N, M> for () where Assert<{ N > M }>: IsTrue {}
//}}}
//{{{ trait: Field
/// Algebraic field over which matrix operations are defined, requiring the four arithmetic
/// operations, their assignment variants, negation, and a total order.
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
/// Provides element access by linear index, used by the lazy expression-template pipeline.
pub trait IndexValue<I>
{
    type Output;

    /// Returns the element at the given linear `index`.
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output;
}

impl<I, T> IndexValue<I> for &T
where
    T: IndexValue<I>,
{
    type Output = T::Output;

    #[inline]
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        (*self).index_value(index)
    }
}

impl<I, T> IndexValue<I> for &mut T
where
    T: IndexValue<I>,
{
    type Output = T::Output;

    #[inline]
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        (**self).index_value(index)
    }
}

//}}}
//{{{ macro: apply_for_all_types
#[macro_export]
#[doc(hidden)]
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
#[doc(hidden)]
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
//{{{ macro: impl_scalar_eval_into
macro_rules! impl_scalar_eval_into {
    ($type:ty) => {
        impl EvalInto<$type> for $type
        {
            #[inline]
            fn eval_into(
                &self,
                out: &mut [$type],
            )
            {
                out.fill(*self);
            }
        }
    };
}

apply_for_all_types!(impl_scalar_eval_into);
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
/// Supplies the additive identity element for a type.
pub trait Zero
{
    /// Returns the additive identity (zero) value for this type.
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
/// Supplies the multiplicative identity element for a type.
pub trait One
{
    /// Returns the multiplicative identity (one) value for this type.
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
/// Provides an absolute-value operation for scalar types.
pub trait Abs
{
    /// Returns the absolute value of `self`.
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
//{{{ trait: MatrixOps
/// Core linear-algebra operations common to all matrix types.
pub trait MatrixOps
where
    Self: Sized,
{
    type ScalarType: Field + Zero + One + Copy;
    type TransposeType;

    /// Returns the transpose of the matrix.
    fn transpose(&self) -> Self::TransposeType;
    /// Computes the determinant of the matrix using an LU factorisation.
    fn determinant(&self) -> Self::ScalarType
    where
        Self::ScalarType: Getrf + Float;
    /// Computes the trace (sum of diagonal elements) of the matrix.
    fn trace(&self) -> Self::ScalarType;
}
//}}}
//{{{ trait: Shape
/// Exposes the runtime dimensions of a two-dimensional array or matrix.
pub trait Shape
where
    Self: Sized,
{
    /// Returns the number of rows.
    fn nrows(&self) -> usize;
    /// Returns the number of columns.
    fn ncols(&self) -> usize;
    /// Returns `(nrows, ncols)` as a tuple.
    fn size(&self) -> (usize, usize)
    {
        (self.nrows(), self.ncols())
    }
}
//}}}
//{{{ trait: EvalInto
/// Evaluates `self` into a pre-allocated output slice.
///
/// Implementing types write their element-wise values into `out` in linear
/// (column-major) order.  The default implementation falls back to
/// `IndexValue` so leaf types only need to implement one path.
pub trait EvalInto<T: Field + Copy>
{
    fn eval_into(
        &self,
        out: &mut [T],
    );
}

impl<X, T: Field + Copy> EvalInto<T> for &X
where
    X: EvalInto<T>,
{
    #[inline]
    fn eval_into(
        &self,
        out: &mut [T],
    )
    {
        (**self).eval_into(out);
    }
}
//}}}
//{{{ trait: MatrixCopySource
/// Source of matrix values that can be copied into existing storage.
///
/// Implementors expose their values in column-major order.  Contiguous
/// destinations can use [`write_column_major`] directly; strided destinations
/// such as subviews can pull values with [`linear_value`] without allocating.
pub trait MatrixCopySource<T: Field + Copy>: Shape
{
    /// Returns the value at `index` in column-major order.
    fn linear_value(
        &self,
        index: usize,
    ) -> T;

    /// Writes all values into `out` in column-major order.
    fn write_column_major(
        &self,
        out: &mut [T],
    )
    {
        debug_assert_eq!(out.len(), self.nrows() * self.ncols());
        for i in 0..out.len()
        {
            unsafe {
                *out.get_unchecked_mut(i) = self.linear_value(i);
            }
        }
    }
}

impl<X, T> MatrixCopySource<T> for &X
where
    X: MatrixCopySource<T>,
    T: Field + Copy,
{
    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> T
    {
        (**self).linear_value(index)
    }

    #[inline]
    fn write_column_major(
        &self,
        out: &mut [T],
    )
    {
        (**self).write_column_major(out);
    }
}

impl<X, T> MatrixCopySource<T> for &mut X
where
    X: MatrixCopySource<T>,
    T: Field + Copy,
{
    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> T
    {
        (**self).linear_value(index)
    }

    #[inline]
    fn write_column_major(
        &self,
        out: &mut [T],
    )
    {
        (**self).write_column_major(out);
    }
}
//}}}
//{{{ trait: LazyExpr
/// Marker trait for types that represent a deferred (lazy) matrix computation.
///
/// Implementing types carry shape information via [`Shape`] and expose their
/// element type through `ScalarType`, enabling the expression-template pipeline
/// to compose operations without intermediate allocations.
pub trait LazyExpr: Shape
{
    type ScalarType: Field + Copy;
}

impl<T> LazyExpr for &T
where
    T: LazyExpr,
{
    type ScalarType = T::ScalarType;
}

impl<T> LazyExpr for &mut T
where
    T: LazyExpr,
{
    type ScalarType = T::ScalarType;
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
/// Matrix multiplication, consuming both operands and producing an output matrix.
pub trait MatMul<Rhs = Self>
where
    Self: Sized,
{
    type Output;

    /// Multiplies `self` by `rhs` and returns the resulting matrix.
    fn matmul(
        self,
        rhs: Rhs,
    ) -> Self::Output;
}
//}}}
//{{{ trait: VectorOps
/// Common geometric and algebraic operations for fixed- and dynamic-length vector types.
#[allow(clippy::len_without_is_empty)]
pub trait VectorOps:
    Index<usize, Output = Self::ScalarType> + IndexMut<usize, Output = Self::ScalarType> + Sized + Clone
{
    type ScalarType: Field + Zero + One + Copy + Default + Float;

    //{{{ fn: len
    /// Returns the number of elements in the vector.
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
    /// Computes the dot (inner) product of `self` and `other`.
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
    /// Returns a unit-length copy of the vector, or the zero vector if the norm is zero.
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
    /// Computes the cross product of two 3-D vectors.
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
//{{{ trait: TransformOps
/// Element-wise transformation operations for matrix and vector types, providing in-place,
/// cloning, and consuming variants for arbitrary closures as well as common special cases
/// (shift, scale, fill).
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

    /// Returns a cloned copy of `self` with every element shifted by `value`.
    fn shifted(
        &self,
        value: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|element| element + value)
    }

    /// Consumes `self`, shifts every element by `value`, and returns it.
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

    /// Returns a cloned copy of `self` with every element scaled by `value`.
    fn scaled(
        &self,
        value: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|element| element * value)
    }

    /// Consumes `self`, scales every element by `value`, and returns it.
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

    /// Returns a cloned copy of `self` with every element set to `value`.
    fn filled(
        &self,
        value: Self::ScalarType,
    ) -> Self
    where
        Self: Clone,
    {
        self.transformed(|_| value)
    }

    /// Consumes `self`, sets every element to `value`, and returns it.
    fn into_filled(
        self,
        value: Self::ScalarType,
    ) -> Self
    {
        self.into_transformed(|_| value)
    }
}
//}}}
//{{{ fun: lin_index
/// Converts a `(row, col)` index pair to a column-major linear index given `n` rows.
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
/// Converts a column-major linear index to a `(row, col)` pair given `nrows`.
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
/// Collection of reduction and aggregation operations built on a pair of primitive fold methods.
///
/// Implementors provide [`fold`](ReduceOps::fold) and [`fold_indexed`](ReduceOps::fold_indexed);
/// all other methods (`sum`, `product`, `min`, `max`, etc.) are derived automatically.
pub trait ReduceOps
{
    type Item: Copy;
    type Index: Copy;

    /// Folds every element into an accumulator using `f`, starting from `init`.
    fn fold<B, F>(
        &self,
        init: B,
        f: F,
    ) -> B
    where
        F: FnMut(B, Self::Item) -> B;

    /// Folds every element together with its index into an accumulator using `f`.
    fn fold_indexed<B, F>(
        &self,
        init: B,
        f: F,
    ) -> B
    where
        F: FnMut(B, Self::Index, Self::Item) -> B;

    /// Returns `true` if the collection contains no elements.
    fn is_empty(&self) -> bool
    {
        !self.fold(false, |_, _| true)
    }

    /// Reduces the collection to a single value using `f`, returning `None` if empty.
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

    /// Returns the sum of all elements, starting from the additive identity.
    fn sum(&self) -> Self::Item
    where
        Self::Item: Add<Output = Self::Item> + Zero,
    {
        self.fold(Self::Item::zero(), |acc, value| acc + value)
    }

    /// Returns the product of all elements, starting from the multiplicative identity.
    fn product(&self) -> Self::Item
    where
        Self::Item: Mul<Output = Self::Item> + One,
    {
        self.fold(Self::Item::one(), |acc, value| acc * value)
    }

    /// Returns the minimum element, or `None` if the collection is empty.
    fn allmin(&self) -> Option<Self::Item>
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

    /// Returns the maximum element, or `None` if the collection is empty.
    fn allmax(&self) -> Option<Self::Item>
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

    /// Returns the element with the smallest key according to `key`, or `None` if empty.
    fn allmin_by_key<K, F>(
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

    /// Returns the element with the largest key according to `key`, or `None` if empty.
    fn allmax_by_key<K, F>(
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

    /// Returns the element whose value after applying `transform` is the smallest.
    fn transform_min<F>(
        &self,
        transform: F,
    ) -> Option<Self::Item>
    where
        Self::Item: PartialOrd,
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.allmin_by_key(transform)
    }

    /// Returns the element whose value after applying `transform` is the largest.
    fn transform_max<F>(
        &self,
        transform: F,
    ) -> Option<Self::Item>
    where
        Self::Item: PartialOrd,
        F: FnMut(Self::Item) -> Self::Item,
    {
        self.allmax_by_key(transform)
    }

    /// Returns the element with the smallest absolute value, or `None` if empty.
    fn abs_min(&self) -> Option<Self::Item>
    where
        Self::Item: Abs + PartialOrd,
    {
        self.fold(None, |acc, value| {
            let abs_value = value.abs();
            Some(match acc
            {
                Some(current_min) if current_min <= abs_value => current_min,
                _ => abs_value,
            })
        })
    }

    /// Returns the element with the largest absolute value, or `None` if empty.
    fn abs_max(&self) -> Option<Self::Item>
    where
        Self::Item: Abs + PartialOrd,
    {
        self.fold(None, |acc, value| {
            let abs_value = value.abs();
            Some(match acc
            {
                Some(current_max) if current_max >= abs_value => current_max,
                _ => abs_value,
            })
        })
    }

    /// Returns the index and value of the minimum element, or `None` if empty.
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

    /// Returns the index and value of the maximum element, or `None` if empty.
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
//{{{ enum: Dimension
/// Selects which axis (or both axes) a reduction or transform operates over.
pub enum Dimension
{
    /// Operate along the row axis (i.e., reduce or transform each column).
    Rows,
    /// Operate along the column axis (i.e., reduce or transform each row).
    Cols,
    /// Operate over all elements regardless of axis.
    All,
}
//}}}
