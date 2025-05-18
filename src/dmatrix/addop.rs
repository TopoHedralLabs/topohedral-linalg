//! This module implements the `Add` trait for the `DMatrix` struct.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue};
use crate::expression::binary_expr::{AddOp, BinOp, BinopExpr};
//}}}
//{{{ std imports
use std::ops::{Add, AddAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: eagerly evaluated expressions
//{{{ impl: Add<T> for DMatrix
impl<T> Add<T> for DMatrix<T>
where
    T: Field + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn add(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self.clone();
        out.iter_mut().for_each(|x| *x += rhs);
        out
    }
}
//}}}
//{{{ impl: Add<DMatrix> for DMatrix
impl<T> Add for DMatrix<T>
where
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = DMatrix<T>;

    #[inline]
    fn add(
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
                *out_elem += *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Add<DMatrix<T>> for T
macro_rules! impl_dmatrix_add {
    ($type: ty) => {
        #[doc(hidden)]
        impl Add<DMatrix<$type>> for $type
        {
            type Output = DMatrix<$type>;

            #[inline]
            fn add(
                self,
                rhs: DMatrix<$type>,
            ) -> Self::Output
            {
                let mut out = rhs.clone();
                out.iter_mut().for_each(|x| *x += self);
                out
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_add);
//}}}
//{{{ impl AddAssign<T> for DMatrix
impl<T> AddAssign<T> for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn add_assign(
        &mut self,
        rhs: T,
    )
    {
        self.iter_mut().for_each(|x| *x += rhs);
    }
}
//}}}
//{{{ impl: AddAssign<DMatrix> for DMatrix
impl<T> AddAssign for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn add_assign(
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
                *out_elem += *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: Lazily evaluated expressions
//{{{ impl: Add<T> for &'a DMatrix
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
macro_rules! impl_dmatrix_ref_add {
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
apply_for_all_types!(impl_dmatrix_ref_add);
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
