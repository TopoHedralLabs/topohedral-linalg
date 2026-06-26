//! Lazy binary expression node for element-wise operations on two matrix operands.
//!
//! Provides the [`BinopExpr`] struct, which captures two operands and a stateless [`BinOp`]
//! descriptor without immediately evaluating the result. Concrete operators — [`AddOp`],
//! [`SubOp`], [`MulOp`], [`DivOp`] — implement [`BinOp`] by applying the corresponding
//! arithmetic operation element-by-element. Evaluation is deferred until the expression is
//! consumed via [`MatrixExpr::eval_into`] or converted into a [`DMatrix`] / [`SMatrix`], at which
//! point a single pass over the linear index range writes the result directly into the output
//! buffer.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
use crate::common::{Field, MatrixExpr, ScalarExpr, Shape};
//}}}
//{{{ std imports
use std::ops::{Add, Div, Mul, Sub};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct AddOp
#[doc(hidden)]
pub struct AddOp;

//}}}
//{{{ struct SubOp
#[doc(hidden)]
pub struct SubOp;

//}}}
//{{{ struct MulOp
#[doc(hidden)]
pub struct MulOp;

//}}}
//{{{ struct DivOp
#[doc(hidden)]
pub struct DivOp;

//}}}
//{{{ trait: BinOp
#[doc(hidden)]
pub trait BinOp {
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T;
}

//}}}
//{{{ impl BinOp for AddOp
#[doc(hidden)]
impl BinOp for AddOp {
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T {
        a + b
    }
}

//}}}
//{{{ impl BinOp for SubOp
#[doc(hidden)]
impl BinOp for SubOp {
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T {
        a - b
    }
}

//}}}
//{{{ impl BinOp for MulOp
#[doc(hidden)]
impl BinOp for MulOp {
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T {
        a * b
    }
}

//}}}
//{{{ impl BInOp for DivOp
#[doc(hidden)]
impl BinOp for DivOp {
    #[inline]
    fn apply<T: Field>(
        a: T,
        b: T,
    ) -> T {
        a / b
    }
}

//}}}
//{{{ struct: BinopExpr
#[doc(hidden)]
pub struct BinopExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: BinOp,
{
    pub a: A,
    pub b: B,
    pub nrows: usize,
    pub ncols: usize,
    pub _marker: std::marker::PhantomData<(T, Op)>,
}

//}}}
//{{{ impl: Shape for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> Shape for BinopExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: BinOp,
{
    #[inline]
    fn nrows(&self) -> usize {
        self.nrows
    }

    #[inline]
    fn ncols(&self) -> usize {
        self.ncols
    }
}

//}}}
//{{{ impl: MatrixExpr for BinopExpr
impl<A, B, T, Op> MatrixExpr for BinopExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: BinOp,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType {
        Op::apply(self.a.linear_value(index), self.b.linear_value(index))
    }

    /// Single-pass evaluation via `out: &mut [T]`.
    ///
    /// Writing through the `noalias &mut [T]` parameter lets LLVM prove the output
    /// doesn't overlap any `&DMatrix` input (which are `noalias readonly`), so it
    /// can hoist all Vec data-pointer loads out of the loop and emit a single SIMD
    /// pass over all operands — matching nalgebra's vectorisation quality while
    /// doing one pass instead of N-1 separate fold passes.
    #[inline]
    fn eval_into(
        &self,
        out: &mut [T],
    ) {
        let len = out.len();
        for i in 0..len {
            // Safety: i < len = out.len()
            // `out` is noalias &mut [T]; all inputs are noalias readonly &DMatrix,
            // so LLVM proves non-aliasing and auto-vectorises this loop.
            unsafe {
                *out.get_unchecked_mut(i) =
                    Op::apply(self.a.linear_value(i), self.b.linear_value(i));
            }
        }
    }
}
//}}}
//{{{ macro: impl_binop_expr_binary_op
macro_rules! impl_binop_expr_binary_op {
    ($trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<A, B, T, Op, Rhs> $trait<Rhs> for BinopExpr<A, B, T, Op>
        where
            A: MatrixExpr<ScalarType = T>,
            B: MatrixExpr<ScalarType = T>,
            T: Field + Copy,
            Op: BinOp,
            Rhs: MatrixExpr<ScalarType = T>,
        {
            type Output = BinopExpr<Self, Rhs, T, $op>;

            #[inline]
            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output {
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

impl_binop_expr_binary_op!(Add, add, AddOp);
impl_binop_expr_binary_op!(Sub, sub, SubOp);
impl_binop_expr_binary_op!(Mul, mul, MulOp);
impl_binop_expr_binary_op!(Div, div, DivOp);

//}}}
//{{{ impl: Add<$type> for BinopExpr
macro_rules! impl_add_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Add<$type> for BinopExpr<A, B, $type, Op>
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, ScalarExpr<$type>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: $type,
            ) -> Self::Output {
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_add_binop_expr_scalar_rhs);

//}}}
//{{{ impl: Add<BinopExpr> for $type
macro_rules! impl_add_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Add<BinopExpr<A, B, $type, Op>> for $type
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<ScalarExpr<$type>, BinopExpr<A, B, $type, Op>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_add_binop_expr);

//}}}
//{{{ impl: Sub<$type> for BinopExpr
macro_rules! impl_sub_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Sub<$type> for BinopExpr<A, B, $type, Op>
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, ScalarExpr<$type>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: $type,
            ) -> Self::Output {
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_sub_binop_expr_scalar_rhs);

//}}}
//{{{ impl: Sub<BinopExpr> for $type
macro_rules! impl_sub_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Sub<BinopExpr<A, B, $type, Op>> for $type
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<ScalarExpr<$type>, BinopExpr<A, B, $type, Op>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_sub_binop_expr);

//}}}
//{{{ impl: Mul<$type> for BinopExpr
macro_rules! impl_mul_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Mul<$type> for BinopExpr<A, B, $type, Op>
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, ScalarExpr<$type>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: $type,
            ) -> Self::Output {
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_mul_binop_expr_scalar_rhs);

//}}}
//{{{ impl: Mul<BinopExpr> for $type
macro_rules! impl_mul_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Mul<BinopExpr<A, B, $type, Op>> for $type
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<ScalarExpr<$type>, BinopExpr<A, B, $type, Op>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_mul_binop_expr);

//}}}
//{{{ impl: Div<$type> for BinopExpr
macro_rules! impl_div_binop_expr_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Div<$type> for BinopExpr<A, B, $type, Op>
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<Self, ScalarExpr<$type>, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: $type,
            ) -> Self::Output {
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_div_binop_expr_scalar_rhs);

//}}}
//{{{ impl: Div<BinopExpr> for $type
macro_rules! impl_div_binop_expr {
    ($type:ty) => {
        #[doc(hidden)]
        impl<A, B, Op> Div<BinopExpr<A, B, $type, Op>> for $type
        where
            A: MatrixExpr<ScalarType = $type>,
            B: MatrixExpr<ScalarType = $type>,
            Op: BinOp,
        {
            type Output = BinopExpr<ScalarExpr<$type>, BinopExpr<A, B, $type, Op>, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: BinopExpr<A, B, $type, Op>,
            ) -> Self::Output {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

apply_for_all_types!(impl_div_binop_expr);

//}}}
