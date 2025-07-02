//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, IndexValue, Zero};
use crate::expression::binary_expr::{BinOp, BinopExpr};
//}}}
//{{{ std imports
use std::convert::From;
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
pub mod io;
pub mod iteration;
pub mod subviews;

//{{{ struct: SMatrix
/// A fixed-size $N \times M$ matrix type that stores its elements in a static, contiguous array.
///
/// The `SMatrix` struct represents a 2D matrix with a fixed size, where the dimensions
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
#[derive(Copy, Clone, Debug)]
pub struct SMatrix<T, const N: usize, const M: usize>
where
    [(); N * M]:,
    T: Field + Copy,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: [T; N * M],
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
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
} //}}}
