//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use super::super::smatrix::*;
use super::common::{BinOp, BinopExpr, DivOp};
//}}}
//{{{ std imports 
use std::fmt;
use std::ops::Div;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ impl: Div for &'a SMatrix
impl<'a, T, const N: usize, const M: usize> Div for &'a SMatrix<T, N, M>
where 
    [(); N * M]:,
    T: Field + Default + Copy + fmt::Display + Clone,
{
    type Output = BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, DivOp>;

    fn div(self, rhs: Self) -> BinopExpr<&'a SMatrix<T, N, M>, &'a SMatrix<T, N, M>, T, DivOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Div<&' SMatrix> for BinopExpr
impl<'a, A, B, Op, T, const N: usize, const M: usize> Div<&'a SMatrix<T, N, M>> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, &'a SMatrix<T, N, M>, T, DivOp>;

    fn div(self, rhs: &'a SMatrix<T, N, M>) -> BinopExpr<Self, &'a SMatrix<T, N, M>, T, DivOp>
    {
        BinopExpr {
            a: self,
            b: rhs,
            _marker: std::marker::PhantomData,
        }
    }
}
//}}}
//{{{ impl: Div<BinopExpr> for &'a SMatrix
impl<'a, A, B, T, Op, const N: usize, const M: usize> Div<BinopExpr<A, B, T, Op>> for &'a SMatrix<T, N, M>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Default + Copy + fmt::Display + Clone,
    Op: BinOp,
    [(); N * M]:,
{
    type Output = BinopExpr<Self, BinopExpr<A, B, T, Op>, T, DivOp>;

    fn div(self, rhs: BinopExpr<A, B, T, Op>) -> BinopExpr<Self, BinopExpr<A, B, T, Op>, T, DivOp>
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
    fn test_matrix_div() {

        let matrix1 = SMatrix::<f64, 2, 2>::from_value(1.0);
        let matrix2 = SMatrix::<f64, 2, 2>::from_value(10.0);
        let matrix3 = SMatrix::<f64, 2, 2>::from_value(100.0);
        let matrix4 = SMatrix::<f64, 2, 2>::from_value(1000.0);
        let matrix5 = SMatrix::<f64, 2, 2>::from_value(10000.0);
        let matrix6 = SMatrix::<f64, 2, 2>::from_value(100000.0);
        let mut matrix7 = SMatrix::<f64, 2, 2>::default();
        matrix7 = ((&matrix4 / &matrix5) / (&matrix1 / &matrix2 / &matrix3) / &matrix6).eval();

        let exp_value: f64 = (1000.0 / 10000.0) / (1.0 / 10.0 / 100.0) / 100000.0;

        for val in &matrix7 {
            assert_eq!(*val, exp_value);
        }
    }
  
}
//}}}