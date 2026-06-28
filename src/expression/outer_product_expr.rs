//! Lazy vector outer-product expression node.
//!
//! `OuterProductExpr` represents `M_ij = v_i * w_j` without materialising the matrix. It is a
//! matrix-shaped expression leaf, so it can participate in the existing element-wise expression
//! tree and be evaluated directly into a destination buffer.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
use crate::common::{Field, MatrixExpr, ScalarExpr, Shape};
use crate::expression::binary_expr::{AddOp, BinopExpr, DivOp, MulOp, SubOp};
use crate::expression::unary_expr::{NegOp, UnaryExpr};
//}}}
//{{{ std imports
use std::ops::{Add, Div, Mul, Neg, Sub};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fun: vector_len
#[inline]
fn vector_len<X>(expr: &X) -> usize
where
    X: Shape,
{
    let nrows = expr.nrows();
    let ncols = expr.ncols();
    if nrows == 1 {
        ncols
    } else if ncols == 1 {
        nrows
    } else {
        panic!("Outer product operands must be vectors");
    }
}
//}}}
//{{{ struct: OuterProductExpr
/// Lazy vector outer-product expression.
pub struct OuterProductExpr<L, R, T>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    pub(crate) left: L,
    pub(crate) right: R,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
    pub(crate) _marker: std::marker::PhantomData<T>,
}

impl<L, R, T> OuterProductExpr<L, R, T>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    #[inline]
    pub(crate) fn new(
        left: L,
        right: R,
    ) -> Self {
        let nrows = vector_len(&left);
        let ncols = vector_len(&right);
        Self {
            left,
            right,
            nrows,
            ncols,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Shape for OuterProductExpr
impl<L, R, T> Shape for OuterProductExpr<L, R, T>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
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
//{{{ impl: MatrixExpr for OuterProductExpr
impl<L, R, T> MatrixExpr for OuterProductExpr<L, R, T>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType {
        let row = index % self.nrows;
        let col = index / self.nrows;
        self.left.linear_value(row) * self.right.linear_value(col)
    }

    #[inline]
    fn eval_into(
        &self,
        out: &mut [T],
    ) {
        debug_assert_eq!(out.len(), self.nrows * self.ncols);
        for col in 0..self.ncols {
            let rhs_value = self.right.linear_value(col);
            let col_offset = col * self.nrows;
            for row in 0..self.nrows {
                unsafe {
                    *out.get_unchecked_mut(col_offset + row) =
                        self.left.linear_value(row) * rhs_value;
                }
            }
        }
    }
}
//}}}
//{{{ trait: OuterProduct
/// Extension trait for lazy vector outer products.
pub trait OuterProduct: MatrixExpr + Sized {
    /// Returns a lazy expression representing `self[i] * rhs[j]`.
    fn outer<'a, Rhs>(
        &'a self,
        rhs: &'a Rhs,
    ) -> OuterProductExpr<&'a Self, &'a Rhs, Self::ScalarType>
    where
        Self::ScalarType: Field + Copy,
        Rhs: MatrixExpr<ScalarType = Self::ScalarType>,
    {
        OuterProductExpr::new(self, rhs)
    }
}

