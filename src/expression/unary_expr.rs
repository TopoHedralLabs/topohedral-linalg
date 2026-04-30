//! Lazy unary expression support.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
use crate::common::{EvalInto, Field, Float, IndexValue, LazyExpr, Shape};
use crate::expression::binary_expr::{AddOp, BinopExpr, DivOp, MulOp, SubOp};
//}}}
//{{{ std imports
use std::ops::{Add, Div, Mul, Neg, Sub};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: UnaryOp
#[doc(hidden)]
pub trait UnaryOp<T>
where
    T: Field + Copy,
{
    fn apply(
        &self,
        value: T,
    ) -> T;
}

//}}}
//{{{ struct: NegOp
#[doc(hidden)]
pub struct NegOp;

impl<T> UnaryOp<T> for NegOp
where
    T: Field + Copy,
{
    #[inline]
    fn apply(
        &self,
        value: T,
    ) -> T
    {
        -value
    }
}

//}}}
//{{{ struct: UnaryExpr
#[doc(hidden)]
pub struct UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    pub a: A,
    pub op: Op,
    pub nrows: usize,
    pub ncols: usize,
    pub _marker: std::marker::PhantomData<T>,
}

impl<A, T, Op> UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    #[inline]
    pub(crate) fn new(
        a: A,
        op: Op,
    ) -> Self
    {
        let nrows = a.nrows();
        let ncols = a.ncols();
        Self {
            a,
            op,
            nrows,
            ncols,
            _marker: std::marker::PhantomData,
        }
    }
}

//}}}
//{{{ impl: Shape for UnaryExpr
impl<A, T, Op> Shape for UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    #[inline]
    fn nrows(&self) -> usize
    {
        self.nrows
    }

    #[inline]
    fn ncols(&self) -> usize
    {
        self.ncols
    }
}

//}}}
//{{{ impl: LazyExpr for UnaryExpr
impl<A, T, Op> LazyExpr for UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    type ScalarType = T;
}

//}}}
//{{{ impl: EvalInto for UnaryExpr
impl<A, T, Op> EvalInto<T> for UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T> + EvalInto<T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    #[inline]
    fn eval_into(
        &self,
        out: &mut [T],
    )
    {
        self.a.eval_into(out);
        let len = out.len();
        for i in 0..len
        {
            unsafe { *out.get_unchecked_mut(i) = self.op.apply(*out.get_unchecked(i)) };
        }
    }
}
//}}}
//{{{ impl: IndexValue for UnaryExpr
impl<A, T, Op> IndexValue<usize> for UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    type Output = T;

    #[inline]
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        self.op.apply(self.a.index_value(index))
    }
}

