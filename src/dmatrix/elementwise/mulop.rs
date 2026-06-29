//! Element-wise multiplication operators for [`DMatrix`]: matrix * scalar and matrix * matrix.
//!
//! Implements the [`Mul`] trait for element-wise (Hadamard) multiplication of [`DMatrix<T>`]
//! operands and for scalar–matrix scaling. This is *not* matrix multiplication; for that see
//! `matmul`. Both matrix–matrix and scalar–matrix products are lazy, returning a
//! `BinopExpr` that is evaluated on demand when converted into a concrete [`DMatrix`].
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, MatrixExpr, ScalarExpr};
use crate::dmatrix::DMatrix;
use crate::expression::binary_expr::{BinOp, BinopExpr, MulOp};
use crate::expression::outer_product_expr::OuterProductExpr;
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
//}}}
//{{{ std imports
use std::ops::{Mul, MulAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fun: mul_assign_expr
#[inline]
fn mul_assign_expr<T, Rhs>(
    lhs: &mut DMatrix<T>,
    rhs: Rhs,
) where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    let rhs_nrows = rhs.nrows();
    let rhs_ncols = rhs.ncols();
    if lhs.nrows != rhs_nrows || lhs.ncols != rhs_ncols {
        panic!(
            "DMatrix::mul_assign dimension mismatch: lhs is {}x{}, rhs is {}x{}",
            lhs.nrows, lhs.ncols, rhs_nrows, rhs_ncols
        );
    }

    for i in 0..lhs.data.len() {
        unsafe {
            *lhs.data.get_unchecked_mut(i) *= rhs.linear_value(i);
        }
    }
}
//}}}

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
    ) -> Self::Output {
        let mut out = self.clone();
        out.iter_mut().for_each(|x| *x *= rhs);
        out
    }
}
//}}}
//{{{ impl: Mul<DMatrix> for DMatrix
impl<T> Mul for DMatrix<T>
where
    T: Field + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn mul(
        self,
        rhs: DMatrix<T>,
    ) -> Self::Output {
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
        impl Mul<DMatrix<$type>> for $type {
            type Output = DMatrix<$type>;

            #[inline]
            fn mul(
                self,
                rhs: DMatrix<$type>,
            ) -> Self::Output {
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
    ) {
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
//{{{ impl: MulAssign<BinopExpr> for DMatrix
impl<A, B, T, Op> MulAssign<BinopExpr<A, B, T, Op>> for DMatrix<T>
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
//{{{ impl: MulAssign<UnaryExpr> for DMatrix
impl<A, T, Op> MulAssign<UnaryExpr<A, T, Op>> for DMatrix<T>
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
//{{{ impl: MulAssign<OuterProductExpr> for DMatrix
impl<L, R, T> MulAssign<OuterProductExpr<L, R, T>> for DMatrix<T>
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
//{{{ collection: MulOp for DMatrix
//{{{ impl: Mul<T> for DMatrix
macro_rules! impl_dmatrix_mul_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Mul<$type> for &'a DMatrix<$type> {
            type Output = BinopExpr<&'a DMatrix<$type>, ScalarExpr<$type>, $type, MulOp>;

            #[inline]
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
        impl<'a> Mul<$type> for &'a mut DMatrix<$type> {
            type Output = BinopExpr<&'a DMatrix<$type>, ScalarExpr<$type>, $type, MulOp>;

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

apply_for_all_types!(impl_dmatrix_mul_scalar_rhs);

//}}}
//{{{ impl: Mul<DMatrix> for $type
macro_rules! impl_dmatrix_mul {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Mul<&'a DMatrix<$type>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a DMatrix<$type>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: &'a DMatrix<$type>,
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
apply_for_all_types!(impl_dmatrix_mul);
//}}}
//{{{ impl: Mul<&mut DMatrix> for $type
macro_rules! impl_dmatrix_ref_mut_mul {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Mul<&'a mut DMatrix<$type>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a DMatrix<$type>, $type, MulOp>;

            #[inline]
            fn mul(
                self,
                rhs: &'a mut DMatrix<$type>,
            ) -> Self::Output {
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
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, MulOp>;

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
//{{{ impl: Mul<Rhs> for &'a mut DMatrix
impl<'a, T, Rhs> Mul<Rhs> for &'a mut DMatrix<T>
where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, MulOp>;

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