impl<X> OuterProduct for X where X: MatrixExpr + Sized {}
//}}}
//{{{ macro: impl_outer_product_expr_binary_op
macro_rules! impl_outer_product_expr_binary_op {
    ($trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<L, R, T, Rhs> $trait<Rhs> for OuterProductExpr<L, R, T>
        where
            L: MatrixExpr<ScalarType = T>,
            R: MatrixExpr<ScalarType = T>,
            T: Field + Copy,
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

impl_outer_product_expr_binary_op!(Add, add, AddOp);
impl_outer_product_expr_binary_op!(Sub, sub, SubOp);
impl_outer_product_expr_binary_op!(Mul, mul, MulOp);
impl_outer_product_expr_binary_op!(Div, div, DivOp);
//}}}
//{{{ macro: impl_outer_product_expr_scalar_rhs_op
macro_rules! impl_outer_product_expr_scalar_rhs_op {
    ($type:ty, $trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<L, R> $trait<$type> for OuterProductExpr<L, R, $type>
        where
            L: MatrixExpr<ScalarType = $type>,
            R: MatrixExpr<ScalarType = $type>,
        {
            type Output = BinopExpr<Self, ScalarExpr<$type>, $type, $op>;

            #[inline]
            fn $method(
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

macro_rules! impl_add_outer_product_expr_scalar_rhs {
    ($type:ty) => {
        impl_outer_product_expr_scalar_rhs_op!($type, Add, add, AddOp);
    };
}

macro_rules! impl_sub_outer_product_expr_scalar_rhs {
    ($type:ty) => {
        impl_outer_product_expr_scalar_rhs_op!($type, Sub, sub, SubOp);
    };
}

macro_rules! impl_mul_outer_product_expr_scalar_rhs {
    ($type:ty) => {
        impl_outer_product_expr_scalar_rhs_op!($type, Mul, mul, MulOp);
    };
}

macro_rules! impl_div_outer_product_expr_scalar_rhs {
    ($type:ty) => {
        impl_outer_product_expr_scalar_rhs_op!($type, Div, div, DivOp);
    };
}

apply_for_all_types!(impl_add_outer_product_expr_scalar_rhs);
apply_for_all_types!(impl_sub_outer_product_expr_scalar_rhs);
apply_for_all_types!(impl_mul_outer_product_expr_scalar_rhs);
apply_for_all_types!(impl_div_outer_product_expr_scalar_rhs);
//}}}
//{{{ macro: impl_scalar_lhs_outer_product_expr_op
macro_rules! impl_scalar_lhs_outer_product_expr_op {
    ($macro_name:ident, $type:ty, $trait:ident, $method:ident, $op:ty) => {
        #[doc(hidden)]
        impl<L, R> $trait<OuterProductExpr<L, R, $type>> for $type
        where
            L: MatrixExpr<ScalarType = $type>,
            R: MatrixExpr<ScalarType = $type>,
        {
            type Output = BinopExpr<ScalarExpr<$type>, OuterProductExpr<L, R, $type>, $type, $op>;

            #[inline]
            fn $method(
                self,
                rhs: OuterProductExpr<L, R, $type>,
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

macro_rules! impl_add_outer_product_expr {
    ($type:ty) => {
        impl_scalar_lhs_outer_product_expr_op!(impl_add_outer_product_expr, $type, Add, add, AddOp);
    };
}

macro_rules! impl_sub_outer_product_expr {
    ($type:ty) => {
        impl_scalar_lhs_outer_product_expr_op!(impl_sub_outer_product_expr, $type, Sub, sub, SubOp);
    };
}

macro_rules! impl_mul_outer_product_expr {
    ($type:ty) => {
        impl_scalar_lhs_outer_product_expr_op!(impl_mul_outer_product_expr, $type, Mul, mul, MulOp);
    };
}

macro_rules! impl_div_outer_product_expr {
    ($type:ty) => {
        impl_scalar_lhs_outer_product_expr_op!(impl_div_outer_product_expr, $type, Div, div, DivOp);
    };
}

apply_for_all_types!(impl_add_outer_product_expr);
apply_for_all_types!(impl_sub_outer_product_expr);
apply_for_all_types!(impl_mul_outer_product_expr);
apply_for_all_types!(impl_div_outer_product_expr);
//}}}
//{{{ impl: Neg for OuterProductExpr
impl<L, R, T> Neg for OuterProductExpr<L, R, T>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    type Output = UnaryExpr<Self, T, NegOp>;

    #[inline]
    fn neg(self) -> Self::Output {
        UnaryExpr::new(self, NegOp)
    }
}
//}}}
