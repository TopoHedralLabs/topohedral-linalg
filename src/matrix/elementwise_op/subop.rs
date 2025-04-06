//! This mdule implements the element-wise subtraction operation for the `SMatrix` and `DMatrix` 
//! types.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::super::dmatrix::*;
use super::super::smatrix::*;
use super::common::{BinOp, BinopExpr, SubOp};
use crate::apply_for_all_types;
use crate::common::*;
//}}}
//{{{ std imports
use std::fmt;
use std::ops::Sub;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: SubOp for SMatrix
//{{{ impl: Sub<T> for SMatrix
#[doc(hidden)]

impl<'a, T, const N: usize, const M: usize> Sub<T> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, SubOp>;

    fn sub(
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
//{{{ impl: Sub<Smatrix> for $type
macro_rules! impl_smatrix_sub {
    ($type:ty) => {
        #[doc(hidden)]

        impl<'a, const N: usize, const M: usize> Sub<&'a SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, SubOp>;

            fn sub(
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

apply_for_all_types!(impl_smatrix_sub);

//}}}
//{{{ impl: Sub for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Sub for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, SubOp>;

    fn sub(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, SubOp>
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
//{{{ impl: Sub<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Sub<&'a SMatrix<T, N, M>>
    for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, SubOp>;

    fn sub(
        self,
        rhs: &'a SMatrix<T, N, M>,
    ) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, SubOp>
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
//{{{ impl: Sub<BinopExpr> for &'a SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> Sub<BinopExpr<A, B, T, Op>> for &SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, SubOp>;

    fn sub(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, SubOp>
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
//{{{ collection: SubOp for DMatrix
//{{{ impl: Sub<T> for DMatrix
#[doc(hidden)]
impl<'a, T> Sub<T> for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, T, T, SubOp>;

    #[inline]
    fn sub(
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
//{{{ impl: Sub<DMatrix> for $type
macro_rules! impl_dmatrix_sub {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Sub<&'a DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, SubOp>;

            #[inline]
            fn sub(
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

apply_for_all_types!(impl_dmatrix_sub);

//}}}
//{{{ impl: Sub for &'a DMatrix
impl<'a, T> Sub for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, SubOp>
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
//{{{ impl: Sub<&' DMatrix> for BinopExpr
impl<'a, A, B, T, Op> Sub<&'a DMatrix<T>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = BinopExpr<Self, &'a DMatrix<T>, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: &'a DMatrix<T>,
    ) -> BinopExpr<Self, &'a DMatrix<T>, T, SubOp>
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
//{{{ impl: Sub<BinopExpr> for &'a DMatrix
impl<A, B, T, Op> Sub<BinopExpr<A, B, T, Op>> for &DMatrix<T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, SubOp>
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

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

}
//}}}
