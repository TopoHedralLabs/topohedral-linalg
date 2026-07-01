//! Subtraction operators for [`DMatrix`]: matrix − matrix and matrix − scalar.
//!
//! Mirrors `addop` for subtraction. Implements [`Sub`] for all owned/borrowed combinations
//! of [`DMatrix<T>`] and for mixed matrix–scalar operands. Matrix–matrix subtraction is lazy,
//! returning a `BinopExpr` that defers allocation until the expression is materialised into
//! a concrete [`DMatrix`]; scalar subtraction is applied element-wise.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, MatrixExpr, ScalarExpr};
use crate::dmatrix::DMatrix;
use crate::expression::binary_expr::{BinOp, BinopExpr, SubOp};
use crate::expression::outer_product_expr::OuterProductExpr;
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
//}}}
//{{{ std imports
use std::ops::{Sub, SubAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fun: sub_assign_expr
#[inline]
fn sub_assign_expr<T, Rhs>(
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
            "DMatrix::sub_assign dimension mismatch: lhs is {}x{}, rhs is {}x{}",
            lhs.nrows, lhs.ncols, rhs_nrows, rhs_ncols
        );
    }

    for i in 0..lhs.data.len() {
        unsafe {
            *lhs.data.get_unchecked_mut(i) -= rhs.linear_value(i);
        }
    }
}
//}}}

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
    ) -> Self::Output {
        let mut out = self.clone();
        out.iter_mut().for_each(|x| *x -= rhs);
        out
    }
}
//}}}
//{{{ impl: Sub<DMatrix> for DMatrix
impl<T> Sub for DMatrix<T>
where
    T: Field + Copy,
{
    type Output = DMatrix<T>;

    #[inline]
    fn sub(
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
        impl Sub<DMatrix<$type>> for $type {
            type Output = DMatrix<$type>;

            #[inline]
            fn sub(
                self,
                rhs: DMatrix<$type>,
            ) -> Self::Output {
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
    ) {
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
                *out_elem = *out_elem - *rhs_elem;
            });
    }
}
//}}}
//{{{ impl: SubAssign<BinopExpr> for DMatrix
impl<A, B, T, Op> SubAssign<BinopExpr<A, B, T, Op>> for DMatrix<T>
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
//{{{ impl: SubAssign<UnaryExpr> for DMatrix
impl<A, T, Op> SubAssign<UnaryExpr<A, T, Op>> for DMatrix<T>
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
//{{{ impl: SubAssign<OuterProductExpr> for DMatrix
impl<L, R, T> SubAssign<OuterProductExpr<L, R, T>> for DMatrix<T>
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
//{{{ collection: SubOp for DMatrix
//{{{ impl: Sub<T> for DMatrix
macro_rules! impl_dmatrix_sub_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Sub<$type> for &'a DMatrix<$type> {
            type Output = BinopExpr<&'a DMatrix<$type>, ScalarExpr<$type>, $type, SubOp>;

            #[inline]
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
        impl<'a> Sub<$type> for &'a mut DMatrix<$type> {
            type Output = BinopExpr<&'a DMatrix<$type>, ScalarExpr<$type>, $type, SubOp>;

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

apply_for_all_types!(impl_dmatrix_sub_scalar_rhs);

//}}}
//{{{ impl: Sub<DMatrix> for $type
macro_rules! impl_dmatrix_sub {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Sub<&'a DMatrix<$type>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a DMatrix<$type>, $type, SubOp>;

            #[inline]
            fn sub(
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
apply_for_all_types!(impl_dmatrix_sub);
//}}}
//{{{ impl: Sub<&mut DMatrix> for $type
macro_rules! impl_dmatrix_ref_mut_sub {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a> Sub<&'a mut DMatrix<$type>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a DMatrix<$type>, $type, SubOp>;

            #[inline]
            fn sub(
                self,
                rhs: &'a mut DMatrix<$type>,
            ) -> Self::Output {
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
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, SubOp>;

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
//{{{ impl: Sub<Rhs> for &'a mut DMatrix
impl<'a, T, Rhs> Sub<Rhs> for &'a mut DMatrix<T>
where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, Rhs, T, SubOp>;

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
