//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::apply_for_all_types;
use crate::common::{Field, IndexValue, LazyExpr};
use crate::expression::binary_expr::{BinopExpr, DivOp};
//}}}
//{{{ std imports
use std::ops::{Div, DivAssign};
//}}}
//--------------------------------------------------------------------------------------------------
//{{{ collection: eagerly evaluated expressions
//{{{ impl: Div<T> for SMatrix
impl<T, const N: usize, const M: usize> Div<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn div(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self;
        out.iter_mut().for_each(|x| *x /= rhs);
        out
    }
}
//}}}
//{{{ impl: Div<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Div for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn div(
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
                *out_elem /= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Div<SMatrix<T, N, M>> for T
macro_rules! impl_smatrix_div_owned {
    ($type: ty) => {
        #[doc(hidden)]
        impl<const N: usize, const M: usize> Div<SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn div(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                let mut out = rhs;
                out.iter_mut().for_each(|x| *x = self / *x);
                out
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_div_owned);
//}}}
//{{{ impl DivAssign<T> for SMatrix
impl<T, const N: usize, const M: usize> DivAssign<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: DivAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> DivAssign for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    #[inline]
    fn div_assign(
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
                *out_elem /= *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: DivOp for SMatrix
//{{{ impl: Div<T> for SMatrix
macro_rules! impl_smatrix_div_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Div<$type> for &'a SMatrix<$type, N, M>
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, $type, $type, DivOp>;

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
        impl<'a, const N: usize, const M: usize> Div<$type> for &'a mut SMatrix<$type, N, M>
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, $type, $type, DivOp>;

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

apply_for_all_types!(impl_smatrix_div_scalar_rhs);

//}}}
//{{{ impl: Div<T> for &'a mut SMatrix
//{{{ impl: Div<Smatrix> for $type
macro_rules! impl_smatrix_div {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Div<&'a SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, DivOp>;

            fn div(
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

apply_for_all_types!(impl_smatrix_div);
//{{{ impl: Div<&mut Smatrix> for $type
macro_rules! impl_smatrix_div_mut {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Div<&'a mut SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, DivOp>;

            #[inline]
            fn div(
                self,
                rhs: &'a mut SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                self.div(&*rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_div_mut);

//}}}
//{{{ impl: Div<Rhs> for &'a SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Div<Rhs> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, DivOp>;

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
//{{{ impl: Div<Rhs> for &'a mut SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Div<Rhs> for &'a mut SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, DivOp>;

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
