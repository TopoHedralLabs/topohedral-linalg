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
pub trait Float: Field {
    /// Returns the absolute value.
    fn abs(self) -> Self;
    /// Returns the positive difference from `other`.
    fn abs_sub(
        self,
        other: Self,
    ) -> Self;
    /// Returns the inverse cosine.
    fn acos(self) -> Self;
    /// Returns the inverse hyperbolic cosine.
    fn acosh(self) -> Self;
    /// Returns the inverse sine.
    fn asin(self) -> Self;
    /// Returns the inverse hyperbolic sine.
    fn asinh(self) -> Self;
    /// Returns the inverse tangent.
    fn atan(self) -> Self;
    /// Returns the four-quadrant inverse tangent with `other`.
    fn atan2(
        self,
        other: Self,
    ) -> Self;
    /// Returns the inverse hyperbolic tangent.
    fn atanh(self) -> Self;
    /// Returns the cube root.
    fn cbrt(self) -> Self;
    /// Rounds toward positive infinity.
    fn ceil(self) -> Self;
    /// Restricts the value to the inclusive range `min..=max`.
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
    /// Returns the cosine.
    fn cos(self) -> Self;
    /// Returns the hyperbolic cosine.
    fn cosh(self) -> Self;
    /// Returns the Euclidean quotient of `self` by `rhs`.
    fn div_euclid(
        self,
        rhs: Self,
    ) -> Self;
    /// Returns the exponential function.
    fn exp(self) -> Self;
    /// Returns two raised to this value.
    fn exp2(self) -> Self;
    /// Returns `exp(self) - 1` accurately near zero.
    fn exp_m1(self) -> Self;
    /// Rounds toward negative infinity.
    fn floor(self) -> Self;
    /// Returns the fractional part.
    fn fract(self) -> Self;
    /// Returns the length of the hypotenuse formed with `other`.
    fn hypot(
        self,
        other: Self,
    ) -> Self;
    /// Returns the natural logarithm.
    fn ln(self) -> Self;
    /// Returns `ln(1 + self)` accurately near zero.
    fn ln_1p(self) -> Self;
    /// Returns the logarithm with the given base.
    fn log(
        self,
        base: Self,
    ) -> Self;
    /// Returns the base-10 logarithm.
    fn log10(self) -> Self;
    /// Returns the base-2 logarithm.
    fn log2(self) -> Self;
    /// Returns the greater of `self` and `other`.
    fn max(
        self,
        other: Self,
    ) -> Self;
    /// Returns the midpoint between `self` and `other`.
    fn midpoint(
        self,
        other: Self,
    ) -> Self;
    /// Returns the lesser of `self` and `other`.
    fn min(
        self,
        other: Self,
    ) -> Self;
    /// Returns `self * a + b` with a fused operation when available.
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
    /// Returns a small positive tolerance for this type.
    fn small() -> Self;
    /// Raises `self` to an integer power.
    fn powi(
        self,
        exp: i32,
    ) -> Self;
    /// Returns the reciprocal.
    fn recip(self) -> Self;
    /// Returns the Euclidean remainder of division by `rhs`.
    fn rem_euclid(
        self,
        rhs: Self,
    ) -> Self;
    /// Rounds to the nearest integer, away from zero on ties.
    fn round(self) -> Self;
    /// Rounds to the nearest integer, toward even on ties.
    fn round_ties_even(self) -> Self;
    /// Returns a value representing the sign of `self`.
    fn signum(self) -> Self;
    /// Returns the sine.
    fn sin(self) -> Self;
    /// Returns the hyperbolic sine.
    fn sinh(self) -> Self;
    /// Returns the square root.
    fn sqrt(self) -> Self;
    /// Returns the tangent.
    fn tan(self) -> Self;
    /// Returns the hyperbolic tangent.
    fn tanh(self) -> Self;
    /// Converts radians to degrees.
    fn to_degrees(self) -> Self;
    /// Converts degrees to radians.
    fn to_radians(self) -> Self;
    /// Returns the integer part by rounding toward zero.
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
    /// Returns the angle between two 2-D or 3-D vectors in radians.
    fn angle(
        &self,
        other: &Self,
    ) -> Self::ScalarType {
        if self.len() != other.len() {
            panic!("Vectors must be of the same length");
        }

        if self.len() != 2 && self.len() != 3 {
            panic!("Angle is only defined for 2D and 3D vectors");
        }

        if self.norm() < Self::ScalarType::small() || other.norm() < Self::ScalarType::small() {
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
//{{{ macro: float_transform_unary
macro_rules! float_transform_unary {
    ($method:ident, $methoded:ident, $into_methoded:ident) => {
        #[doc = concat!("Applies [`Float::", stringify!($method), "`] to every element in place.")]
        fn $method(&mut self) {
            self.transform(|value| value.$method());
        }

        #[doc = concat!("Returns a copy with [`Float::", stringify!($method), "`] applied element-wise.")]
        fn $methoded(&self) -> Self
        where
            Self: Clone,
        {
            self.transformed(|value| value.$method())
        }

        #[doc = concat!("Consumes the value and applies [`Float::", stringify!($method), "`] element-wise.")]
        fn $into_methoded(self) -> Self {
            self.into_transformed(|value| value.$method())
        }
    };
}
//}}}
//{{{ macro: float_transform_unary_with_arg
macro_rules! float_transform_unary_with_arg {
    ($method:ident, $methoded:ident, $into_methoded:ident, $arg:ident: $arg_type:ty) => {
        #[doc = concat!("Applies [`Float::", stringify!($method), "`] to every element in place.")]
        fn $method(
            &mut self,
            $arg: $arg_type,
        ) {
            self.transform(|value| value.$method($arg));
        }

        #[doc = concat!("Returns a copy with [`Float::", stringify!($method), "`] applied element-wise.")]
        fn $methoded(
            &self,
            $arg: $arg_type,
        ) -> Self
        where
            Self: Clone,
        {
            self.transformed(|value| value.$method($arg))
        }

        #[doc = concat!("Consumes the value and applies [`Float::", stringify!($method), "`] element-wise.")]
        fn $into_methoded(
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
        $methoded:ident,
        $into_methoded:ident,
        $arg1:ident: $arg1_type:ty,
        $arg2:ident: $arg2_type:ty
    ) => {
        #[doc = concat!("Applies [`Float::", stringify!($method), "`] to every element in place.")]
        fn $method(
            &mut self,
            $arg1: $arg1_type,
            $arg2: $arg2_type,
        ) {
            self.transform(|value| value.$method($arg1, $arg2));
        }

        #[doc = concat!("Returns a copy with [`Float::", stringify!($method), "`] applied element-wise.")]
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

        #[doc = concat!("Consumes the value and applies [`Float::", stringify!($method), "`] element-wise.")]
        fn $into_methoded(
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
/// in-place, cloning (`*ed`), and consuming (`into_*ed`) variants for each operation.
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
    float_transform_unary_with_arg!(copysign, copysigned, into_copysigned, sign: Self::ScalarType);
    float_transform_unary!(cos, cosed, into_cosed);
    float_transform_unary!(cosh, coshed, into_coshed);
    float_transform_unary_with_arg!(div_euclid, div_euclided, into_div_euclided, rhs: Self::ScalarType);
    float_transform_unary!(exp, exped, into_exped);
    float_transform_unary!(exp2, exp2ed, into_exp2ed);
    float_transform_unary!(exp_m1, exp_m1ed, into_exp_m1ed);
    float_transform_unary!(floor, floored, into_floored);
    float_transform_unary!(fract, fracted, into_fracted);
    float_transform_unary_with_arg!(hypot, hypoted, into_hypoted, other: Self::ScalarType);
    float_transform_unary!(ln, lned, into_lned);
    float_transform_unary!(ln_1p, ln_1ped, into_ln_1ped);
    float_transform_unary_with_arg!(log, loged, into_loged, base: Self::ScalarType);
    float_transform_unary!(log10, log10ed, into_log10ed);
    float_transform_unary!(log2, log2ed, into_log2ed);
    float_transform_unary_with_arg!(max, maxed, into_maxed, other: Self::ScalarType);
    float_transform_unary_with_arg!(midpoint, midpointed, into_midpointed, other: Self::ScalarType);
    float_transform_unary_with_arg!(min, mined, into_mined, other: Self::ScalarType);
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

    /// Clamps every element to the positive part in-place (negative values become zero).
    fn pos(&mut self)
    where
        Self::ScalarType: Zero,
    {
        self.transform(|value| {
            if value > Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        });
    }

    /// Returns a cloned copy of `self` with every element clamped to its positive part.
    fn posed(&self) -> Self
    where
        Self: Clone,
        Self::ScalarType: Zero,
    {
        self.transformed(|value| {
            if value > Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        })
    }

    /// Consumes `self`, clamps every element to its positive part, and returns it.
    fn into_posed(mut self) -> Self
    where
        Self::ScalarType: Zero,
    {
        self.pos();
        self
    }

    /// Clamps every element to the non-positive part in-place (positive values become zero).
    fn neg(&mut self)
    where
        Self::ScalarType: Zero,
    {
        self.transform(|value| {
            if value <= Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        });
    }

    /// Returns a cloned copy of `self` with every element clamped to its non-positive part.
    fn neged(&self) -> Self
    where
        Self: Clone,
        Self::ScalarType: Zero,
    {
        self.transformed(|value| {
            if value <= Self::ScalarType::zero() {
                value
            } else {
                Self::ScalarType::zero()
            }
        })
    }

    /// Consumes `self`, clamps every element to its non-positive part, and returns it.
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
