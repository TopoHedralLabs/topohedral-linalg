//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue};
use crate::expression::binary_expr::{BinOp, BinopExpr, DivOp};
//}}}
//{{{ std imports
use std::ops::{Div, DivAssign};
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: eagerly evaluated expressions
//{{{ impl: Div<T> for SMatrix
impl<T, const N: usize, const M: usize> Div<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn div(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self;
        out.iter_mut().for_each(|x| *x /= rhs);
        out
    }
}
//}}}
//{{{ impl: Div<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Div for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn div(
        self,
        rhs: SMatrix<T, N, M>,
    ) -> Self::Output
    {
        //{{{ check: assert dimensions are equal
        #[cfg(feature = "enable_checks")]
        {
            assert_eq!(self.nrows, rhs.nrows);
            assert_eq!(self.ncols, rhs.ncols);
        }
        //}}}
        let mut out = self;
        out.iter_mut()
            .zip(rhs.iter())
            .for_each(|(out_elem, rhs_elem)| {
                *out_elem /= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Div<SMatrix<T, N, M>> for T
macro_rules! impl_smatrix_div_owned {
    ($type: ty) => {
        #[doc(hidden)]
        impl<const N: usize, const M: usize> Div<SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn div(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                let mut out = rhs;
                out.iter_mut().for_each(|x| *x = self / *x);
                out
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_div_owned);
//}}}
//{{{ impl DivAssign<T> for SMatrix
impl<T, const N: usize, const M: usize> DivAssign<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: DivAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> DivAssign for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    #[inline]
    fn div_assign(
        &mut self,
        rhs: SMatrix<T, N, M>,
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
//{{{ collection: DivOp for SMatrix
//{{{ impl: Div<T> for SMatrix
#[doc(hidden)]
impl<'a, T, const N: usize, const M: usize> Div<T> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, DivOp>;

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
//{{{ impl: Div<Smatrix> for $type
macro_rules! impl_smatrix_div {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Div<&'a SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, DivOp>;

            fn div(
                self,
                rhs: &'a SMatrix<$type, N, M>,
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

apply_for_all_types!(impl_smatrix_div);

//}}}
//{{{ impl: Div for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Div for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, DivOp>;

    fn div(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, DivOp>
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
//{{{ impl: Div<&' SMatrix> for BinopExpr
impl<'a, A, B, Op, T, const N: usize, const M: usize> Div<&'a SMatrix<T, N, M>>
    for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, DivOp>;

    fn div(
        self,
        rhs: &'a SMatrix<T, N, M>,
    ) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, DivOp>
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
//{{{ impl: Div<BinopExpr> for &'a SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> Div<BinopExpr<A, B, T, Op>> for &SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, DivOp>;

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
