//! Lazy boolean mask composition.

use crate::common::{MatrixExpr, Shape};
use crate::expression::comparison_expr::{CompareExpr, CompareOp};
use crate::{DMatrix, SMatrix};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitOr, BitXor, Not};

#[doc(hidden)]
pub trait BoolBinaryOp
{
    fn apply(
        a: bool,
        b: bool,
    ) -> bool;
}

#[doc(hidden)]
pub struct AndOp;
#[doc(hidden)]
pub struct OrOp;
#[doc(hidden)]
pub struct XorOp;

impl BoolBinaryOp for AndOp
{
    fn apply(
        a: bool,
        b: bool,
    ) -> bool
    {
        a & b
    }
}

impl BoolBinaryOp for OrOp
{
    fn apply(
        a: bool,
        b: bool,
    ) -> bool
    {
        a | b
    }
}

impl BoolBinaryOp for XorOp
{
    fn apply(
        a: bool,
        b: bool,
    ) -> bool
    {
        a ^ b
    }
}

/// Lazy binary boolean expression.
pub struct BoolBinaryExpr<A, B, Op>
where
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    pub(crate) a: A,
    pub(crate) b: B,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
    pub(crate) _marker: PhantomData<Op>,
}

impl<A, B, Op> BoolBinaryExpr<A, B, Op>
where
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    fn new(
        a: A,
        b: B,
    ) -> Self
    {
        let nrows = a.nrows();
        let ncols = a.ncols();
        assert_eq!(
            (nrows, ncols),
            (b.nrows(), b.ncols()),
            "boolean expression dimension mismatch: lhs is {}x{}, rhs is {}x{}",
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

impl<A, B, Op> Shape for BoolBinaryExpr<A, B, Op>
where
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    fn nrows(&self) -> usize
    {
        self.nrows
    }

    fn ncols(&self) -> usize
    {
        self.ncols
    }
}

impl<A, B, Op> MatrixExpr for BoolBinaryExpr<A, B, Op>
where
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    type ScalarType = bool;

    fn linear_value(
        &self,
        index: usize,
    ) -> bool
    {
        Op::apply(self.a.linear_value(index), self.b.linear_value(index))
    }
}

/// Lazy element-wise boolean negation.
pub struct BoolNotExpr<A>
where
    A: MatrixExpr<ScalarType = bool>,
{
    pub(crate) a: A,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}

impl<A> BoolNotExpr<A>
where
    A: MatrixExpr<ScalarType = bool>,
{
    fn new(a: A) -> Self
    {
        let nrows = a.nrows();
        let ncols = a.ncols();
        Self { a, nrows, ncols }
    }
}

impl<A> Shape for BoolNotExpr<A>
where
    A: MatrixExpr<ScalarType = bool>,
{
    fn nrows(&self) -> usize
    {
        self.nrows
    }

    fn ncols(&self) -> usize
    {
        self.ncols
    }
}

impl<A> MatrixExpr for BoolNotExpr<A>
where
    A: MatrixExpr<ScalarType = bool>,
{
    type ScalarType = bool;

    fn linear_value(
        &self,
        index: usize,
    ) -> bool
    {
        !self.a.linear_value(index)
    }
}

macro_rules! impl_binary_ops_for_compare {
    ($trait:ident, $method:ident, $operation:ty) => {
        impl<A, B, T, Op, Rhs> $trait<Rhs> for CompareExpr<A, B, T, Op>
        where
            A: MatrixExpr<ScalarType = T>,
            B: MatrixExpr<ScalarType = T>,
            T: Copy,
            Op: CompareOp<T>,
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, $operation>;

            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }
    };
}

impl_binary_ops_for_compare!(BitAnd, bitand, AndOp);
impl_binary_ops_for_compare!(BitOr, bitor, OrOp);
impl_binary_ops_for_compare!(BitXor, bitxor, XorOp);

impl<A, B, T, Op> Not for CompareExpr<A, B, T, Op>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Copy,
    Op: CompareOp<T>,
{
    type Output = BoolNotExpr<Self>;

    fn not(self) -> Self::Output
    {
        BoolNotExpr::new(self)
    }
}

macro_rules! impl_binary_ops_for_binary_expr {
    ($trait:ident, $method:ident, $operation:ty) => {
        impl<A, B, Op, Rhs> $trait<Rhs> for BoolBinaryExpr<A, B, Op>
        where
            A: MatrixExpr<ScalarType = bool>,
            B: MatrixExpr<ScalarType = bool>,
            Op: BoolBinaryOp,
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, $operation>;

            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }
    };
}

impl_binary_ops_for_binary_expr!(BitAnd, bitand, AndOp);
impl_binary_ops_for_binary_expr!(BitOr, bitor, OrOp);
impl_binary_ops_for_binary_expr!(BitXor, bitxor, XorOp);

