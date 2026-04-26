//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue, LazyExpr};
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::expression::binary_expr::{BinopExpr, SubOp};
//}}}
//{{{ std imports
use std::ops::{Sub, SubAssign};
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: eagerly evaluated expressions
//{{{ impl: Sub<T> for SMatrix
impl<T, const N: usize, const M: usize> Sub<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn sub(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self;
        out.iter_mut().for_each(|x| *x -= rhs);
        out
    }
}
//}}}
//{{{ impl: Sub<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Sub for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn sub(
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
                *out_elem -= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Sub<SMatrix<T, N, M>> for T
macro_rules! impl_smatrix_sub_owned {
    ($type: ty) => {
        #[doc(hidden)]
        impl<const N: usize, const M: usize> Sub<SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn sub(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                let mut out = rhs;
                out.iter_mut().for_each(|x| *x = self - *x);
                out
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_sub_owned);
//}}}
//{{{ impl SubAssign<T> for SMatrix
impl<T, const N: usize, const M: usize> SubAssign<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: SubAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> SubAssign for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    #[inline]
    fn sub_assign(
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
                *out_elem -= *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: SubOp for SMatrix
//{{{ impl: Sub<T> for SMatrix
macro_rules! impl_smatrix_sub_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<$type> for &'a SMatrix<$type, N, M>
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, $type, $type, SubOp>;

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
        impl<'a, const N: usize, const M: usize> Sub<$type> for &'a mut SMatrix<$type, N, M>
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, $type, $type, SubOp>;

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

apply_for_all_types!(impl_smatrix_sub_scalar_rhs);

//}}}
//{{{ impl: Sub<T> for &'a mut SMatrix
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
//{{{ impl: Sub<&mut Smatrix> for $type
macro_rules! impl_smatrix_sub_mut {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<&'a mut SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: &'a mut SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                self.sub(&*rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_sub_mut);

//}}}
//{{{ impl: Sub<Rhs> for &'a SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Sub<Rhs> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, SubOp>;

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
//{{{ impl: Sub<Rhs> for &'a mut SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Sub<Rhs> for &'a mut SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, SubOp>;

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
