//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::super::smatrix::*;
use super::common::{AddOp, BinopExpr, BinOp};
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::Add;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: Add for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Add for &'a SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, AddOp>;

    fn add(self, rhs: Self) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, AddOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<&' SMatrix> for BinopExpr
impl<'a, A, B, T, Op, const N: usize, const M: usize> Add<&'a SMatrix<T, N, M>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,

{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, AddOp>;

    fn add(self, rhs: &'a SMatrix<T, N, M>) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, AddOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Add<BinopExpr> for &'a SMatrix
impl<'a, A, B, T, Op, const N: usize, const M: usize> Add<BinopExpr<A, B, T, Op>> for &'a SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>;

    fn add(self, rhs: BinopExpr<A, B, T, Op>) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, AddOp>
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

    use topohedral_tracing::*;

    #[test]
    fn test_matrix_add() {
        let matrix1 = SMatrix::<i32, 2, 2>::from_value(1);
        let matrix2 = SMatrix::<i32, 2, 2>::from_value(10);
        let matrix3 = SMatrix::<i32, 2, 2>::from_value(100);
        let matrix4 = SMatrix::<i32, 2, 2>::from_value(1000);
        let matrix5 = SMatrix::<i32, 2, 2>::from_value(10000);
        let matrix6 = SMatrix::<i32, 2, 2>::from_value(100000);
        let mut matrix7 = SMatrix::<i32, 2, 2>::default();
        matrix7 = ((&matrix4 + &matrix5) + (&matrix1 + &matrix2 + &matrix3) + &matrix6).eval();


        let exp_value: i32 = (1000 + 10000) + (1 + 10 + 100) + 100000;
        for val in &matrix7 {
            assert_eq!(*val, exp_value);
        }
    }
  
}
//}}}