//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Dimension, EvalInto, Field, IndexValue, LazyExpr, One, Zero};
use crate::expression::binary_expr::{BinOp, BinopExpr};
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
//}}}
//{{{ std imports
use serde::{Deserialize, Serialize};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

// binary operations
pub mod addop;
pub mod divop;
pub mod mulop;
pub mod subop;
// unary operations
pub mod negop;
// matrix operations
pub mod eig;
pub mod lu;
pub mod matmul;
pub mod matrix_ops;
pub mod qr;
pub mod reduce_ops;
pub mod schur;
pub mod solve;
pub mod symeig;
pub mod transform_ops;
// everything else
pub mod construction;
pub mod indexing;
pub mod io;
pub mod iteration;
pub mod subviews;

//{{{ struct: DMatrix
/// A dynamic-size $N \times M$ matrix type that stores its elements in a dynamic, contiguous array.
///
/// The `DMatrix` struct represents a 2D matrix with a dynamic size, where the dimensions
/// are specified as generic parameters `N` and `M`. The elements of the matrix are
/// stored in a contiguous array, which allows for efficient access and manipulation.
///
/// The matrix is stored in column-major order, which means a matrix is stored column by column
/// in memory. So, for example, the matrix:
/// ```ignore
/// 1 2 3
/// 4 5 6
/// 7 8 9
/// ```
/// will be stored in memory as:
/// ```ignore
/// 1 4 7 2 5 9 3 6 9
/// ```
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DMatrix<T>
where
    T: Field + Copy,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: Vec<T>,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
impl<T> Clone for DMatrix<T>
where
    T: Field + Copy,
{
    fn clone(&self) -> Self
    {
        Self {
            data: self.data.clone(),
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }

    fn clone_from(
        &mut self,
        source: &Self,
    )
    {
        self.data.clone_from(&source.data);
        self.nrows = source.nrows;
        self.ncols = source.ncols;
    }
}

//{{{ impl: DMatrix
impl<T> DMatrix<T>
where
    T: Field + Copy + Ord + Zero + One,
{
    //{{{ fn: sort
    pub fn sort(
        &mut self,
        dim: Dimension,
    )
    {
        match dim
        {
            Dimension::Rows =>
            {
                for r in 0..self.nrows
                {
                    let mut row = Vec::with_capacity(self.ncols);
                    for c in 0..self.ncols
                    {
                        row.push(self[(r, c)]);
                    }
                    row.sort();

                    for (c, value) in row.into_iter().enumerate()
                    {
                        (*self)[(r, c)] = value;
                    }
                }
            }
            Dimension::Cols =>
            {
                for c in 0..self.ncols
                {
                    let offset = c * self.nrows;
                    self.data[offset..(offset + self.nrows)].sort();
                }
            }
            Dimension::All =>
            {
                self.data.sort();
            }
        }
    }
    //}}}
    //{{{ fn: sorted
    pub fn sorted(
        &self,
        dim: Dimension,
    ) -> Self
    {
        let mut out = self.clone();
        out.sort(dim);
        out
    }
    //}}}
    //{{{ fn: into_sorted
    pub fn into_sorted(
        mut self,
        dim: Dimension,
    ) -> Self
    {
        self.sort(dim);
        self
    }
    //}}}
}
//}}}
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
        #[allow(clippy::uninit_vec)]
        let mut data: Vec<T> = Vec::with_capacity(total);
        unsafe { data.set_len(total) };
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
} //}}}
