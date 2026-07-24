//! Floating-point scalar traits and element-wise transform operations.
//!
//! Defines [`Float`], the extension of [`Field`](crate::common::Field) with the full suite of
//! floating-point methods (trigonometric, exponential, logarithmic, rounding, etc.).
//! [`FloatVectorOps`] and [`FloatTransformOps`] build on top of [`Float`] to provide
//! element-wise and geometric operations for vector and matrix types.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, One, TransformOps, VectorOps, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: Float
/// Extends [`Field`](crate::common::Field) with the full suite of floating-point mathematical
/// operations required by numeric algorithms in this crate (trigonometric, exponential,
/// logarithmic, rounding, etc.).
pub trait Float: Field + PartialOrd {
    /// Returns the absolute value.
    fn abs(self) -> Self;
    /// Returns the positive difference between `self` and `other`.
    fn abs_sub(
        self,
        other: Self,
    ) -> Self;
    /// Computes the inverse cosine.
    fn acos(self) -> Self;
    /// Computes the inverse hyperbolic cosine.
    fn acosh(self) -> Self;
    /// Computes the inverse sine.
    fn asin(self) -> Self;
    /// Computes the inverse hyperbolic sine.
    fn asinh(self) -> Self;
    /// Computes the inverse tangent.
    fn atan(self) -> Self;
    /// Computes the four-quadrant inverse tangent of `self` and `other`.
    fn atan2(
        self,
        other: Self,
    ) -> Self;
    /// Computes the inverse hyperbolic tangent.
    fn atanh(self) -> Self;
    /// Computes the cube root.
    fn cbrt(self) -> Self;
    /// Returns the smallest integer greater than or equal to `self`.
    fn ceil(self) -> Self;
    /// Restricts `self` to the inclusive range `min..=max`.
    fn clamp(
        self,
        min: Self,
        max: Self,
    ) -> Self;
    /// Returns the magnitude of `self` with the sign of `sign`.
    fn copysign(
        self,
        sign: Self,
    ) -> Self;
    /// Computes the cosine.
    fn cos(self) -> Self;
    /// Computes the hyperbolic cosine.
    fn cosh(self) -> Self;
    /// Computes Euclidean division.
    fn div_euclid(
        self,
        rhs: Self,
    ) -> Self;
    /// Computes `e` raised to `self`.
    fn exp(self) -> Self;
    /// Computes 2 raised to `self`.
    fn exp2(self) -> Self;
    /// Computes `e` raised to `self`, minus one.
    fn exp_m1(self) -> Self;
    /// Returns the largest integer less than or equal to `self`.
    fn floor(self) -> Self;
    /// Returns the fractional part.
    fn fract(self) -> Self;
    /// Computes the length of the hypotenuse formed with `other`.
    fn hypot(
        self,
        other: Self,
    ) -> Self;
    /// Computes the natural logarithm.
    fn ln(self) -> Self;
    /// Computes the natural logarithm of one plus `self`.
    fn ln_1p(self) -> Self;
    /// Computes the logarithm in the specified base.
    fn log(
        self,
        base: Self,
    ) -> Self;
    /// Computes the base-10 logarithm.
    fn log10(self) -> Self;
    /// Computes the base-2 logarithm.
    fn log2(self) -> Self;
    /// Returns the greater of `self` and `other`.
    fn max(
        self,
        other: Self,
    ) -> Self;
    /// Computes the midpoint between `self` and `other`.
    fn midpoint(
        self,
        other: Self,
    ) -> Self;
    /// Returns the lesser of `self` and `other`.
    fn min(
        self,
        other: Self,
    ) -> Self;
    /// Computes `(self * a) + b`, permitting a fused implementation.
    fn mul_add(
        self,
        a: Self,
        b: Self,
    ) -> Self;
    /// Returns the next representable value toward negative infinity.
    fn next_down(self) -> Self;
    /// Returns the next representable value toward positive infinity.
    fn next_up(self) -> Self;
    /// Raises `self` to a floating-point power.
    fn powf(
        self,
        exp: Self,
    ) -> Self;
    /// Returns the tolerance used for approximate zero checks.
    fn small() -> Self;
    /// Raises `self` to an integer power.
    fn powi(
        self,
        exp: i32,
    ) -> Self;
    /// Computes the reciprocal.
    fn recip(self) -> Self;
    /// Computes the least nonnegative Euclidean remainder.
    fn rem_euclid(
        self,
        rhs: Self,
    ) -> Self;
    /// Rounds to the nearest integer, with halfway cases away from zero.
    fn round(self) -> Self;
    /// Rounds to the nearest integer, with halfway cases to the even integer.
    fn round_ties_even(self) -> Self;
    /// Returns a value representing the sign of `self`.
    fn signum(self) -> Self;
    /// Computes the sine.
    fn sin(self) -> Self;
    /// Computes the hyperbolic sine.
    fn sinh(self) -> Self;
    /// Computes the square root.
    fn sqrt(self) -> Self;
    /// Computes the tangent.
    fn tan(self) -> Self;
    /// Computes the hyperbolic tangent.
    fn tanh(self) -> Self;
    /// Converts radians to degrees.
    fn to_degrees(self) -> Self;
    /// Converts degrees to radians.
    fn to_radians(self) -> Self;
    /// Returns the integer part of `self`.
    fn trunc(self) -> Self;
}
//}}}
//{{{ macro: impl_float
macro_rules! impl_float {
    ($type:ty) => {
        impl Float for $type {
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline]
            fn abs_sub(
                self,
                other: Self,
            ) -> Self {
                #[allow(deprecated)]
                {
                    self.abs_sub(other)
                }
            }

            #[inline]
            fn acos(self) -> Self {
                self.acos()
            }

            #[inline]
            fn acosh(self) -> Self {
                self.acosh()
            }

            #[inline]
            fn asin(self) -> Self {
                self.asin()
            }

            #[inline]
            fn asinh(self) -> Self {
                self.asinh()
            }

            #[inline]
            fn atan(self) -> Self {
                self.atan()
            }

            #[inline]
            fn atan2(
                self,
                other: Self,
            ) -> Self {
                self.atan2(other)
            }

            #[inline]
            fn atanh(self) -> Self {
                self.atanh()
            }

            #[inline]
            fn cbrt(self) -> Self {
                self.cbrt()
            }

            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[inline]
            fn clamp(
                self,
                min: Self,
                max: Self,
            ) -> Self {
                <$type>::clamp(self, min, max)
            }

            #[inline]
            fn copysign(
                self,
                sign: Self,
            ) -> Self {
                self.copysign(sign)
            }

            #[inline]
            fn cos(self) -> Self {
                self.cos()
            }

            #[inline]
            fn cosh(self) -> Self {
                self.cosh()
            }

            #[inline]
            fn div_euclid(
                self,
                rhs: Self,
            ) -> Self {
                self.div_euclid(rhs)
            }

            #[inline]
            fn exp(self) -> Self {
                self.exp()
            }

            #[inline]
            fn exp2(self) -> Self {
                self.exp2()
            }

            #[inline]
            fn exp_m1(self) -> Self {
                self.exp_m1()
            }

            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }

            #[inline]
            fn fract(self) -> Self {
                self.fract()
            }

            #[inline]
            fn hypot(
                self,
                other: Self,
            ) -> Self {
                self.hypot(other)
            }

            #[inline]
            fn ln(self) -> Self {
                self.ln()
            }

            #[inline]
            fn ln_1p(self) -> Self {
                self.ln_1p()
            }

            #[inline]
            fn log(
                self,
                base: Self,
            ) -> Self {
                self.log(base)
            }

            #[inline]
            fn log10(self) -> Self {
                self.log10()
            }

            #[inline]
            fn log2(self) -> Self {
                self.log2()
            }

            #[inline]
            fn max(
                self,
                other: Self,
            ) -> Self {
                self.max(other)
            }

            #[inline]
            fn midpoint(
                self,
                other: Self,
            ) -> Self {
                self.midpoint(other)
            }

            #[inline]
            fn min(
                self,
                other: Self,
            ) -> Self {
                self.min(other)
            }

            #[inline]
            fn mul_add(
                self,
                a: Self,
                b: Self,
            ) -> Self {
                self.mul_add(a, b)
            }

            #[inline]
            fn next_down(self) -> Self {
                self.next_down()
            }

            #[inline]
            fn next_up(self) -> Self {
                self.next_up()
            }

            #[inline]
            fn powf(
                self,
                exp: Self,
            ) -> Self {
                self.powf(exp)
            }

            #[inline]
            fn powi(
                self,
                exp: i32,
            ) -> Self {
                self.powi(exp)
            }

            #[inline]
            fn recip(self) -> Self {
                self.recip()
            }

            #[inline]
            fn rem_euclid(
                self,
                rhs: Self,
            ) -> Self {
                self.rem_euclid(rhs)
            }

            #[inline]
            fn round(self) -> Self {
                self.round()
            }

            #[inline]
            fn round_ties_even(self) -> Self {
                self.round_ties_even()
            }

            #[inline]
            fn signum(self) -> Self {
                self.signum()
            }

            #[inline]
            fn sin(self) -> Self {
                self.sin()
            }

            #[inline]
            fn sinh(self) -> Self {
                self.sinh()
            }

            #[inline]
            fn small() -> Self {
                <$type>::EPSILON
            }

            #[inline]
            fn sqrt(self) -> Self {
                self.sqrt()
            }

            #[inline]
            fn tan(self) -> Self {
                self.tan()
            }

            #[inline]
            fn tanh(self) -> Self {
                self.tanh()
            }

            #[inline]
            fn to_degrees(self) -> Self {
                self.to_degrees()
            }

            #[inline]
            fn to_radians(self) -> Self {
                self.to_radians()
            }

            #[inline]
            fn trunc(self) -> Self {
                self.trunc()
            }
        }
    };
}
//}}}
//{{{ collection: impl_float implementations
impl_float!(f32);
impl_float!(f64);
//}}}
//{{{ trait: FloatVectorOps
/// Extends [`VectorOps`] with operations that require floating-point scalars, such as computing
/// the angle between two vectors.
pub trait FloatVectorOps: VectorOps
where
    Self::ScalarType: Float + Zero + One + Copy + Default,
{
    //{{{ fn: angle
    /// Computes the angle in radians between two two- or three-dimensional vectors.
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different lengths, are not two- or three-dimensional, or
    /// either vector has approximately zero length.
    fn angle(
        &self,
        other: &Self,
    ) -> Self::ScalarType {
        if self.len() != other.len() {
            panic!("vectors must be of the same length");
        }

        if self.len() != 2 && self.len() != 3 {
            panic!("angle is only defined for 2D and 3D vectors");
        }

        if self.norm() < Self::ScalarType::small() || other.norm() < Self::ScalarType::small() {
            panic!("cannot compute angle with zero vector");
        }

        let a = self.normalize();
        let b = other.normalize();
        let dot = (a.dot(&b)).clamp(-Self::ScalarType::one(), Self::ScalarType::one());
        dot.acos()
    }
    //}}}
}
//}}}
//{{{ macro: float_transform_unary
macro_rules! float_transform_unary {
    ($method:ident, $method_mut:ident, $to_method:ident, $into_method:ident) => {
        #[doc = concat!("Applies [`Float::", stringify!($method), "`] to every element in place.")]
        fn $method_mut(&mut self) {
            self.transform_mut(|value| value.$method());
        }

        #[doc = concat!(
                                                            "Returns a cloned value with [`Float::",
                                                            stringify!($method),
                                                            "`] applied to every element."
                                                        )]
        fn $to_method(&self) -> Self
        where
            Self: Clone,
        {
            self.to_transformed(|value| value.$method())
        }

        #[doc = concat!(
                                                            "Consumes `self` and applies [`Float::",
                                                            stringify!($method),
                                                            "`] to every element."
                                                        )]
        fn $into_method(self) -> Self {
            self.into_transformed(|value| value.$method())
        }
    };
}
//}}}
//{{{ macro: float_transform_unary_with_arg
macro_rules! float_transform_unary_with_arg {
    ($method:ident, $method_mut:ident, $to_method:ident, $into_method:ident, $arg:ident: $arg_type:ty) => {
        #[doc = concat!("Applies [`Float::", stringify!($method), "`] to every element in place.")]
        fn $method_mut(
            &mut self,
            $arg: $arg_type,
        ) {
            self.transform_mut(|value| value.$method($arg));
        }

        #[doc = concat!(
                                                            "Returns a cloned value with [`Float::",
                                                            stringify!($method),
                                                            "`] applied to every element."
                                                        )]
        fn $to_method(
            &self,
            $arg: $arg_type,
        ) -> Self
        where
            Self: Clone,
        {
            self.to_transformed(|value| value.$method($arg))
        }

        #[doc = concat!(
                                                            "Consumes `self` and applies [`Float::",
                                                            stringify!($method),
                                                            "`] to every element."
                                                        )]
        fn $into_method(
            self,
            $arg: $arg_type,
        ) -> Self {
            self.into_transformed(|value| value.$method($arg))
        }
    };
}
//}}}
//{{{ macro: float_transform_unary_with_two_args
macro_rules! float_transform_unary_with_two_args {
    (
        $method:ident,
        $method_mut:ident,
        $to_method:ident,
        $into_method:ident,
        $arg1:ident: $arg1_type:ty,
        $arg2:ident: $arg2_type:ty
    ) => {
        #[doc = concat!("Applies [`Float::", stringify!($method), "`] to every element in place.")]
        fn $method_mut(
            &mut self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        ) {
            self.transform_mut(|value| value.$method($arg1, $arg2));
        }

        #[doc = concat!(
                                                            "Returns a cloned value with [`Float::",
                                                            stringify!($method),
                                                            "`] applied to every element."
                                                        )]
        fn $to_method(
            &self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        ) -> Self
        where
            Self: Clone,
        {
            self.to_transformed(|value| value.$method($arg1, $arg2))
        }

        #[doc = concat!(
                                                            "Consumes `self` and applies [`Float::",
                                                            stringify!($method),
                                                            "`] to every element."
                                                        )]
        fn $into_method(
            self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        ) -> Self {
            self.into_transformed(|value| value.$method($arg1, $arg2))
        }
    };
}
//}}}
//{{{ trait: FloatTransformOps
/// Extends [`TransformOps`] with element-wise versions of every [`Float`] method, providing
/// in-place (`*_mut`), cloning (`to_*`), and consuming (`into_*`) variants for each operation.
pub trait FloatTransformOps: TransformOps
where
    Self::ScalarType: Float,
{
    float_transform_unary!(abs, abs_mut, to_abs, into_abs);
    float_transform_unary_with_arg!(abs_sub, abs_sub_mut, to_abs_sub, into_abs_sub, other: Self::ScalarType);
    float_transform_unary!(acos, acos_mut, to_acos, into_acos);
    float_transform_unary!(acosh, acosh_mut, to_acosh, into_acosh);
    float_transform_unary!(asin, asin_mut, to_asin, into_asin);
    float_transform_unary!(asinh, asinh_mut, to_asinh, into_asinh);
    float_transform_unary!(atan, atan_mut, to_atan, into_atan);
    float_transform_unary_with_arg!(atan2, atan2_mut, to_atan2, into_atan2, other: Self::ScalarType);
    float_transform_unary!(atanh, atanh_mut, to_atanh, into_atanh);
    float_transform_unary!(cbrt, cbrt_mut, to_cbrt, into_cbrt);
    float_transform_unary!(ceil, ceil_mut, to_ceil, into_ceil);
    float_transform_unary_with_two_args!(
        clamp,
        clamp_mut,
        to_clamp,
        into_clamp,
        min: Self::ScalarType,
        max: Self::ScalarType
    );
    float_transform_unary_with_arg!(copysign, copysign_mut, to_copysign, into_copysign, sign: Self::ScalarType);
    float_transform_unary!(cos, cos_mut, to_cos, into_cos);
    float_transform_unary!(cosh, cosh_mut, to_cosh, into_cosh);
    float_transform_unary_with_arg!(div_euclid, div_euclid_mut, to_div_euclid, into_div_euclid, rhs: Self::ScalarType);
    float_transform_unary!(exp, exp_mut, to_exp, into_exp);
    float_transform_unary!(exp2, exp2_mut, to_exp2, into_exp2);
    float_transform_unary!(exp_m1, exp_m1_mut, to_exp_m1, into_exp_m1);
    float_transform_unary!(floor, floor_mut, to_floor, into_floor);
    float_transform_unary!(fract, fract_mut, to_fract, into_fract);
    float_transform_unary_with_arg!(hypot, hypot_mut, to_hypot, into_hypot, other: Self::ScalarType);
    float_transform_unary!(ln, ln_mut, to_ln, into_ln);
    float_transform_unary!(ln_1p, ln_1p_mut, to_ln_1p, into_ln_1p);
    float_transform_unary_with_arg!(log, log_mut, to_log, into_log, base: Self::ScalarType);
    float_transform_unary!(log10, log10_mut, to_log10, into_log10);
    float_transform_unary!(log2, log2_mut, to_log2, into_log2);
    float_transform_unary_with_arg!(max, max_mut, to_max, into_max, other: Self::ScalarType);
    float_transform_unary_with_arg!(midpoint, midpoint_mut, to_midpoint, into_midpoint, other: Self::ScalarType);
    float_transform_unary_with_arg!(min, min_mut, to_min, into_min, other: Self::ScalarType);
    float_transform_unary_with_two_args!(
        mul_add,
        mul_add_mut,
        to_mul_add,
        into_mul_add,
        a: Self::ScalarType,
        b: Self::ScalarType
    );
    float_transform_unary!(next_down, next_down_mut, to_next_down, into_next_down);
    float_transform_unary!(next_up, next_up_mut, to_next_up, into_next_up);
    float_transform_unary_with_arg!(powf, powf_mut, to_powf, into_powf, exp: Self::ScalarType);
    float_transform_unary_with_arg!(powi, powi_mut, to_powi, into_powi, exp: i32);
    float_transform_unary!(recip, recip_mut, to_recip, into_recip);
    float_transform_unary_with_arg!(rem_euclid, rem_euclid_mut, to_rem_euclid, into_rem_euclid, rhs: Self::ScalarType);
    float_transform_unary!(round, round_mut, to_round, into_round);
    float_transform_unary!(
        round_ties_even,
        round_ties_even_mut,
        to_round_ties_even,
        into_round_ties_even
    );
    float_transform_unary!(signum, signum_mut, to_signum, into_signum);
    float_transform_unary!(sin, sin_mut, to_sin, into_sin);
    float_transform_unary!(sinh, sinh_mut, to_sinh, into_sinh);
    float_transform_unary!(sqrt, sqrt_mut, to_sqrt, into_sqrt);
    float_transform_unary!(tan, tan_mut, to_tan, into_tan);
    float_transform_unary!(tanh, tanh_mut, to_tanh, into_tanh);
    float_transform_unary!(to_degrees, to_degrees_mut, to_degrees, into_degrees);
    float_transform_unary!(to_radians, to_radians_mut, to_radians, into_radians);
    float_transform_unary!(trunc, trunc_mut, to_trunc, into_trunc);

    /// Clamps every element to the positive part in-place (negative values become zero).
    fn pos_mut(&mut self)
    where
        Self::ScalarType: Zero,
    {
        self.transform_mut(|value| {
            if value > Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        });
    }

    /// Returns a cloned copy of `self` with every element clamped to its positive part.
    fn to_pos(&self) -> Self
    where
        Self: Clone,
        Self::ScalarType: Zero,
    {
        self.to_transformed(|value| {
            if value > Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        })
    }

    /// Consumes `self`, clamps every element to its positive part, and returns it.
    fn into_pos(mut self) -> Self
    where
        Self::ScalarType: Zero,
    {
        self.pos_mut();
        self
    }

    /// Clamps every element to the non-positive part in-place (positive values become zero).
    fn neg_mut(&mut self)
    where
        Self::ScalarType: Zero,
    {
        self.transform_mut(|value| {
            if value <= Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        });
    }

    /// Returns a cloned copy of `self` with every element clamped to its non-positive part.
    fn to_neg(&self) -> Self
    where
        Self: Clone,
        Self::ScalarType: Zero,
    {
        self.to_transformed(|value| {
            if value <= Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        })
    }

    /// Consumes `self`, clamps every element to its non-positive part, and returns it.
    fn into_neg(mut self) -> Self
    where
        Self::ScalarType: Zero,
    {
        self.neg_mut();
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
