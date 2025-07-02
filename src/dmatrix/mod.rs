//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, IndexValue, Zero};
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
pub mod schur;
pub mod solve;
pub mod symeig;
// everything else
pub mod construction;
pub mod indexing;
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
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
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
}//}}}
