//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
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

//{{{ impl: From<BinopExpr> for DMatrix
impl<A, B, T, Op> From<BinopExpr<A, B, T, Op>> for DMatrix<T>
where
    A: MatrixExpr<ScalarType = T>,
    B: MatrixExpr<ScalarType = T>,
    T: Field + Copy + Zero,
    Op: BinOp,
{
    fn from(expr: BinopExpr<A, B, T, Op>) -> DMatrix<T> {
        let nrows = expr.nrows;
        let ncols = expr.ncols;
        let total = nrows * ncols;
        // Allocate uninitialised storage, then drive evaluation through
        // MatrixExpr::eval_into. That method writes through `&mut [T]` which
        // LLVM marks `noalias`, letting it prove the output slice doesn't overlap
        // the input DMatrix struct fields and enabling SIMD auto-vectorisation.
        let mut data: Vec<T> = Vec::with_capacity(total);
        #[allow(clippy::uninit_vec)]
        unsafe {
            data.set_len(total)
        };
        expr.eval_into(&mut data);
        DMatrix { data, nrows, ncols }
    }
} //}}}
  //{{{ impl: From<UnaryExpr> for DMatrix
impl<A, T, Op> From<UnaryExpr<A, T, Op>> for DMatrix<T>
where
    A: MatrixExpr<ScalarType = T>,
    T: Field + Copy + Zero,
    Op: UnaryOp<T>,
{
    fn from(expr: UnaryExpr<A, T, Op>) -> DMatrix<T> {
        let nrows = expr.nrows;
        let ncols = expr.ncols;
        let total = nrows * ncols;
        let mut data: Vec<T> = Vec::with_capacity(total);
        #[allow(clippy::uninit_vec)]
        unsafe {
            data.set_len(total)
        };
        expr.eval_into(&mut data);
        DMatrix { data, nrows, ncols }
    }
}
//}}}
//{{{ impl: From<OuterProductExpr> for DMatrix
impl<L, R, T> From<OuterProductExpr<L, R, T>> for DMatrix<T>
where
    L: MatrixExpr<ScalarType = T>,
    R: MatrixExpr<ScalarType = T>,
    T: Field + Copy + Zero,
{
    fn from(expr: OuterProductExpr<L, R, T>) -> DMatrix<T> {
        let nrows = expr.nrows;
        let ncols = expr.ncols;
        let total = nrows * ncols;
        let mut data: Vec<T> = Vec::with_capacity(total);
        #[allow(clippy::uninit_vec)]
        unsafe {
            data.set_len(total)
        };
        expr.eval_into(&mut data);
        DMatrix { data, nrows, ncols }
    }
}
//}}}