//}}}
//{{{ macro: impl_unary_expr_binary_op
macro_rules! impl_unary_expr_binary_op {
    ($trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<A, T, Op, Rhs> $trait<Rhs> for UnaryExpr<A, T, Op>
        where
            A: Shape + IndexValue<usize, Output = T>,
            T: Field + Copy,
            Op: UnaryOp<T>,
            Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
        {
            type Output = BinopExpr<Self, Rhs, T, $op>;

            #[inline]
            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                debug_assert!(self.nrows == rhs.nrows());
                debug_assert!(self.ncols == rhs.ncols());
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

//}}}
//{{{ macro: impl_unary_expr_scalar_rhs_op
macro_rules! impl_unary_expr_scalar_rhs_op {
    ($type:ty, $trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<A, Op> $trait<$type> for UnaryExpr<A, $type, Op>
        where
            A: Shape + IndexValue<usize, Output = $type>,
            Op: UnaryOp<$type>,
        {
            type Output = BinopExpr<Self, $type, $type, $op>;

            #[inline]
            fn $method(
                self,
                rhs: $type,
            ) -> Self::Output
            {
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_add_unary_expr_scalar_rhs {
    ($type:ty) => {
        impl_unary_expr_scalar_rhs_op!($type, Add, add, AddOp);
    };
}

macro_rules! impl_sub_unary_expr_scalar_rhs {
    ($type:ty) => {
        impl_unary_expr_scalar_rhs_op!($type, Sub, sub, SubOp);
    };
}

macro_rules! impl_mul_unary_expr_scalar_rhs {
    ($type:ty) => {
        impl_unary_expr_scalar_rhs_op!($type, Mul, mul, MulOp);
    };
}

macro_rules! impl_div_unary_expr_scalar_rhs {
    ($type:ty) => {
        impl_unary_expr_scalar_rhs_op!($type, Div, div, DivOp);
    };
}

//}}}
//{{{ macro: impl_scalar_lhs_unary_expr_op
macro_rules! impl_scalar_lhs_unary_expr_op {
    ($macro_name:ident, $type:ty, $trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<A, Op> $trait<UnaryExpr<A, $type, Op>> for $type
        where
            A: Shape + IndexValue<usize, Output = $type>,
            Op: UnaryOp<$type>,
        {
            type Output = BinopExpr<Self, UnaryExpr<A, $type, Op>, $type, $op>;

            #[inline]
            fn $method(
                self,
                rhs: UnaryExpr<A, $type, Op>,
            ) -> Self::Output
            {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_add_unary_expr {
    ($type:ty) => {
        impl_scalar_lhs_unary_expr_op!(impl_add_unary_expr, $type, Add, add, AddOp);
    };
}

macro_rules! impl_sub_unary_expr {
    ($type:ty) => {
        impl_scalar_lhs_unary_expr_op!(impl_sub_unary_expr, $type, Sub, sub, SubOp);
    };
}

macro_rules! impl_mul_unary_expr {
    ($type:ty) => {
        impl_scalar_lhs_unary_expr_op!(impl_mul_unary_expr, $type, Mul, mul, MulOp);
    };
}

macro_rules! impl_div_unary_expr {
    ($type:ty) => {
        impl_scalar_lhs_unary_expr_op!(impl_div_unary_expr, $type, Div, div, DivOp);
    };
}

//}}}
//{{{ impl: binary ops for UnaryExpr
impl_unary_expr_binary_op!(Add, add, AddOp);
impl_unary_expr_binary_op!(Sub, sub, SubOp);
impl_unary_expr_binary_op!(Mul, mul, MulOp);
impl_unary_expr_binary_op!(Div, div, DivOp);

apply_for_all_types!(impl_add_unary_expr_scalar_rhs);
apply_for_all_types!(impl_sub_unary_expr_scalar_rhs);
apply_for_all_types!(impl_mul_unary_expr_scalar_rhs);
apply_for_all_types!(impl_div_unary_expr_scalar_rhs);

apply_for_all_types!(impl_add_unary_expr);
apply_for_all_types!(impl_sub_unary_expr);
apply_for_all_types!(impl_mul_unary_expr);
apply_for_all_types!(impl_div_unary_expr);

//}}}
//{{{ impl: Neg for UnaryExpr
impl<A, T, Op> Neg for UnaryExpr<A, T, Op>
where
    A: Shape + IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    type Output = UnaryExpr<Self, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output
    {
        UnaryExpr::new(self, NegOp)
    }
}

//}}}
//{{{ impl: Neg for BinopExpr
impl<A, B, T, Op> Neg for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: crate::expression::binary_expr::BinOp,
{
    type Output = UnaryExpr<Self, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output
    {
        UnaryExpr::new(self, NegOp)
    }
}

//}}}
//{{{ macro: define_float_unary_op
macro_rules! define_float_unary_op {
    ($op_name:ident, $func_name:ident, $method:ident) => {
        #[doc(hidden)]
        pub struct $op_name;

        impl<T> UnaryOp<T> for $op_name
        where
            T: Float + Copy,
        {
            #[inline]
            fn apply(
                &self,
                value: T,
            ) -> T
            {
                value.$method()
            }
        }

        #[inline]
        pub fn $func_name<A, T>(expr: A) -> UnaryExpr<A, T, $op_name>
        where
            A: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
            T: Float + Copy,
        {
            UnaryExpr::new(expr, $op_name)
        }
    };
}

//}}}
//{{{ macro: define_float_unary_op_with_same_arg
macro_rules! define_float_unary_op_with_same_arg {
    ($op_name:ident, $func_name:ident, $method:ident, $arg_name:ident) => {
        #[doc(hidden)]
        pub struct $op_name<T>
        where
            T: Float + Copy,
        {
            pub $arg_name: T,
        }

        impl<T> UnaryOp<T> for $op_name<T>
        where
            T: Float + Copy,
        {
            #[inline]
            fn apply(
                &self,
                value: T,
            ) -> T
            {
                value.$method(self.$arg_name)
            }
        }

        #[inline]
        pub fn $func_name<A, T>(
            expr: A,
            $arg_name: T,
        ) -> UnaryExpr<A, T, $op_name<T>>
        where
            A: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
            T: Float + Copy,
        {
            UnaryExpr::new(expr, $op_name { $arg_name })
        }
    };
}

//}}}
//{{{ macro: define_float_unary_op_with_two_same_args
macro_rules! define_float_unary_op_with_two_same_args {
    ($op_name:ident, $func_name:ident, $method:ident, $arg1:ident, $arg2:ident) => {
        #[doc(hidden)]
        pub struct $op_name<T>
        where
            T: Float + Copy,
        {
            pub $arg1: T,
            pub $arg2: T,
        }

        impl<T> UnaryOp<T> for $op_name<T>
        where
            T: Float + Copy,
        {
            #[inline]
            fn apply(
                &self,
                value: T,
            ) -> T
            {
                value.$method(self.$arg1, self.$arg2)
            }
        }

        #[inline]
        pub fn $func_name<A, T>(
            expr: A,
            $arg1: T,
            $arg2: T,
        ) -> UnaryExpr<A, T, $op_name<T>>
        where
            A: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
            T: Float + Copy,
        {
            UnaryExpr::new(expr, $op_name { $arg1, $arg2 })
        }
    };
}

//}}}
//{{{ collection: float unary ops
define_float_unary_op!(AbsOp, abs, abs);
define_float_unary_op_with_same_arg!(AbsSubOp, abs_sub, abs_sub, other);
define_float_unary_op!(AcosOp, acos, acos);
define_float_unary_op!(AcoshOp, acosh, acosh);
define_float_unary_op!(AsinOp, asin, asin);
define_float_unary_op!(AsinhOp, asinh, asinh);
define_float_unary_op!(AtanOp, atan, atan);
define_float_unary_op_with_same_arg!(Atan2Op, atan2, atan2, other);
define_float_unary_op!(AtanhOp, atanh, atanh);
define_float_unary_op!(CbrtOp, cbrt, cbrt);
define_float_unary_op!(CeilOp, ceil, ceil);
define_float_unary_op_with_two_same_args!(ClampOp, clamp, clamp, min, max);
define_float_unary_op_with_same_arg!(ClampMagnitudeOp, clamp_magnitude, clamp_magnitude, limit);
define_float_unary_op_with_same_arg!(CopysignOp, copysign, copysign, sign);
define_float_unary_op!(CosOp, cos, cos);
define_float_unary_op!(CoshOp, cosh, cosh);
define_float_unary_op_with_same_arg!(DivEuclidOp, div_euclid, div_euclid, rhs);
define_float_unary_op!(ErfOp, erf, erf);
define_float_unary_op!(ErfcOp, erfc, erfc);
define_float_unary_op!(ExpOp, exp, exp);
define_float_unary_op!(Exp2Op, exp2, exp2);
define_float_unary_op!(ExpM1Op, exp_m1, exp_m1);
define_float_unary_op!(FloorOp, floor, floor);
define_float_unary_op!(FractOp, fract, fract);
define_float_unary_op!(GammaOp, gamma, gamma);
define_float_unary_op_with_same_arg!(HypotOp, hypot, hypot, other);
define_float_unary_op!(LnOp, ln, ln);
define_float_unary_op!(Ln1pOp, ln_1p, ln_1p);
define_float_unary_op_with_same_arg!(LogOp, log, log, base);
define_float_unary_op!(Log10Op, log10, log10);
define_float_unary_op!(Log2Op, log2, log2);
define_float_unary_op_with_same_arg!(MaxOp, max, max, other);
define_float_unary_op_with_same_arg!(MaximumOp, maximum, maximum, other);
define_float_unary_op_with_same_arg!(MidpointOp, midpoint, midpoint, other);
define_float_unary_op_with_same_arg!(MinOp, min, min, other);
define_float_unary_op_with_same_arg!(MinimumOp, minimum, minimum, other);
define_float_unary_op_with_two_same_args!(MulAddOp, mul_add, mul_add, a, b);
define_float_unary_op!(NextDownOp, next_down, next_down);
define_float_unary_op!(NextUpOp, next_up, next_up);
define_float_unary_op_with_same_arg!(PowfOp, powf, powf, exp);

#[doc(hidden)]
pub struct PowiOp
{
    pub exp: i32,
}

impl<T> UnaryOp<T> for PowiOp
where
    T: Float + Copy,
{
    #[inline]
    fn apply(
        &self,
        value: T,
    ) -> T
    {
        value.powi(self.exp)
    }
}

#[inline]
pub fn powi<A, T>(
    expr: A,
    exp: i32,
) -> UnaryExpr<A, T, PowiOp>
where
    A: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
    T: Float + Copy,
{
    UnaryExpr::new(expr, PowiOp { exp })
}

define_float_unary_op!(RecipOp, recip, recip);
define_float_unary_op_with_same_arg!(RemEuclidOp, rem_euclid, rem_euclid, rhs);
define_float_unary_op!(RoundOp, round, round);
define_float_unary_op!(RoundTiesEvenOp, round_ties_even, round_ties_even);
define_float_unary_op!(SignumOp, signum, signum);
define_float_unary_op!(SinOp, sin, sin);
define_float_unary_op!(SinhOp, sinh, sinh);
define_float_unary_op!(SqrtOp, sqrt, sqrt);
define_float_unary_op!(TanOp, tan, tan);
define_float_unary_op!(TanhOp, tanh, tanh);
define_float_unary_op!(ToDegreesOp, to_degrees, to_degrees);
define_float_unary_op!(ToRadiansOp, to_radians, to_radians);
define_float_unary_op!(TruncOp, trunc, trunc);
define_float_unary_op_with_same_arg!(AlgebraicAddOp, algebraic_add, algebraic_add, rhs);
define_float_unary_op_with_same_arg!(AlgebraicSubOp, algebraic_sub, algebraic_sub, rhs);
define_float_unary_op_with_same_arg!(AlgebraicMulOp, algebraic_mul, algebraic_mul, rhs);
define_float_unary_op_with_same_arg!(AlgebraicDivOp, algebraic_div, algebraic_div, rhs);
define_float_unary_op_with_same_arg!(AlgebraicRemOp, algebraic_rem, algebraic_rem, rhs);

//}}}
