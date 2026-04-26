//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::apply_for_all_types;
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
//{{{ impl: Add<T> for SMatrix
impl<T, const N: usize, const M: usize> Add<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn add(
        self,
        rhs: T,
    ) -> Self::Output
    {
        let mut out = self;
        out.iter_mut().for_each(|x| *x += rhs);
        out
    }
}
//}}}
//{{{ impl: Add<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Add for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + IndexValue<usize, Output = T>,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn add(
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
                *out_elem += *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Add<SMatrix<T, N, M>> for T
macro_rules! impl_smatrix_add_owned {
    ($type: ty) => {
        #[doc(hidden)]
        impl<const N: usize, const M: usize> Add<SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn add(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                let mut out = rhs;
                out.iter_mut().for_each(|x| *x += self);
                out
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_add_owned);
//}}}
//{{{ impl AddAssign<T> for SMatrix
impl<T, const N: usize, const M: usize> AddAssign<T> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: AddAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> AddAssign for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    #[inline]
    fn add_assign(
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
                *out_elem += *rhs_elem;
            });
    }
}
//}}}
//}}}
//{{{ collection: AddOp for SMatrix
//{{{ impl: Add<T> for SMatrix
macro_rules! impl_smatrix_add_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Add<$type> for &'a SMatrix<$type, N, M>
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, $type, $type, AddOp>;

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
        impl<'a, const N: usize, const M: usize> Add<$type> for &'a mut SMatrix<$type, N, M>
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, $type, $type, AddOp>;

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

apply_for_all_types!(impl_smatrix_add_scalar_rhs);

//}}}
//{{{ impl: Add<T> for &'a mut SMatrix
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
//{{{ impl: Add<&mut Smatrix> for $type
macro_rules! impl_smatrix_add_mut {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Add<&'a mut SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: &'a mut SMatrix<$type, N, M>,
            ) -> Self::Output
            {
                self.add(&*rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_add_mut);

//}}}
//{{{ impl: Add<Rhs> for &'a SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Add<Rhs> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, AddOp>;

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
//{{{ impl: Add<Rhs> for &'a mut SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Add<Rhs> for &'a mut SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
    Rhs: LazyExpr<ScalarType = T> + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, AddOp>;

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
