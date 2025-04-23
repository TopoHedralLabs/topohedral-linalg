//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------


//{{{ crate imports
use crate::common::{Field, IndexValue, Zero};
use crate::expression::binary_expr::{BinopExpr, BinOp};
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
pub use matmul::MatMul;
pub mod matrix_ops;
pub mod qr;
pub mod schur;
pub mod solve;
// everything else 
pub mod construction;
pub mod indexing;
pub mod iteration;

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
//{{{ collection: Evaluation to DMatrix
//{{{ trait: Evaluate
pub trait EvaluateDMatrix<T>
where
    
    T: Field + Copy,
{
    fn evald(&self) -> DMatrix<T>;
}

//}}}
//{{{ impl: Evaluate for DMatrix
impl<T> EvaluateDMatrix<T> for DMatrix<T>
where
    
    T: Field + Copy,
{
    fn evald(&self) -> DMatrix<T>
    {
        self.clone()
    }
}

//}}}
//{{{ impl: EvaluateDMatrix for BinopExpr
#[doc(hidden)]
impl<A, B, T, Op> EvaluateDMatrix<T> for BinopExpr<A, B, T, Op>
where
    A: IndexValue<usize, Output = T>,
    B: IndexValue<usize, Output = T>,
    T: Field + Copy + Zero,
    Op: BinOp,
{
    fn evald(&self) -> DMatrix<T>
    {
        let mut out = DMatrix::<T>::zeros(self.nrows, self.ncols);
        for i in 0..self.nrows * self.ncols
        {
            out.data[i] = self.index_value(i);
        }
        out
    }
}
//}}}
//}}}