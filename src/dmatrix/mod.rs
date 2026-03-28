//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Dimension, Field, IndexValue, One, Zero};
use crate::expression::binary_expr::{BinOp, BinopExpr};
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
        let mut out = DMatrix::<T>::zeros(expr.nrows, expr.ncols);
        for i in 0..expr.nrows * expr.ncols
        {
            out.data[i] = expr.index_value(i);
        }
        out
    }
} //}}}
