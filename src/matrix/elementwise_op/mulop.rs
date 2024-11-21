

//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::super::smatrix::*;
use super::common::{BinOp, BinopExpr, MulOp};
use crate::apply_for_all_types;
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::Mul;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ impl: Mul<T> for SMatrix
#[doc(hidden)]
impl<'a, T, const N: usize, const M: usize> Mul<T> for &'a SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone + IndexValue<usize, Output = T>,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, T, T, MulOp>;

    fn mul(self, rhs: T) -> Self::Output {
        BinopExpr {
            a: self,
            b: rhs,
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

            fn mul(self, rhs: &'a SMatrix<$type, N, M>) -> Self::Output {
                BinopExpr {
                    a: self,
                    b: rhs,
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

    fn mul(self, rhs: Self) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, MulOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Mul<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Mul<&'a SMatrix<T, N, M>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, MulOp>;

    fn mul(self, rhs: &'a SMatrix<T, N, M>) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, MulOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Mul<BinopExpr> for &'a SMatrix
impl<'a, A, B, T, Op, const N: usize, const M: usize> Mul<BinopExpr<A, B, T, Op>> for &'a SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>;

    fn mul(self, rhs: BinopExpr<A, B, T, Op>) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, MulOp>
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
    fn test_mul_matrix() {
        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);
        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);
        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);
        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);
        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);
        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);
        let mut matrix7 = SMatrix::<f64, 2, 2>::default();
        matrix7 = ((&matrix4 * &matrix5) * (&matrix1 * &matrix2 * &matrix3) * &matrix6).eval();

        let exp_value: f64 = (1000.0 * 10000.0) * (1.0 * 10.0 * 100.0) * 100000.0;

        for val in &matrix7 {
            assert_eq!(*val, exp_value);
        }
    }

    #[test]
    fn test_mul_scalar() {

        let matrix1 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(100);

        let mut matrix4 = SMatrix::<i32, 2, 2>::default();
        matrix4 = (4i32 * (2i32 * &matrix1) * (&matrix2 * 3i32) * 5i32).eval();

        let exp_val = 4 * (2 * 10) * (100 * 3) * 5;

        for val in &matrix4 {
            assert_eq!(*val, exp_val);
        }


    }
  
}
//}}}