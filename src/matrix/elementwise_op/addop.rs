//! This module implements the element-wise addition operation for the `SMatrix` type.
//!
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::super::dmatrix::*;
use super::super::smatrix::*;
use super::common::{AddOp, BinOp, BinopExpr};
use crate::apply_for_all_types;
use crate::common::*;

//}}}
//{{{ std imports
use std::fmt;
use std::ops::Add;

//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: AddOp for SMatrix
//{{{ impl: Add<T> for SMatrix
#[doc(hidden)]
impl<'a, T, const N: usize, const M: usize> Add<T> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, AddOp>;

    #[inline]

    fn add(
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

//}}}
//{{{ impl: Add for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Add for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, AddOp>
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
//{{{ impl: Add<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Add<&'a SMatrix<T, N, M>>
    for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: &'a SMatrix<T, N, M>,
    ) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, AddOp>
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
//{{{ impl: Add<BinopExpr> for &'a SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> Add<BinopExpr<A, B, T, Op>> for &SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>
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
//{{{ collection: AddOp for DMatrix
//{{{ impl: Add<T> for DMatrix
#[doc(hidden)]
impl<'a, T> Add<T> for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a DMatrix<T>, T, T, AddOp>;

    #[inline]

    fn add(
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
//{{{ impl: Add<DMatrix> for $type
macro_rules! impl_smatrix_add {
    ($type:ty) => {
        #[doc(hidden)]

        impl<'a> Add<&'a DMatrix<$type>> for $type
        {
            type Output = BinopExpr<$type, &'a DMatrix<$type>, $type, AddOp>;

            #[inline]

            fn add(
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

apply_for_all_types!(impl_smatrix_add);

//}}}
//{{{ impl: Add for &'a DMatrix
impl<'a, T> Add for &'a DMatrix<T>
where
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: Self,
    ) -> BinopExpr<&'a DMatrix<T>, &'a DMatrix<T>, T, AddOp>
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
//{{{ impl: Add<&' DMatrix> for BinopExpr
impl<'a, A, B, T, Op> Add<&'a DMatrix<T>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = BinopExpr<Self, &'a DMatrix<T>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: &'a DMatrix<T>,
    ) -> BinopExpr<Self, &'a DMatrix<T>, T, AddOp>
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
//{{{ impl: Add<BinopExpr> for &'a DMatrix
impl<A, B, T, Op> Add<BinopExpr<A, B, T, Op>> for &DMatrix<T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>;

    #[inline]

    fn add(
        self,
        rhs: BinopExpr<A, B, T, Op>,
    ) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>
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
    fn test_add_smatrix()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);

        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);

        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);

        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);

        let matrix7 = SMatrix::<i32, 2, 2>::from_value(1000000);

        let mut matrix8 = SMatrix::<i32, 2, 2>::default();

        matrix8 = (&matrix7 + (&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6)
            .evals();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_add_dmatrix()
    {
        let matrix1 = DMatrix::<i32>::from_value(2, 2, 1);

        let matrix2 = DMatrix::<i32>::from_value(2, 2, 10);

        let matrix3 = DMatrix::<i32>::from_value(2, 2, 100);

        let matrix4 = DMatrix::<i32>::from_value(2, 2, 1000);

        let matrix5 = DMatrix::<i32>::from_value(2, 2, 10000);

        let matrix6 = DMatrix::<i32>::from_value(2, 2, 100000);

        let matrix7 = DMatrix::<i32>::from_value(2, 2, 1000000);

        let mut matrix8 = DMatrix::<i32>::zeros(2, 2);

        matrix8 = (&matrix7 + (&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6)
            .evald();

        let exp_value: i32 = 1000000 + (1000 + 10000) + (1 + 10 + 100) + 100000;

        for val in &matrix8
        {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_add_scalar_smatrix()
    {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);

        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();

        matrix4 = (4i32 + (2i32 + &matrix1) + (&matrix2 + 3i32) + 5i32).evals();

        let exp_val = 4 + (2 + 10) + (100 + 3) + 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }

    #[test]
    fn test_add_scalar_dmatrix()
    {
        let matrix1 = DMatrix::<i32>::from_value(2,2, 10);

        let matrix2 = DMatrix::<i32>::from_value(2,2, 100);

        let mut matrix4 = DMatrix::<i32>::zeros(2,2);   

        matrix4 = (4i32 + (2i32 + &matrix1) + (&matrix2 + 3i32) + 5i32).evald();

        let exp_val = 4 + (2 + 10) + (100 + 3) + 5;

        for val in &matrix4
        {
            assert_eq!(*val, exp_val);
        }
    }
}

//}}}
