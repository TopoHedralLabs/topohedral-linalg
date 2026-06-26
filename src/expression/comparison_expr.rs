//! Lazy element-wise comparison expressions.

use crate::common::{MatrixExpr, ScalarExpr, Shape};
use crate::{apply_for_all_types, DMatrix, SMatrix};
use std::marker::PhantomData;

/// Converts a comparison right-hand side into a matrix expression.
#[doc(hidden)]
pub trait CompareRhs<T>
where
    T: Copy,
{
    type Expr: MatrixExpr<ScalarType = T>;

    fn into_compare_expr(
        self,
        nrows: usize,
        ncols: usize,
    ) -> Self::Expr;
}

impl<'a, T, X> CompareRhs<T> for &'a X
where
    T: Copy,
    X: MatrixExpr<ScalarType = T>,
{
    type Expr = &'a X;

    fn into_compare_expr(
        self,
        _nrows: usize,
        _ncols: usize,
    ) -> Self::Expr {
        self
    }
}

impl<'a, T, X> CompareRhs<T> for &'a mut X
where
    T: Copy,
    X: MatrixExpr<ScalarType = T>,
{
    type Expr = &'a mut X;

    fn into_compare_expr(
        self,
        _nrows: usize,
        _ncols: usize,
    ) -> Self::Expr {
        self
    }
}

macro_rules! impl_scalar_compare_rhs {
    ($type:ty) => {
        impl CompareRhs<$type> for $type {
            type Expr = ScalarExpr<$type>;

            fn into_compare_expr(
                self,
                nrows: usize,
                ncols: usize,
            ) -> Self::Expr {
                ScalarExpr::new(self, nrows, ncols)
            }
        }
    };
}

apply_for_all_types!(impl_scalar_compare_rhs);
impl_scalar_compare_rhs!(bool);

#[doc(hidden)]
pub trait CompareOp<T>
where
    T: Copy,
{
    fn apply(
        a: T,
        b: T,
    ) -> bool;
}

macro_rules! define_compare_op {
    ($name:ident, $bound:path, $body:expr) => {
        #[doc(hidden)]
        pub struct $name;

        impl<T> CompareOp<T> for $name
        where
            T: Copy + $bound,
        {
            #[inline]
            fn apply(
                a: T,
                b: T,
            ) -> bool {
                $body(a, b)
            }
        }
    };
}

define_compare_op!(EqOp, PartialEq, |a, b| a == b);
define_compare_op!(NeOp, PartialEq, |a, b| a != b);
define_compare_op!(LtOp, PartialOrd, |a, b| a < b);
define_compare_op!(LeOp, PartialOrd, |a, b| a <= b);
define_compare_op!(GtOp, PartialOrd, |a, b| a > b);
define_compare_op!(GeOp, PartialOrd, |a, b| a >= b);

/// Lazy element-wise comparison whose values are booleans.
#[doc(hidden)]
pub struct CompareExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    pub(crate) a: A,
    pub(crate) b: B,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
    pub(crate) _marker: PhantomData<(T, Op)>,
}

impl<A, B, T, Op> CompareExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    fn new(
        a: A,
        b: B,
    ) -> Self {
        let nrows = a.nrows();
        let ncols = a.ncols();
        assert_eq!(
            (nrows, ncols),
            (b.nrows(), b.ncols()),
            "element-wise comparison dimension mismatch: lhs is {}x{}, rhs is {}x{}",
            nrows,
            ncols,
            b.nrows(),
            b.ncols()
        );
        Self {
            a,
            b,
            nrows,
            ncols,
            _marker: PhantomData,
        }
    }
}

impl<A, B, T, Op> Shape for CompareExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}

impl<A, B, T, Op> MatrixExpr for CompareExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    type ScalarType = bool;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> bool {
        Op::apply(self.a.linear_value(index), self.b.linear_value(index))
    }
}

/// Named element-wise comparisons for matrix expressions.
pub trait ElementwiseCompare: MatrixExpr {
    #[doc(hidden)]
    fn eq<Rhs>(
        &self,
        rhs: Rhs,
    ) -> CompareExpr<&Self, Rhs::Expr, Self::ScalarType, EqOp>
    where
        Self::ScalarType: PartialEq,
        Rhs: CompareRhs<Self::ScalarType>,
    {
        let rhs = rhs.into_compare_expr(self.nrows(), self.ncols());
        CompareExpr::new(self, rhs)
    }

    #[doc(hidden)]
    fn ne<Rhs>(
        &self,
        rhs: Rhs,
    ) -> CompareExpr<&Self, Rhs::Expr, Self::ScalarType, NeOp>
    where
        Self::ScalarType: PartialEq,
        Rhs: CompareRhs<Self::ScalarType>,
    {
        let rhs = rhs.into_compare_expr(self.nrows(), self.ncols());
        CompareExpr::new(self, rhs)
    }

    #[doc(hidden)]
    fn lt<Rhs>(
        &self,
        rhs: Rhs,
    ) -> CompareExpr<&Self, Rhs::Expr, Self::ScalarType, LtOp>
    where
        Self::ScalarType: PartialOrd,
        Rhs: CompareRhs<Self::ScalarType>,
    {
        let rhs = rhs.into_compare_expr(self.nrows(), self.ncols());
        CompareExpr::new(self, rhs)
    }

    #[doc(hidden)]
    fn le<Rhs>(
        &self,
        rhs: Rhs,
    ) -> CompareExpr<&Self, Rhs::Expr, Self::ScalarType, LeOp>
    where
        Self::ScalarType: PartialOrd,
        Rhs: CompareRhs<Self::ScalarType>,
    {
        let rhs = rhs.into_compare_expr(self.nrows(), self.ncols());
        CompareExpr::new(self, rhs)
    }

    #[doc(hidden)]
    fn gt<Rhs>(
        &self,
        rhs: Rhs,
    ) -> CompareExpr<&Self, Rhs::Expr, Self::ScalarType, GtOp>
    where
        Self::ScalarType: PartialOrd,
        Rhs: CompareRhs<Self::ScalarType>,
    {
        let rhs = rhs.into_compare_expr(self.nrows(), self.ncols());
        CompareExpr::new(self, rhs)
    }

    #[doc(hidden)]
    fn ge<Rhs>(
        &self,
        rhs: Rhs,
    ) -> CompareExpr<&Self, Rhs::Expr, Self::ScalarType, GeOp>
    where
        Self::ScalarType: PartialOrd,
        Rhs: CompareRhs<Self::ScalarType>,
    {
        let rhs = rhs.into_compare_expr(self.nrows(), self.ncols());
        CompareExpr::new(self, rhs)
    }
}

impl<X> ElementwiseCompare for X where X: MatrixExpr {}

#[doc(hidden)]
impl<A, B, T, Op> From<CompareExpr<A, B, T, Op>> for DMatrix<bool>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    fn from(expr: CompareExpr<A, B, T, Op>) -> Self {
        let mut out = DMatrix::from_value(false, expr.nrows, expr.ncols);
        expr.eval_into(&mut out.data);
        out
    }
}

#[doc(hidden)]
impl<A, B, T, Op, const N: usize, const M: usize> From<CompareExpr<A, B, T, Op>>
    for SMatrix<bool, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    fn from(expr: CompareExpr<A, B, T, Op>) -> Self {
        assert_eq!(
            (N, M),
            (expr.nrows, expr.ncols),
            "comparison result dimension mismatch: destination is {}x{}, expression is {}x{}",
            N,
            M,
            expr.nrows,
            expr.ncols
        );
        let mut out = SMatrix::from_value(false);
        expr.eval_into(out.as_mut_slice());
        out
    }
}
