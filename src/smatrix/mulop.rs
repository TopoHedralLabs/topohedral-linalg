//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue};
use crate::expression::binary_expr::{BinOp, BinopExpr, MulOp};
//}}}
//{{{ std imports
use std::ops::{Mul, MulAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: eagerly evaluated expressions
//{{{ impl: Mul<T> for SMatrix
impl<T, const N: usize, const M: usize> Mul<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn mul(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self;
        out.iter_mut().for_each(|x| *x *= rhs);
        out
    }
}
//}}}
//{{{ impl: Mul<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Mul for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn mul(
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
                *out_elem *= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Mul<SMatrix<T, N, M>> for T
macro_rules! impl_smatrix_mul_owned {
    ($type: ty) => {
        #[doc(hidden)]
        impl<const N: usize, const M: usize> Mul<SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn mul(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                let mut out = rhs;
                out.iter_mut().for_each(|x| *x *= self);
                out
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_mul_owned);
//}}}
//{{{ impl MulAssign<T> for SMatrix
impl<T, const N: usize, const M: usize> MulAssign<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    #[inline]
    fn mul_assign(
        &mut self,
        rhs: T,
    )
    {
        self.iter_mut().for_each(|x| *x *= rhs);
    }
}
//}}}
//{{{ impl: MulAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> MulAssign for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    #[inline]
    fn mul_assign(
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
                *out_elem *= *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: MulOp for SMatrix
//{{{ impl: Mul<T> for SMatrix
#[doc(hidden)]
impl<'a, T, const N: usize, const M: usize> Mul<T> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, MulOp>;

    fn mul(
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
//{{{ impl: Mul<Smatrix> for $type
macro_rules! impl_smatrix_mul {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Mul<&'a SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, MulOp>;

            fn mul(
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

apply_for_all_types!(impl_smatrix_mul);

//}}}
//{{{ impl: Mul for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Mul for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, MulOp>;

    fn mul(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, MulOp>
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
//{{{ impl: Mul<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Mul<&'a SMatrix<T, N, M>>
    for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, MulOp>;

    fn mul(
        self,
        rhs: &'a SMatrix<T, N, M>,
    ) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, MulOp>
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
//{{{ impl: Mul<BinopExpr> for &'a SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> Mul<BinopExpr<A, B, T, Op>> for &SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>;

    fn mul(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>
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
