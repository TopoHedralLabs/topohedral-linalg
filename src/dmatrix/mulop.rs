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
use crate::expression::binary_expr::{BinopExpr, MulOp};
//}}}
//{{{ std imports
use std::ops::{Mul, MulAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: eagerly evaluated expressions
//{{{ impl: Mul<T> for DMatrix
impl<T> Mul<T> for DMatrix<T>
where
    T: Field + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn mul(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self.clone();
        out.iter_mut().for_each(|x| *x *= rhs);
        out
    }
}
//}}}
//{{{ impl: Mul<DMatrix> for DMatrix
impl<T> Mul for DMatrix<T>
where
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = DMatrix<T>;

    #[inline]
    fn mul(
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
                *out_elem *= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Mul<DMatrix<T>> for T
macro_rules! impl_dmatrix_scalar_mul {
    ($type: ty) => {
        #[doc(hidden)]
        impl Mul<DMatrix<$type>> for $type
        {
            type Output = DMatrix<$type>;

            #[inline]
            fn mul(
                self,
                rhs: DMatrix<$type>,
            ) -> Self::Output
            {
                let mut out = rhs.clone();
                out.iter_mut().for_each(|x| *x *= self);
                out
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_scalar_mul);
//}}}
//{{{ impl MulAssign<T> for DMatrix
impl<T> MulAssign<T> for DMatrix<T>
where
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
//{{{ impl: MulAssign<DMatrix> for DMatrix
impl<T> MulAssign for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn mul_assign(
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
                *out_elem *= *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: MulOp for DMatrix
//{{{ impl: Mul<T> for DMatrix
macro_rules! impl_dmatrix_mul_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Mul<$type> for &'a DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, MulOp>;

            #[inline]
            fn mul(
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
        impl<'a> Mul<$type> for &'a mut DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: $type,
            ) -> Self::Output
            {
                (&*self).mul(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_dmatrix_mul_scalar_rhs);

//}}}
//{{{ impl: Mul<DMatrix> for $type
macro_rules! impl_dmatrix_mul {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Mul<&'a DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, MulOp>;

            #[inline]
            fn mul(
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
apply_for_all_types!(impl_dmatrix_mul);
//}}}
//{{{ impl: Mul<&mut DMatrix> for $type
macro_rules! impl_dmatrix_ref_mut_mul {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Mul<&'a mut DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: &'a mut DMatrix<$type>,
            ) -> Self::Output
            {
                self.mul(&*rhs)
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_ref_mut_mul);
//}}}
//{{{ impl: Mul<Rhs> for &'a DMatrix
impl<'a, T, Rhs> Mul<Rhs> for &'a DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, MulOp>;

    #[inline]
    fn mul(
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
//{{{ impl: Mul<Rhs> for &'a mut DMatrix
impl<'a, T, Rhs> Mul<Rhs> for &'a mut DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: Rhs,
    ) -> Self::Output
    {
        (&*self).mul(rhs)
    }
}

//}}}
//}}}