impl<A, B, Op> Not for BoolBinaryExpr<A, B, Op>
where
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    type Output = BoolNotExpr<Self>;

    fn not(self) -> Self::Output
    {
        BoolNotExpr::new(self)
    }
}

macro_rules! impl_binary_ops_for_not_expr {
    ($trait:ident, $method:ident, $operation:ty) => {
        impl<A, Rhs> $trait<Rhs> for BoolNotExpr<A>
        where
            A: MatrixExpr<ScalarType = bool>,
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, $operation>;

            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }
    };
}

impl_binary_ops_for_not_expr!(BitAnd, bitand, AndOp);
impl_binary_ops_for_not_expr!(BitOr, bitor, OrOp);
impl_binary_ops_for_not_expr!(BitXor, bitxor, XorOp);

impl<A> Not for BoolNotExpr<A>
where
    A: MatrixExpr<ScalarType = bool>,
{
    type Output = BoolNotExpr<Self>;

    fn not(self) -> Self::Output
    {
        BoolNotExpr::new(self)
    }
}

macro_rules! impl_bool_ops_for_dmatrix {
    ($lhs:ty) => {
        impl<Rhs> BitAnd<Rhs> for $lhs
        where
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, AndOp>;

            fn bitand(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }

        impl<Rhs> BitOr<Rhs> for $lhs
        where
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, OrOp>;

            fn bitor(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }

        impl<Rhs> BitXor<Rhs> for $lhs
        where
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, XorOp>;

            fn bitxor(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }

        impl Not for $lhs
        {
            type Output = BoolNotExpr<Self>;

            fn not(self) -> Self::Output
            {
                BoolNotExpr::new(self)
            }
        }
    };
}

impl_bool_ops_for_dmatrix!(DMatrix<bool>);
impl_bool_ops_for_dmatrix!(&DMatrix<bool>);
impl_bool_ops_for_dmatrix!(&mut DMatrix<bool>);

macro_rules! impl_bool_ops_for_smatrix {
    ($lhs:ty) => {
        impl<const N: usize, const M: usize, Rhs> BitAnd<Rhs> for $lhs
        where
            [(); N * M]:,
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, AndOp>;

            fn bitand(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }

        impl<const N: usize, const M: usize, Rhs> BitOr<Rhs> for $lhs
        where
            [(); N * M]:,
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, OrOp>;

            fn bitor(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }

        impl<const N: usize, const M: usize, Rhs> BitXor<Rhs> for $lhs
        where
            [(); N * M]:,
            Rhs: MatrixExpr<ScalarType = bool>,
        {
            type Output = BoolBinaryExpr<Self, Rhs, XorOp>;

            fn bitxor(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                BoolBinaryExpr::new(self, rhs)
            }
        }

        impl<const N: usize, const M: usize> Not for $lhs
        where
            [(); N * M]:,
        {
            type Output = BoolNotExpr<Self>;

            fn not(self) -> Self::Output
            {
                BoolNotExpr::new(self)
            }
        }
    };
}

impl_bool_ops_for_smatrix!(SMatrix<bool, N, M>);
impl_bool_ops_for_smatrix!(&SMatrix<bool, N, M>);
impl_bool_ops_for_smatrix!(&mut SMatrix<bool, N, M>);

impl<A, B, Op> From<BoolBinaryExpr<A, B, Op>> for DMatrix<bool>
where
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    fn from(expr: BoolBinaryExpr<A, B, Op>) -> Self
    {
        let mut out = DMatrix::from_value(false, expr.nrows, expr.ncols);
        expr.eval_into(&mut out.data);
        out
    }
}

impl<A> From<BoolNotExpr<A>> for DMatrix<bool>
where
    A: MatrixExpr<ScalarType = bool>,
{
    fn from(expr: BoolNotExpr<A>) -> Self
    {
        let mut out = DMatrix::from_value(false, expr.nrows, expr.ncols);
        expr.eval_into(&mut out.data);
        out
    }
}

impl<A, B, Op, const N: usize, const M: usize> From<BoolBinaryExpr<A, B, Op>>
    for SMatrix<bool, N, M>
where
    [(); N * M]:,
    A: MatrixExpr<ScalarType = bool>,
    B: MatrixExpr<ScalarType = bool>,
    Op: BoolBinaryOp,
{
    fn from(expr: BoolBinaryExpr<A, B, Op>) -> Self
    {
        assert_eq!((N, M), (expr.nrows, expr.ncols));
        let mut out = SMatrix::from_value(false);
        expr.eval_into(&mut out.data);
        out
    }
}

impl<A, const N: usize, const M: usize> From<BoolNotExpr<A>> for SMatrix<bool, N, M>
where
    [(); N * M]:,
    A: MatrixExpr<ScalarType = bool>,
{
    fn from(expr: BoolNotExpr<A>) -> Self
    {
        assert_eq!((N, M), (expr.nrows, expr.ncols));
        let mut out = SMatrix::from_value(false);
        expr.eval_into(&mut out.data);
        out
    }
}
