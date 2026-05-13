//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------
//{{{ crate imports
use crate::common::{Field, Float, FloatVectorOps, One, VectorOps, Zero};
//}}}
//{{{ std imports
use serde::{Deserialize, Serialize};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: DMatrix
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
    /// Number of rows in the matrix.
    pub(crate) nrows: usize,
    /// Number of columns in the matrix.
    pub(crate) ncols: usize,
}
//}}}
//}}}
//{{{ collection: DVector
//{{{ type: DVector
/// A dynamic vector stored as a single-row or single-column [`DMatrix`].
pub type DVector<T> = DMatrix<T>;
//}}}
//{{{ enum: VecType
/// Selects whether a `DVector` is oriented as a row vector or a column vector.
pub enum VecType
{
    /// A 1×N row vector.
    Row,
    /// An N×1 column vector.
    Col,
}
//}}}
//{{{ impl: VectorOps for DVector<T>
impl<T> VectorOps for DVector<T>
where
    T: Field + Default + Copy + Clone + Zero + One + Float,
{
    type ScalarType = T;

    fn len(&self) -> usize
    {
        if self.nrows != 1 && self.ncols != 1
        {
            panic!("Vector must be either a row or column vector");
        }

        if self.nrows == 1
        {
            self.ncols
        }
        else
        {
            self.nrows
        }
    }
}
//}}}
//{{{ impl: FloatVectorOps for DVector<T>
impl<T> FloatVectorOps for DVector<T> where T: Float + Default + Copy + Clone + Zero + One {}
//}}}
//}}}
