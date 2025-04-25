//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue};
use crate::expression::binary_expr::{AddOp, BinOp, BinopExpr};
//}}}
//{{{ std imports
use std::ops::Add;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: AddOp for SMatrix
//{{{ impl: Add<T> for SMatrix
#[doc(hidden)]
impl<'a, T> Add<T> for &'a DMatrix<T>
where
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, T, T, AddOp>;

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
//{{{ impl: Add<Dmatrix> for $type
macro_rules! impl_dmatrix_add {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Add<&'a DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, AddOp>;

            #[inline]
            fn add(
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
apply_for_all_types!(impl_dmatrix_add);
//}}}
//{{{ impl: Add for &'a DMatrix
impl<'a, T> Add for &'a DMatrix<T>
where
    T: Field + Copy,
{
    type Output = BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, AddOp>
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
//{{{ impl: Add<&' DMatrix> for BinopExpr
impl<'a, A, B, T, Op> Add<&'a DMatrix<T>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
{
    type Output = BinopExpr<Self, &'a DMatrix<T>, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: &'a DMatrix<T>,
    ) -> BinopExpr<Self, &'a DMatrix<T>, T, AddOp>
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
//{{{ impl: Add<BinopExpr> for &'a DMatrix
impl<A, B, T, Op> Add<BinopExpr<A, B, T, Op>> for &DMatrix<T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy,
    Op: BinOp,
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
