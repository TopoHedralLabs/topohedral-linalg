//! Subtraction operators for [`SMatrix`]: matrix − matrix and matrix − scalar.
//!
//! Mirrors `addop` for subtraction. Implements [`Sub`] for all owned/borrowed combinations of
//! [`SMatrix<T, N, M>`] and for mixed matrix–scalar operands. Compile-time dimension parameters
//! enforce shape compatibility. Matrix–matrix subtraction is lazy, returning a `BinopExpr`;
//! scalar subtraction is applied element-wise.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, MatrixExpr, ScalarExpr};
use crate::expression::binary_expr::{BinOp, BinopExpr, SubOp};
use crate::expression::outer_product_expr::OuterProductExpr;
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
use crate::smatrix::SMatrix;
//}}}
//{{{ std imports
use std::ops::{Sub, SubAssign};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fun: sub_assign_expr
#[inline]
fn sub_assign_expr<T, Rhs, const N: usize, const M: usize>(
    lhs: &mut SMatrix<T, N, M>,
    rhs: Rhs,
) where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    let rhs_nrows = rhs.nrows();
    let rhs_ncols = rhs.ncols();
    if lhs.nrows != rhs_nrows || lhs.ncols != rhs_ncols {
        panic!(
            "SMatrix::sub_assign dimension mismatch: lhs is {}x{}, rhs is {}x{}",
            lhs.nrows, lhs.ncols, rhs_nrows, rhs_ncols
        );
    }

    let out = lhs.as_mut_slice();
    for i in 0..out.len() {
        unsafe {
            *out.get_unchecked_mut(i) -= rhs.linear_value(i);
        }
    }
}
//}}}

//{{{ collection: eagerly evaluated expressions
//{{{ impl: Sub<T> for SMatrix
impl<T, const N: usize, const M: usize> Sub<T> for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn sub(
        self,
        rhs: T,
    ) -> Self::Output {
        let mut out = self;
        out.iter_mut().for_each(|x| *x -= rhs);
        out
    }
}
//}}}
//{{{ impl: Sub<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Sub for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn sub(
        self,
        rhs: SMatrix<T, N, M>,
    ) -> Self::Output {
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
        impl<const N: usize, const M: usize> Sub<SMatrix<$type, N, M>> for $type {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn sub(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output {
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
    T: Field + Copy,
{
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: T,
    ) {
        self.iter_mut().for_each(|x| *x -= rhs);
    }
}
//}}}
//{{{ impl: SubAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> SubAssign for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: SMatrix<T, N, M>,
    ) {
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
//{{{ impl: SubAssign<BinopExpr> for SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> SubAssign<BinopExpr<A, B, T, Op>>
    for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: BinOp,
{
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: BinopExpr<A, B, T, Op>,
    ) {
        sub_assign_expr(self, rhs);
    }
}
//}}}
//{{{ impl: SubAssign<UnaryExpr> for SMatrix
impl<A, T, Op, const N: usize, const M: usize> SubAssign<UnaryExpr<A, T, Op>> for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: UnaryExpr<A, T, Op>,
    ) {
        sub_assign_expr(self, rhs);
    }
}
//}}}
//{{{ impl: SubAssign<OuterProductExpr> for SMatrix
impl<L, R, T, const N: usize, const M: usize> SubAssign<OuterProductExpr<L, R, T>>
    for SMatrix<T, N, M>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    #[inline]
    fn sub_assign(
        &mut self,
        rhs: OuterProductExpr<L, R, T>,
    ) {
        sub_assign_expr(self, rhs);
    }
}
//}}}
//}}}
//{{{ collection: SubOp for SMatrix
//{{{ impl: Sub<T> for SMatrix
macro_rules! impl_smatrix_sub_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<$type> for &'a SMatrix<$type, N, M> {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, ScalarExpr<$type>, $type, SubOp>;

            fn sub(
                self,
                rhs: $type,
            ) -> Self::Output {
                let nr = self.nrows;
                let nc = self.ncols;
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<$type> for &'a mut SMatrix<$type, N, M> {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, ScalarExpr<$type>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: $type,
            ) -> Self::Output {
                (&*self).sub(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_sub_scalar_rhs);

//}}}
//{{{ impl: Sub<Smatrix> for $type
macro_rules! impl_smatrix_sub {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<&'a SMatrix<$type, N, M>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a SMatrix<$type, N, M>, $type, SubOp>;

            fn sub(
                self,
                rhs: &'a SMatrix<$type, N, M>,
            ) -> Self::Output {
                let nr = rhs.nrows;
                let nc = rhs.ncols;
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
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
//}}}
//{{{ impl: Sub<&mut Smatrix> for $type
macro_rules! impl_smatrix_sub_mut {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<&'a mut SMatrix<$type, N, M>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a SMatrix<$type, N, M>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: &'a mut SMatrix<$type, N, M>,
            ) -> Self::Output {
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
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: Rhs,
    ) -> Self::Output {
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
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, SubOp>;

    #[inline]
    fn sub(
        self,
        rhs: Rhs,
    ) -> Self::Output {
        (&*self).sub(rhs)
    }
}

//}}}
//}}}
