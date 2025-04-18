//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, IndexValue};
use crate::expression::binary_expr::{AddOp, BinOp, BinopExpr };
use super::SMatrix;
use crate::apply_for_all_types;
//}}}
//{{{ std imports 
use std::ops::Add;
use std::fmt;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: AddOp for SMatrix
//{{{ impl: Add<T> for SMatrix
#[doc(hidden)]
impl<'a, T, const N: usize, const M: usize> Add<T> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, AddOp>;

    #[inline]

    fn add(
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
//{{{ impl: Add<Smatrix> for $type
macro_rules! impl_smatrix_add {
    ($type:ty) => {
        #[doc(hidden)]

        impl<'a, const N: usize, const M: usize> Add<&'a SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, AddOp>;

            #[inline]

            fn add(
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

apply_for_all_types!(impl_smatrix_add);

//}}}
//{{{ impl: Add for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Add for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, AddOp>
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
//{{{ impl: Add<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Add<&'a SMatrix<T, N, M>>
    for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: &'a SMatrix<T, N, M>,
    ) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, AddOp>
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
//{{{ impl: Add<BinopExpr> for &'a SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> Add<BinopExpr<A, B, T, Op>> for &SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>
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