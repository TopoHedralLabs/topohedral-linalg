//! This module implements the `Add` trait for the `DMatrix` struct.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, IndexValue, LazyExpr};
use crate::expression::binary_expr::{AddOp, BinopExpr};
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
macro_rules! impl_dmatrix_add_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Add<$type> for &'a DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: $type,
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

        #[doc(hidden)]
        impl<'a> Add<$type> for &'a mut DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: $type,
            ) -> Self::Output
            {
                (&*self).add(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_dmatrix_add_scalar_rhs);

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
//{{{ impl: Add<&mut Dmatrix> for $type
macro_rules! impl_dmatrix_ref_mut_add {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Add<&'a mut DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: &'a mut DMatrix<$type>,
            ) -> Self::Output
            {
                self.add(&*rhs)
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_ref_mut_add);
//}}}
//{{{ impl: Add<Rhs> for &'a DMatrix
impl<'a, T, Rhs> Add<Rhs> for &'a DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: Rhs,
    ) -> Self::Output
    {
        #[cfg(feature = "enable_checks")]
        {
            assert_eq!(self.nrows(), rhs.nrows());
            assert_eq!(self.ncols(), rhs.ncols());
        }

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
//{{{ impl: Add<Rhs> for &'a mut DMatrix
impl<'a, T, Rhs> Add<Rhs> for &'a mut DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: Rhs,
    ) -> Self::Output
    {
        (&*self).add(rhs)
    }
}

//}}}
//}}}
