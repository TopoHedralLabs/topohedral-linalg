//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{Field, MatrixExpr, Zero};
use crate::expression::binary_expr::{BinOp, BinopExpr};
use crate::expression::outer_product_expr::OuterProductExpr;
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

//{{{ impl: From<BinopExpr> for SMatrix
impl<A, B, T, Op, const N: usize, const M: usize> From<BinopExpr<A, B, T, Op>> for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy + Zero,
    Op: BinOp,
{
    fn from(expr: BinopExpr<A, B, T, Op>) -> Self {
        let mut out = SMatrix::<T, N, M>::zeros();
        expr.eval_into(out.as_mut_slice());
        out
    }
}
//}}}
//{{{ impl: From<UnaryExpr> for SMatrix
impl<A, T, Op, const N: usize, const M: usize> From<UnaryExpr<A, T, Op>> for SMatrix<T, N, M>
where
    A: MatrixExpr<ScalarType = T>,
    T: Field + Copy + Zero,
    Op: UnaryOp<T>,
{
    fn from(expr: UnaryExpr<A, T, Op>) -> Self {
        let mut out = SMatrix::<T, N, M>::zeros();
        expr.eval_into(out.as_mut_slice());
        out
    }
}
//}}}
//{{{ impl: From<OuterProductExpr> for SMatrix
impl<L, R, T, const N: usize, const M: usize> From<OuterProductExpr<L, R, T>> for SMatrix<T, N, M>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy + Zero,
{
    fn from(expr: OuterProductExpr<L, R, T>) -> Self {
        let mut out = SMatrix::<T, N, M>::zeros();
        expr.eval_into(out.as_mut_slice());
        out
    }
}
//}}}
