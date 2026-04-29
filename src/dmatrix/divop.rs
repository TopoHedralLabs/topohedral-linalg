//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, IndexValue, LazyExpr};
use crate::expression::binary_expr::{BinopExpr, DivOp};
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
macro_rules! impl_dmatrix_div_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Div<$type> for &'a DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, DivOp>;

            #[inline]
            fn div(
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
        impl<'a> Div<$type> for &'a mut DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: $type,
            ) -> Self::Output
            {
                (&*self).div(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_dmatrix_div_scalar_rhs);

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
//{{{ impl: Div<&mut DMatrix> for $type
macro_rules! impl_dmatrix_ref_mut_div {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Div<&'a mut DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: &'a mut DMatrix<$type>,
            ) -> Self::Output
            {
                self.div(&*rhs)
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_ref_mut_div);
//}}}
//{{{ impl: Div<Rhs> for &'a DMatrix
impl<'a, T, Rhs> Div<Rhs> for &'a DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, DivOp>;

    #[inline]
    fn div(
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
//{{{ impl: Div<Rhs> for &'a mut DMatrix
impl<'a, T, Rhs> Div<Rhs> for &'a mut DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, DivOp>;

    #[inline]
    fn div(
        self,
        rhs: Rhs,
    ) -> Self::Output
    {
        (&*self).div(rhs)
    }
}

//}}}
//}}}
