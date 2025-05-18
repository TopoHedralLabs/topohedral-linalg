//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue};
use crate::expression::binary_expr::{BinOp, BinopExpr, DivOp};
//}}}
//{{{ std imports
use std::ops::{Div, DivAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: eagerly evaluated expressions
//{{{ impl: Div<T> for DMatrix
impl<T> Div<T> for DMatrix<T>
where
    T: Field + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn div(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self.clone();
        out.iter_mut().for_each(|x| *x /= rhs);
        out
    }
}
//}}}
//{{{ impl: Div<DMatrix> for DMatrix
impl<T> Div for DMatrix<T>
where
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = DMatrix<T>;

    #[inline]
    fn div(
        self,
        rhs: DMatrix<T>,
    ) -> Self::Output
    {
        //{{{ check: assert dimensions are equal
        #[cfg(feature = "enable_checks")]
        {
            assert_eq!(self.nrows, rhs.nrows);
            assert_eq!(self.ncols, rhs.ncols);
        }
        //}}}
        let mut out = self.clone();
        out.iter_mut()
            .zip(rhs.iter())
            .for_each(|(out_elem, rhs_elem)| {
                *out_elem /= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Div<DMatrix<T>> for T
macro_rules! impl_dmatrix_div_scalar {
    ($type: ty) => {
        #[doc(hidden)]
        impl Div<DMatrix<$type>> for $type
        {
            type Output = DMatrix<$type>;

            #[inline]
            fn div(
                self,
                rhs: DMatrix<$type>,
            ) -> Self::Output
            {
                let mut out = rhs.clone();
                out.iter_mut().for_each(|x| *x = self / *x);
                out
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_div_scalar);
//}}}
//{{{ impl DivAssign<T> for DMatrix
impl<T> DivAssign<T> for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn div_assign(
        &mut self,
        rhs: T,
    )
    {
        self.iter_mut().for_each(|x| *x /= rhs);
    }
}
//}}}
//{{{ impl: DivAssign<DMatrix> for DMatrix
impl<T> DivAssign for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn div_assign(
        &mut self,
        rhs: DMatrix<T>,
    )
    {
        //{{{ check: assert dimensions are equal
        #[cfg(feature = "enable_checks")]
        {
            assert_eq!(self.nrows, rhs.nrows);
            assert_eq!(self.ncols, rhs.ncols);
        }
        //}}}
        self.iter_mut()
            .zip(rhs.iter())
            .for_each(|(out_elem, rhs_elem)| {
                *out_elem /= *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: DivOp for DMatrix
//{{{ impl: Div<T> for DMatrix
#[doc(hidden)]
impl<'a, T> Div<T> for &'a DMatrix<T>
where
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, T, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: T,
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

//}}}
//{{{ impl: Div<DMatrix> for $type
macro_rules! impl_dmatrix_div {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Div<&'a DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: &'a DMatrix<$type>,
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
apply_for_all_types!(impl_dmatrix_div);
//}}}
//{{{ impl: Div for &'a DMatrix
impl<'a, T> Div for &'a DMatrix<T>
where
    T: Field + Copy,
{
    type Output = BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, DivOp>
    {
        let nr = self.nrows;
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

//}}}
//{{{ impl: Div<&' DMatrix> for BinopExpr
impl<'a, A, B, T, Op> Div<&'a DMatrix<T>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    type Output = BinopExpr<Self, &'a DMatrix<T>, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: &'a DMatrix<T>,
    ) -> BinopExpr<Self, &'a DMatrix<T>, T, DivOp>
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

//}}}
//{{{ impl: Div<BinopExpr> for &'a DMatrix
impl<A, B, T, Op> Div<BinopExpr<A, B, T, Op>> for &DMatrix<T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, DivOp>
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

//}}}
//}}}
