//! Addition operators for [`SMatrix`]: matrix + matrix and matrix + scalar.
//!
//! Implements the standard [`Add`] trait for all combinations of owned and borrowed
//! [`SMatrix<T, N, M>`] operands, as well as mixed matrix–scalar and scalar–matrix overloads.
//! Matrix–matrix addition returns a lazy `BinopExpr` that defers allocation until materialised;
//! scalar addition is applied element-wise. Const-generic dimension parameters ensure that only
//! shape-compatible matrices can be added at compile time.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
#[cfg(feature = "enable_checks")]
use crate::common::Shape;
use crate::common::{Field, MatrixExpr, ScalarExpr};
use crate::expression::binary_expr::{AddOp, BinOp, BinopExpr};
use crate::expression::outer_product_expr::OuterProductExpr;
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
use crate::smatrix::SMatrix;
//}}}
//{{{ std imports
use std::ops::{Add, AddAssign};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fun: add_assign_expr
#[inline]
fn add_assign_expr<T, Rhs, const N: usize, const M: usize>(
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
            "SMatrix::add_assign dimension mismatch: lhs is {}x{}, rhs is {}x{}",
            lhs.nrows, lhs.ncols, rhs_nrows, rhs_ncols
        );
    }

    let out = lhs.as_mut_slice();
    for i in 0..out.len() {
        unsafe {
            *out.get_unchecked_mut(i) += rhs.linear_value(i);
        }
    }
}
//}}}

//{{{ collection: eagerly evaluated expressions
//{{{ impl: Add<T> for SMatrix
impl<T, const N: usize, const M: usize> Add<T> for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn add(
        self,
        rhs: T,
    ) -> Self::Output {
        let mut out = self;
        out.iter_mut().for_each(|x| *x += rhs);
        out
    }
}
//}}}
//{{{ impl: Add<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> Add for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    type Output = SMatrix<T, N, M>;

    #[inline]
    fn add(
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
        impl<const N: usize, const M: usize> Add<SMatrix<$type, N, M>> for $type {
            type Output = SMatrix<$type, N, M>;

            #[inline]
            fn add(
                self,
                rhs: SMatrix<$type, N, M>,
            ) -> Self::Output {
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
    T: Field + Copy,
{
    #[inline]
    fn add_assign(
        &mut self,
        rhs: T,
    ) {
        self.iter_mut().for_each(|x| *x += rhs);
    }
}
//}}}
//{{{ impl: AddAssign<SMatrix> for SMatrix
impl<T, const N: usize, const M: usize> AddAssign for SMatrix<T, N, M>
where
    T: Field + Copy,
{
    #[inline]
    fn add_assign(
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
                *out_elem += *rhs_elem;
            });
    }
}
//}}}
//{{{ impl: AddAssign<BinopExpr> for SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> AddAssign<BinopExpr<A, B, T, Op>>
    for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: BinOp,
{
    #[inline]
    fn add_assign(
        &mut self,
        rhs: BinopExpr<A, B, T, Op>,
    ) {
        add_assign_expr(self, rhs);
    }
}
//}}}
//{{{ impl: AddAssign<UnaryExpr> for SMatrix
impl<A, T, Op, const N: usize, const M: usize> AddAssign<UnaryExpr<A, T, Op>> for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
    Op: UnaryOp<T>,
{
    #[inline]
    fn add_assign(
        &mut self,
        rhs: UnaryExpr<A, T, Op>,
    ) {
        add_assign_expr(self, rhs);
    }
}
//}}}
//{{{ impl: AddAssign<OuterProductExpr> for SMatrix
impl<L, R, T, const N: usize, const M: usize> AddAssign<OuterProductExpr<L, R, T>>
    for SMatrix<T, N, M>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy,
{
    #[inline]
    fn add_assign(
        &mut self,
        rhs: OuterProductExpr<L, R, T>,
    ) {
        add_assign_expr(self, rhs);
    }
}
//}}}
//}}}
//{{{ collection: AddOp for SMatrix
//{{{ impl: Add<T> for SMatrix
macro_rules! impl_smatrix_add_scalar_rhs {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Add<$type> for &'a SMatrix<$type, N, M> {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, ScalarExpr<$type>, $type, AddOp>;

            #[inline]
            fn add(
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
        impl<'a, const N: usize, const M: usize> Add<$type> for &'a mut SMatrix<$type, N, M> {
            type Output = BinopExpr<&'a SMatrix<$type, N, M>, ScalarExpr<$type>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: $type,
            ) -> Self::Output {
                (&*self).add(rhs)
            }
        }
    };
}

apply_for_all_types!(impl_smatrix_add_scalar_rhs);

//}}}
//{{{ impl: Add<Smatrix> for $type
macro_rules! impl_smatrix_add {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Add<&'a SMatrix<$type, N, M>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a SMatrix<$type, N, M>, $type, AddOp>;

            #[inline]
            fn add(
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

apply_for_all_types!(impl_smatrix_add);
//}}}
//{{{ impl: Add<&mut Smatrix> for $type
macro_rules! impl_smatrix_add_mut {
    ($type:ty) => {
        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Add<&'a mut SMatrix<$type, N, M>> for $type {
            type Output = BinopExpr<ScalarExpr<$type>, &'a SMatrix<$type, N, M>, $type, AddOp>;

            #[inline]
            fn add(
                self,
                rhs: &'a mut SMatrix<$type, N, M>,
            ) -> Self::Output {
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
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, AddOp>;

    #[inline]
    fn add(
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
//{{{ impl: Add<Rhs> for &'a mut SMatrix
impl<'a, T, Rhs, const N: usize, const M: usize> Add<Rhs> for &'a mut SMatrix<T, N, M>
where
    T: Field + Copy,
    Rhs: MatrixExpr<ScalarType = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, Rhs, T, AddOp>;

    #[inline]
    fn add(
        self,
        rhs: Rhs,
    ) -> Self::Output {
        (&*self).add(rhs)
    }
}

//}}}
//}}}
