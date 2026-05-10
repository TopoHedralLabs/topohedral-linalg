//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{
    EvalInto, Field, IndexValue, LazyExpr, Zero,
};
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

//{{{ impl: LazyExpr for DMatrix
impl<T> LazyExpr for DMatrix<T>
where
    T: Field + Copy,
{
    type ScalarType = T;
}

//}}}
//{{{ impl: From<BinopExpr> for DMatrix
impl<A, B, T, Op> From<BinopExpr<A, B, T, Op>> for DMatrix<T>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + Zero,
    Op: BinOp,
{
    fn from(expr: BinopExpr<A, B, T, Op>) -> DMatrix<T>
    {
        let nrows = expr.nrows;
        let ncols = expr.ncols;
        let total = nrows * ncols;
        // Allocate uninitialised storage, then drive evaluation through
        // EvalInto::eval_into.  That method writes through `&mut [T]` which
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
    A: IndexValue<usize, Output = T> + crate::common::Shape,
    T: Field + Copy + Zero,
    Op: UnaryOp<T>,
{
    fn from(expr: UnaryExpr<A, T, Op>) -> DMatrix<T>
    {
        let nrows = expr.nrows;
        let ncols = expr.ncols;
        let total = nrows * ncols;
        let mut data: Vec<T> = Vec::with_capacity(total);
        let out_ptr: *mut T = data.as_mut_ptr();
        for i in 0..total
        {
            unsafe { out_ptr.add(i).write(expr.index_value(i)) };
        }
        unsafe { data.set_len(total) };
        DMatrix { data, nrows, ncols }
    }
}
//}}}