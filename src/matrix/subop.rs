
//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::smatrix::*;
use super::binop::{SubOp, BinopExpr};
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::Sub;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


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
impl<'a, A, B, T, const N: usize, const M: usize> Sub<&'a SMatrix<T, N, M>> for BinopExpr<A, B, T, SubOp>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
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
impl<'a, A, B, T, const N: usize, const M: usize> Sub<BinopExpr<A, B, T, SubOp>> for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, SubOp>, T, SubOp>;

    fn sub(self, rhs: BinopExpr<A, B, T, SubOp>) -> BinopExpr<Self, BinopExpr<A, B, T, SubOp>, T, SubOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Sub for BinopExpr
impl<A, B, C, D, T> Sub<BinopExpr<A, B, T, SubOp>> for BinopExpr<C, D, T, SubOp>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    C: IndexValue<usize, Output = T>,
    D: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<BinopExpr<C, D, T, SubOp>, BinopExpr<A, B,T, SubOp>, T, SubOp>;

    fn sub(self, rhs: BinopExpr<A, B, T, SubOp>) ->  BinopExpr<BinopExpr<C, D, T, SubOp>, BinopExpr<A, B,T, SubOp>, T, SubOp> {
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

    use topohedral_tracing::*;

    #[test]
    fn test_matrix_sub() {
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
  
}
//}}}