
//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::super::smatrix::*;
use super::common::{BinOp, BinopExpr, SubOp};
use crate::apply_for_all_types;
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::Sub;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Sub<T> for SMatrix
#[doc(hidden)]
impl<'a, T, const N: usize, const M: usize> Sub<T> for &'a SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, SubOp>;

    fn sub(self, rhs: T) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Sub<Smatrix> for $type
macro_rules! impl_smatrix_sub{
    ($type:ty) => {

        #[doc(hidden)]
        impl<'a, const N: usize, const M: usize> Sub<&'a SMatrix<$type, N, M>> for $type
        where 
            [(); N * M]:,
        {
            type Output = BinopExpr<$type, &'a SMatrix<$type, N, M>, $type, SubOp>;

            fn sub(self, rhs: &'a SMatrix<$type, N, M>) -> Self::Output {
                BinopExpr {
                    a: self,
                    b: rhs,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}
apply_for_all_types!(impl_smatrix_sub); 
//}}}

//{{{ impl: Sub for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Sub for &'a SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, SubOp>;

    fn sub(self, rhs: Self) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, SubOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Sub<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Sub<&'a SMatrix<T, N, M>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, SubOp>;

    fn sub(self, rhs: &'a SMatrix<T, N, M>) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, SubOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Sub<BinopExpr> for &'a SMatrix
impl<'a, A, B, T, Op, const N: usize, const M: usize> Sub<BinopExpr<A, B, T, Op>> for &'a SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, SubOp>;

    fn sub(self, rhs: BinopExpr<A, B, T, Op>) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, SubOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{

    use super::*;   

    

    #[test]
    fn test_sub_matrix() {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);
        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);
        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);
        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);
        let mut matrix7 = SMatrix::<i32, 2, 2>::default();
        matrix7 = ((&matrix4 - &matrix5) - (&matrix1 - &matrix2 - &matrix3) - &matrix6).eval();

        let exp_value: i32 = (1000 - 10000) - (1 - 10 - 100) - 100000;

        for val in &matrix7 {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_sub_scalar() {

        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();
        matrix4 = (4i32 - (2i32 - &matrix1) - (&matrix2 - 3i32) - 5i32).eval();

        let exp_val = 4 - (2 - 10) - (100 - 3) - 5;

        for val in &matrix4 {
            assert_eq!(*val, exp_val);
        }

    }
  
}
//}}}