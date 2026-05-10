//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{Field, IndexValue, LazyExpr, Zero};
use crate::expression::binary_expr::{BinOp, BinopExpr};
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

mod addop;
mod divop;
mod mulop;
mod negop;
mod subop;

//{{{ impl: LazyExpr for SMatrix
impl<T, const N: usize, const M: usize> LazyExpr for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type ScalarType = T;
}

//}}}
//{{{ impl: From<BinopExpr> for SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> From<BinopExpr<A, B, T, Op>> for SMatrix<T, N, M>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + Zero,
    Op: BinOp,
{
    fn from(expr: BinopExpr<A, B, T, Op>) -> Self
    {
        let mut out = SMatrix::<T, N, M>::zeros();

        for i in 0..N * M
        {
            out[i] = expr.index_value(i);
        }

        out
    }
}
//}}}
//{{{ impl: From<UnaryExpr> for SMatrix
impl<A, T, Op, const N: usize, const M: usize> From<UnaryExpr<A, T, Op>> for SMatrix<T, N, M>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T> + crate::common::Shape,
    T: Field + Copy + Zero,
    Op: UnaryOp<T>,
{
    fn from(expr: UnaryExpr<A, T, Op>) -> Self
    {
        let mut out = SMatrix::<T, N, M>::zeros();

        for i in 0..N * M
        {
            out[i] = expr.index_value(i);
        }

        out
    }
}
//}}}
