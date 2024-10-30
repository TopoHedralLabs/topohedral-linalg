//! Short Description of module
//!
//! Longer description of module

//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::super::dmatrix::*;
use super::super::smatrix::*;
use super::common::{BinOp, BinopExpr, MulOp};
use crate::apply_for_all_types;
use crate::common::*;

//}}}
//{{{ std imports
use std::fmt;
use std::ops::Mul;

//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: MulOp for SMatrix
//{{{ impl: Mul<T> for SMatrix
#[doc(hidden)]

impl<'a, T, const N: usize, const M: usize> Mul<T> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, MulOp>;

    fn mul(
        self,
        rhs: T,
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

//}}}
//{{{ impl: Mul<Smatrix> for $type
macro_rules! impl_smatrix_mul {
    ($type:ty) => {
        #[doc(hidden)]

        impl<'a, const N: usize, const M: usize> Mul<&'a SMatrix<$type, N, M>> for $type
        where
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, MulOp>;

            fn mul(
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

apply_for_all_types!(impl_smatrix_mul);

//}}}
//{{{ impl: Mul for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Mul for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, MulOp>;

    fn mul(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, MulOp>
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

//}}}
//{{{ impl: Mul<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Mul<&'a SMatrix<T, N, M>>
    for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, MulOp>;

    fn mul(
        self,
        rhs: &'a SMatrix<T, N, M>,
    ) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, MulOp>
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

//}}}
//{{{ impl: Mul<BinopExpr> for &'a SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> Mul<BinopExpr<A, B, T, Op>> for &SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>;

    fn mul(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>
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

//}}}
//}}}
//{{{ collection: MulOp for DMatrix
//{{{ impl: Mul<T> for DMatrix
#[doc(hidden)]
impl<'a, T> Mul<T> for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, T, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: T,
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
//{{{ impl: Mul for &'a DMatrix
impl<'a, T> Mul for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, MulOp>
    {
        let nr = self.nrows;
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

//}}}
//{{{ impl: Mul<&' DMatrix> for BinopExpr
impl<'a, A, B, T, Op> Mul<&'a DMatrix<T>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = BinopExpr<Self, &'a DMatrix<T>, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: &'a DMatrix<T>,
    ) -> BinopExpr<Self, &'a DMatrix<T>, T, MulOp>
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

//}}}
//{{{ impl: Mul<BinopExpr> for &'a DMatrix
impl<A, B, T, Op> Mul<BinopExpr<A, B, T, Op>> for &DMatrix<T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>;

    #[inline]
    fn mul(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>
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
//}}}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]

mod tests
{

    use super::*;

    #[test]

    fn test_mul_smatrix()
    {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);

        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);

        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);

        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);

        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);

        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);

        let mut matrix7 = SMatrix::<f64, 2, 2>::default();

        matrix7 = ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).evals();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }


    #[test]
    fn test_mul_dmatrix()
    {
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 1.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix3 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let matrix4 = DMatrix::<f64>::from_value(2, 2, 1000.0);
        let matrix5 = DMatrix::<f64>::from_value(2, 2, 10000.0);
        let matrix6 = DMatrix::<f64>::from_value(2, 2, 100000.0);
        let mut matrix7 = DMatrix::<f64>::zeros(2, 2);

        matrix7 = ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).evald();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar_smatrix()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();

        matrix4 = (4i32 * (2i32 * &matrix1) * (&matrix2 * 3i32) * 5i32).evals();

        let exp_val = 4 * (2 * 10) * (100 * 3) * 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }


    #[test]
    fn test_mul_scalar_dmatrix()
    {
        let matrix1 = DMatrix::<f64>::from_value(2, 2, 10.0);
        let matrix2 = DMatrix::<f64>::from_value(2, 2, 100.0);
        let mut matrix4 = DMatrix::<f64>::zeros(2, 2);

        matrix4 = (4.0 * (2.0 * &matrix1) * (&matrix2 * 3.0) * 5.0).evald();

        let exp_val = 4.0 * (2.0 * 10.0) * (100.0 * 3.0) * 5.0;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
}

//}}}
