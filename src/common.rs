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
    fn abs(self) -> Self;
    fn abs_sub(
        self,
        other: Self,
    ) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(
        self,
        other: Self,
    ) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn clamp(
        self,
        min: Self,
        max: Self,
    ) -> Self;
    fn clamp_magnitude(
        self,
        limit: Self,
    ) -> Self;
    fn copysign(
        self,
        sign: Self,
    ) -> Self;
    fn cos(self) -> Self;
    fn cosh(self) -> Self;
    fn div_euclid(
        self,
        rhs: Self,
    ) -> Self;
    fn erf(self) -> Self;
    fn erfc(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
    fn gamma(self) -> Self;
    fn hypot(
        self,
        other: Self,
    ) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log(
        self,
        base: Self,
    ) -> Self;
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn max(
        self,
        other: Self,
    ) -> Self;
    fn maximum(
        self,
        other: Self,
    ) -> Self;
    fn midpoint(
        self,
        other: Self,
    ) -> Self;
    fn min(
        self,
        other: Self,
    ) -> Self;
    fn minimum(
        self,
        other: Self,
    ) -> Self;
    fn mul_add(
        self,
        a: Self,
        b: Self,
    ) -> Self;
    fn next_down(self) -> Self;
    fn next_up(self) -> Self;
    fn powf(
        self,
        exp: Self,
    ) -> Self;
    fn small() -> Self;
    fn powi(
        self,
        exp: i32,
    ) -> Self;
    fn recip(self) -> Self;
    fn rem_euclid(
        self,
        rhs: Self,
    ) -> Self;
    fn round(self) -> Self;
    fn round_ties_even(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn trunc(self) -> Self;

    fn algebraic_add(
        self,
        rhs: Self,
    ) -> Self;
    fn algebraic_sub(
        self,
        rhs: Self,
    ) -> Self;
    fn algebraic_mul(
        self,
        rhs: Self,
    ) -> Self;
    fn algebraic_div(
        self,
        rhs: Self,
    ) -> Self;
    fn algebraic_rem(
        self,
        rhs: Self,
    ) -> Self;
}
//}}}
//{{{ macro: impl_float
macro_rules! impl_float {
    ($type:ty) => {
        impl Float for $type
        {
            #[inline]
            fn abs(self) -> Self
            {
                self.abs()
            }

            #[inline]
            fn abs_sub(
                self,
                other: Self,
            ) -> Self
            {
                #[allow(deprecated)]
                {
                    self.abs_sub(other)
                }
            }

            #[inline]
            fn acos(self) -> Self
            {
                self.acos()
            }

            #[inline]
            fn acosh(self) -> Self
            {
                self.acosh()
            }

            #[inline]
            fn asin(self) -> Self
            {
                self.asin()
            }

            #[inline]
            fn asinh(self) -> Self
            {
                self.asinh()
            }

            #[inline]
            fn atan(self) -> Self
            {
                self.atan()
            }

            #[inline]
            fn atan2(
                self,
                other: Self,
            ) -> Self
            {
                self.atan2(other)
            }

            #[inline]
            fn atanh(self) -> Self
            {
                self.atanh()
            }

            #[inline]
            fn cbrt(self) -> Self
            {
                self.cbrt()
            }

            #[inline]
            fn ceil(self) -> Self
            {
                self.ceil()
            }

            #[inline]
            fn clamp(
                self,
                min: Self,
                max: Self,
            ) -> Self
            {
                <$type>::clamp(self, min, max)
            }

            #[inline]
            fn clamp_magnitude(
                self,
                limit: Self,
            ) -> Self
            {
                self.clamp_magnitude(limit)
            }

            #[inline]
            fn copysign(
                self,
                sign: Self,
            ) -> Self
            {
                self.copysign(sign)
            }

            #[inline]
            fn cos(self) -> Self
            {
                self.cos()
            }

            #[inline]
            fn cosh(self) -> Self
            {
                self.cosh()
            }

            #[inline]
            fn div_euclid(
                self,
                rhs: Self,
            ) -> Self
            {
                self.div_euclid(rhs)
            }

            #[inline]
            fn erf(self) -> Self
            {
                self.erf()
            }

            #[inline]
            fn erfc(self) -> Self
            {
                self.erfc()
            }

            #[inline]
            fn exp(self) -> Self
            {
                self.exp()
            }

            #[inline]
            fn exp2(self) -> Self
            {
                self.exp2()
            }

            #[inline]
            fn exp_m1(self) -> Self
            {
                self.exp_m1()
            }

            #[inline]
            fn floor(self) -> Self
            {
                self.floor()
            }

            #[inline]
            fn fract(self) -> Self
            {
                self.fract()
            }

            #[inline]
            fn gamma(self) -> Self
            {
                self.gamma()
            }

            #[inline]
            fn hypot(
                self,
                other: Self,
            ) -> Self
            {
                self.hypot(other)
            }

            #[inline]
            fn ln(self) -> Self
            {
                self.ln()
            }

            #[inline]
            fn ln_1p(self) -> Self
            {
                self.ln_1p()
            }

            #[inline]
            fn log(
                self,
                base: Self,
            ) -> Self
            {
                self.log(base)
            }

            #[inline]
            fn log10(self) -> Self
            {
                self.log10()
            }

            #[inline]
            fn log2(self) -> Self
            {
                self.log2()
            }

            #[inline]
            fn max(
                self,
                other: Self,
            ) -> Self
            {
                self.max(other)
            }

            #[inline]
            fn maximum(
                self,
                other: Self,
            ) -> Self
            {
                self.maximum(other)
            }

            #[inline]
            fn midpoint(
                self,
                other: Self,
            ) -> Self
            {
                self.midpoint(other)
            }

            #[inline]
            fn min(
                self,
                other: Self,
            ) -> Self
            {
                self.min(other)
            }

            #[inline]
            fn minimum(
                self,
                other: Self,
            ) -> Self
            {
                self.minimum(other)
            }

            #[inline]
            fn mul_add(
                self,
                a: Self,
                b: Self,
            ) -> Self
            {
                self.mul_add(a, b)
            }

            #[inline]
            fn next_down(self) -> Self
            {
                self.next_down()
            }

            #[inline]
            fn next_up(self) -> Self
            {
                self.next_up()
            }

            #[inline]
            fn powf(
                self,
                exp: Self,
            ) -> Self
            {
                self.powf(exp)
            }

            #[inline]
            fn powi(
                self,
                exp: i32,
            ) -> Self
            {
                self.powi(exp)
            }

            #[inline]
            fn recip(self) -> Self
            {
                self.recip()
            }

            #[inline]
            fn rem_euclid(
                self,
                rhs: Self,
            ) -> Self
            {
                self.rem_euclid(rhs)
            }

            #[inline]
            fn round(self) -> Self
            {
                self.round()
            }

            #[inline]
            fn round_ties_even(self) -> Self
            {
                self.round_ties_even()
            }

            #[inline]
            fn signum(self) -> Self
            {
                self.signum()
            }

            #[inline]
            fn sin(self) -> Self
            {
                self.sin()
            }

            #[inline]
            fn sinh(self) -> Self
            {
                self.sinh()
            }

            #[inline]
            fn small() -> Self
            {
                <$type>::EPSILON
            }

            #[inline]
            fn sqrt(self) -> Self
            {
                self.sqrt()
            }

            #[inline]
            fn tan(self) -> Self
            {
                self.tan()
            }

            #[inline]
            fn tanh(self) -> Self
            {
                self.tanh()
            }

            #[inline]
            fn to_degrees(self) -> Self
            {
                self.to_degrees()
            }

            #[inline]
            fn to_radians(self) -> Self
            {
                self.to_radians()
            }

            #[inline]
            fn trunc(self) -> Self
            {
                self.trunc()
            }

            #[inline]
            fn algebraic_add(
                self,
                rhs: Self,
            ) -> Self
            {
                self.algebraic_add(rhs)
            }

            #[inline]
            fn algebraic_sub(
                self,
                rhs: Self,
            ) -> Self
            {
                self.algebraic_sub(rhs)
            }

            #[inline]
            fn algebraic_mul(
                self,
                rhs: Self,
            ) -> Self
            {
                self.algebraic_mul(rhs)
            }

            #[inline]
            fn algebraic_div(
                self,
                rhs: Self,
            ) -> Self
            {
                self.algebraic_div(rhs)
            }

            #[inline]
            fn algebraic_rem(
                self,
                rhs: Self,
            ) -> Self
            {
                self.algebraic_rem(rhs)
            }
        }
    };
}
//}}}
//{{{ collection: impl_float implementations
impl_float!(f32);
impl_float!(f64);
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
//{{{ macro: float_transform_unary
macro_rules! float_transform_unary {
    ($method:ident, $methoded:ident, $into_methoded:ident) => {
        fn $method(&mut self)
        {
            self.transform(|value| value.$method());
        }

        fn $methoded(&self) -> Self
        where
            Self: Clone,
        {
            self.transformed(|value| value.$method())
        }

        fn $into_methoded(self) -> Self
        {
            self.into_transformed(|value| value.$method())
        }
    };
}
//}}}
//{{{ macro: float_transform_unary_with_arg
macro_rules! float_transform_unary_with_arg {
    ($method:ident, $methoded:ident, $into_methoded:ident, $arg:ident: $arg_type:ty) => {
        fn $method(
            &mut self,
            $arg: $arg_type,
        )
        {
            self.transform(|value| value.$method($arg));
        }

        fn $methoded(
            &self,
            $arg: $arg_type,
        ) -> Self
        where
            Self: Clone,
        {
            self.transformed(|value| value.$method($arg))
        }

        fn $into_methoded(
            self,
            $arg: $arg_type,
        ) -> Self
        {
            self.into_transformed(|value| value.$method($arg))
        }
    };
}
//}}}
//{{{ macro: float_transform_unary_with_two_args
macro_rules! float_transform_unary_with_two_args {
    (
        $method:ident,
        $methoded:ident,
        $into_methoded:ident,
        $arg1:ident: $arg1_type:ty,
        $arg2:ident: $arg2_type:ty
    ) => {
        fn $method(
            &mut self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        )
        {
            self.transform(|value| value.$method($arg1, $arg2));
        }

        fn $methoded(
            &self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        ) -> Self
        where
            Self: Clone,
        {
            self.transformed(|value| value.$method($arg1, $arg2))
        }

        fn $into_methoded(
            self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        ) -> Self
        {
            self.into_transformed(|value| value.$method($arg1, $arg2))
        }
    };
}
//}}}
//{{{ trait: FloatTransformOps
pub trait FloatTransformOps: TransformOps
where
    Self::ScalarType: Float,
{
    float_transform_unary!(abs, absed, into_absed);
    float_transform_unary_with_arg!(abs_sub, abs_subed, into_abs_subed, other: Self::ScalarType);
    float_transform_unary!(acos, acosed, into_acosed);
    float_transform_unary!(acosh, acoshed, into_acoshed);
    float_transform_unary!(asin, asined, into_asined);
    float_transform_unary!(asinh, asinhed, into_asinhed);
    float_transform_unary!(atan, ataned, into_ataned);
    float_transform_unary_with_arg!(atan2, atan2ed, into_atan2ed, other: Self::ScalarType);
    float_transform_unary!(atanh, atanhed, into_atanhed);
    float_transform_unary!(cbrt, cbrted, into_cbrted);
    float_transform_unary!(ceil, ceiled, into_ceiled);
    float_transform_unary_with_two_args!(
        clamp,
        clamped,
        into_clamped,
        min: Self::ScalarType,
        max: Self::ScalarType
    );
    float_transform_unary_with_arg!(
        clamp_magnitude,
        clamp_magnituded,
        into_clamp_magnituded,
        limit: Self::ScalarType
    );
    float_transform_unary_with_arg!(copysign, copysigned, into_copysigned, sign: Self::ScalarType);
    float_transform_unary!(cos, cosed, into_cosed);
    float_transform_unary!(cosh, coshed, into_coshed);
    float_transform_unary_with_arg!(div_euclid, div_euclided, into_div_euclided, rhs: Self::ScalarType);
    float_transform_unary!(erf, erfed, into_erfed);
    float_transform_unary!(erfc, erfced, into_erfced);
    float_transform_unary!(exp, exped, into_exped);
    float_transform_unary!(exp2, exp2ed, into_exp2ed);
    float_transform_unary!(exp_m1, exp_m1ed, into_exp_m1ed);
    float_transform_unary!(floor, floored, into_floored);
    float_transform_unary!(fract, fracted, into_fracted);
    float_transform_unary!(gamma, gammaed, into_gammaed);
    float_transform_unary_with_arg!(hypot, hypoted, into_hypoted, other: Self::ScalarType);
    float_transform_unary!(ln, lned, into_lned);
    float_transform_unary!(ln_1p, ln_1ped, into_ln_1ped);
    float_transform_unary_with_arg!(log, loged, into_loged, base: Self::ScalarType);
    float_transform_unary!(log10, log10ed, into_log10ed);
    float_transform_unary!(log2, log2ed, into_log2ed);
    float_transform_unary_with_arg!(max, maxed, into_maxed, other: Self::ScalarType);
    float_transform_unary_with_arg!(maximum, maximumed, into_maximumed, other: Self::ScalarType);
    float_transform_unary_with_arg!(midpoint, midpointed, into_midpointed, other: Self::ScalarType);
    float_transform_unary_with_arg!(min, mined, into_mined, other: Self::ScalarType);
    float_transform_unary_with_arg!(minimum, minimumed, into_minimumed, other: Self::ScalarType);
    float_transform_unary_with_two_args!(
        mul_add,
        mul_added,
        into_mul_added,
        a: Self::ScalarType,
        b: Self::ScalarType
    );
    float_transform_unary!(next_down, next_downed, into_next_downed);
    float_transform_unary!(next_up, next_uped, into_next_uped);
    float_transform_unary_with_arg!(powf, powfed, into_powfed, exp: Self::ScalarType);
    float_transform_unary_with_arg!(powi, powied, into_powied, exp: i32);
    float_transform_unary!(recip, reciped, into_reciped);
    float_transform_unary_with_arg!(rem_euclid, rem_euclided, into_rem_euclided, rhs: Self::ScalarType);
    float_transform_unary!(round, rounded, into_rounded);
    float_transform_unary!(round_ties_even, round_ties_evened, into_round_ties_evened);
    float_transform_unary!(signum, signumed, into_signumed);
    float_transform_unary!(sin, sined, into_sined);
    float_transform_unary!(sinh, sinhed, into_sinhed);
    float_transform_unary!(sqrt, sqrted, into_sqrted);
    float_transform_unary!(tan, taned, into_taned);
    float_transform_unary!(tanh, tanhed, into_tanhed);
    float_transform_unary!(to_degrees, to_degreesed, into_to_degreesed);
    float_transform_unary!(to_radians, to_radiansed, into_to_radiansed);
    float_transform_unary!(trunc, trunced, into_trunced);
    float_transform_unary_with_arg!(
        algebraic_add,
        algebraic_added,
        into_algebraic_added,
        rhs: Self::ScalarType
    );
    float_transform_unary_with_arg!(
        algebraic_sub,
        algebraic_subed,
        into_algebraic_subed,
        rhs: Self::ScalarType
    );
    float_transform_unary_with_arg!(
        algebraic_mul,
        algebraic_muled,
        into_algebraic_muled,
        rhs: Self::ScalarType
    );
    float_transform_unary_with_arg!(
        algebraic_div,
        algebraic_dived,
        into_algebraic_dived,
        rhs: Self::ScalarType
    );
    float_transform_unary_with_arg!(
        algebraic_rem,
        algebraic_remed,
        into_algebraic_remed,
        rhs: Self::ScalarType
    );

    fn pos(&mut self)
    where
        Self::ScalarType: Zero,
    {
        self.transform(|value| {
            if value > Self::ScalarType::zero()
            {
                value
            }
            else
            {
                Self::ScalarType::zero()
            }
        });
    }

    fn posed(&self) -> Self
    where
        Self: Clone,
        Self::ScalarType: Zero,
    {
        self.transformed(|value| {
            if value > Self::ScalarType::zero()
            {
                value
            }
            else
            {
                Self::ScalarType::zero()
            }
        })
    }

    fn into_posed(mut self) -> Self
    where
        Self::ScalarType: Zero,
    {
        self.pos();
        self
    }

    fn neg(&mut self)
    where
        Self::ScalarType: Zero,
    {
        self.transform(|value| {
            if value <= Self::ScalarType::zero()
            {
                value
            }
            else
            {
                Self::ScalarType::zero()
            }
        });
    }

    fn neged(&self) -> Self
    where
        Self: Clone,
        Self::ScalarType: Zero,
    {
        self.transformed(|value| {
            if value <= Self::ScalarType::zero()
            {
                value
            }
            else
            {
                Self::ScalarType::zero()
            }
        })
    }

    fn into_neged(mut self) -> Self
    where
        Self::ScalarType: Zero,
    {
        self.neg();
        self
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
        self.fold(None, |acc, value| {
            let abs_value = value.abs();
            Some(match acc
            {
                Some(current_min) if current_min <= abs_value => current_min,
                _ => abs_value,
            })
        })
    }

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
