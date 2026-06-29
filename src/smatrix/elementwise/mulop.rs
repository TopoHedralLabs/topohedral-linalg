//! Element-wise multiplication operators for [`SMatrix`]: matrix * scalar and matrix * matrix.
//!
//! Implements the [`Mul`] trait for element-wise (Hadamard) multiplication of [`SMatrix<T, N, M>`]
//! operands and for scalar–matrix scaling. This is *not* matrix multiplication; for that see
//! `matmul`. Both operand combinations return a lazy `BinopExpr`. Const-generic dimensions
//! ensure shape correctness at compile time.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, MatrixExpr, ScalarExpr};
use crate::expression::binary_expr::{BinOp, BinopExpr, MulOp};
use crate::expression::outer_product_expr::OuterProductExpr;
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
use crate::smatrix::SMatrix;
//}}}
//{{{ std imports
use std::ops::{Mul, MulAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fun: mul_assign_expr
#[inline]
fn mul_assign_expr<T, Rhs, const N: usize, const M: usize>(
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
            "SMatrix::mul_assign dimension mismatch: lhs is {}x{}, rhs is {}x{}",
            lhs.nrows, lhs.ncols, rhs_nrows, rhs_ncols
        );
    }

    let out = lhs.as_mut_slice();
    for i in 0..out.len() {
        unsafe {
            *out.get_unchecked_mut(i) *= rhs.linear_value(i);
        }
    }
}
//}}}

//{{{ collection: eagerly evaluated expressions
//{{{ impl: Mul<T> for SMatrix
impl<T, const N: usize, const M: usize> Mul<T> for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn mul(
        self,
        rhs: T,
    ) -> Self::Output {
        let mut out = self;
        out.iter_mut().for_each(|x| *x *= rhs);
        out
    }
}
//}}}
//{{{ impl: Mul<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Mul for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn mul(
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
                *out_elem *= *rhs_elem;
            });

        out
    }
}
//}}}
//{{{ impl Mul<SMatrix<T, N, M>> for T
macro_rules! impl_smatrix_mul_owned {
    ($type: ty) => {
        #[doc(hidden)]
        impl<const N: usize, const M: usize> Mul<SMatrix<$type, N, M>> for $type {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn mul(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output {
                let mut out = rhs;
                out.iter_mut().for_each(|x| *x *= self);
                out
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_mul_owned);
//}}}
//{{{ impl MulAssign<T> for SMatrix
impl<T, const N: usize, const M: usize> MulAssign<T> for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    #[inline]
    fn mul_assign(
        &mut self,
        rhs: T,
    ) {
        self.iter_mut().for_each(|x| *x *= rhs);
    }
}
//}}}
//{{{ impl: MulAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> MulAssign for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    #[inline]
    fn mul_assign(
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
                *out_elem *= *rhs_elem;
            });
    }
}
//}}}
//{{{ impl: MulAssign<BinopExpr> for SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> MulAssign<BinopExpr<A, B, T, Op>>
    for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: BinOp,
{
    #[inline]
    fn mul_assign(
        &mut self,
        rhs: BinopExpr<A, B, T, Op>,
    ) {
        mul_assign_expr(self, rhs);
    }
}
//}}}
//{{{ impl: MulAssign<UnaryExpr> for SMatrix
impl<A, T, Op, const N: usize, const M: usize> MulAssign<UnaryExpr<A, T, Op>> for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    #[inline]
    fn mul_assign(
        &mut self,
        rhs: UnaryExpr<A, T, Op>,
    ) {
        mul_assign_expr(self, rhs);
    }
}
//}}}
//{{{ impl: MulAssign<OuterProductExpr> for SMatrix
impl<L, R, T, const N: usize, const M: usize> MulAssign<OuterProductExpr<L, R, T>>
    for SMatrix<T, N, M>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    #[inline]
    fn mul_assign(
        &mut self,
        rhs: OuterProductExpr<L, R, T>,
    ) {
        mul_assign_expr(self, rhs);
    }
}
//}}}
//}}}
//{{{ collection: MulOp for SMatrix
//{{{ impl: Mul<T> for SMatrix
macro_rules! impl_smatrix_mul_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Mul<$type> for &'a SMatrix<$type, N, M> {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, ScalarExpr<$type>, $type, MulOp>;

            fn mul(
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
        impl<'a, const N: usize, const M: usize> Mul<$type> for &'a mut SMatrix<$type, N, M> {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, ScalarExpr<$type>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: $type,
            ) -> Self::Output {
                (&*self).mul(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_mul_scalar_rhs);

//}}}
//{{{ impl: Mul<Smatrix> for $type
macro_rules! impl_smatrix_mul {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Mul<&'a SMatrix<$type, N, M>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a SMatrix<$type, N, M>, $type, MulOp>;

            fn mul(
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

apply_for_all_types!(impl_smatrix_mul);
//}}}
//{{{ impl: Mul<&mut Smatrix> for $type
macro_rules! impl_smatrix_mul_mut {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Mul<&'a mut SMatrix<$type, N, M>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a SMatrix<$type, N, M>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: &'a mut SMatrix<$type, N, M>,
            ) -> Self::Output {
                self.mul(&*rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_mul_mut);

//}}}
//{{{ impl: Mul<Rhs> for &'a SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Mul<Rhs> for &'a SMatrix<T, N, M>
where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, MulOp>;

    #[inline]
    fn mul(
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
//{{{ impl: Mul<Rhs> for &'a mut SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Mul<Rhs> for &'a mut SMatrix<T, N, M>
where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: Rhs,
    ) -> Self::Output {
        (&*self).mul(rhs)
    }
}

//}}}
//}}}
