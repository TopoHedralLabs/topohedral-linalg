//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue, LazyExpr};
use crate::expression::binary_expr::{BinopExpr, SubOp};
//}}}
//{{{ std imports
use std::ops::{Sub, SubAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: eagerly evaluated expressions
//{{{ impl: Sub<T> for DMatrix
impl<T> Sub<T> for DMatrix<T>
where
    T: Field + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn sub(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self.clone();
        out.iter_mut().for_each(|x| *x -= rhs);
        out
    }
}
//}}}
//{{{ impl: Sub<DMatrix> for DMatrix
impl<T> Sub for DMatrix<T>
where
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = DMatrix<T>;

    #[inline]
    fn sub(
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
                *out_elem -= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Sub<DMatrix<T>> for T
macro_rules! impl_dmatrix_sub {
    ($type: ty) => {
        #[doc(hidden)]
        impl Sub<DMatrix<$type>> for $type
        {
            type Output = DMatrix<$type>;

            #[inline]
            fn sub(
                self,
                rhs: DMatrix<$type>,
            ) -> Self::Output
            {
                let mut out = rhs.clone();
                out.iter_mut().for_each(|x| *x = self - *x);
                out
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_sub);
//}}}
//{{{ impl SubAssign<T> for DMatrix
impl<T> SubAssign<T> for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: T,
    )
    {
        self.iter_mut().for_each(|x| *x -= rhs);
    }
}
//}}}
//{{{ impl: SubAssign<DMatrix> for DMatrix
impl<T> SubAssign for DMatrix<T>
where
    T: Field + Copy,
{
    #[inline]
    fn sub_assign(
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
                *out_elem = *out_elem - *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: SubOp for DMatrix
//{{{ impl: Sub<T> for DMatrix
macro_rules! impl_dmatrix_sub_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Sub<$type> for &'a DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, SubOp>;

            #[inline]
            fn sub(
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
        impl<'a> Sub<$type> for &'a mut DMatrix<$type>
        {
            type Output = BinopExpr<&'a DMatrix<$type>, $type, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: $type,
            ) -> Self::Output
            {
                (&*self).sub(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_dmatrix_sub_scalar_rhs);

//}}}
//{{{ impl: Sub<T> for &'a mut DMatrix
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
//{{{ impl: Sub<&mut DMatrix> for $type
macro_rules! impl_dmatrix_ref_mut_sub {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Sub<&'a mut DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: &'a mut DMatrix<$type>,
            ) -> Self::Output
            {
                self.sub(&*rhs)
            }
        }
    };
}
apply_for_all_types!(impl_dmatrix_ref_mut_sub);
//}}}
//{{{ impl: Sub<Rhs> for &'a DMatrix
impl<'a, T, Rhs> Sub<Rhs> for &'a DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, SubOp>;

    #[inline]
    fn sub(
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
//{{{ impl: Sub<Rhs> for &'a mut DMatrix
impl<'a, T, Rhs> Sub<Rhs> for &'a mut DMatrix<T>
where
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: Rhs,
    ) -> Self::Output
    {
        (&*self).sub(rhs)
    }
}

//}}}
//}}}
