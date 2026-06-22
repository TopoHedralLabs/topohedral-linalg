//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{Field, GreaterThan, One, VectorOps, Zero};
use crate::float::{Float, FloatVectorOps};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: SMatrix
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
    T: Copy,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: [T; N * M],
    /// Number of rows (always equal to `N`).
    pub(crate) nrows: usize,
    /// Number of columns (always equal to `M`).
    pub(crate) ncols: usize,
}
//}}}
//}}}
//{{{ collection: SRVector
//{{{ type: SRVector
/// A type alias for a row vector of size N.
pub type SRVector<T, const N: usize> = SMatrix<T, 1, N>;
//}}}
//{{{ impl: VectorOps for SRVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> VectorOps for SRVector<T, N>
where
    [(); 1usize * N]:,
    T: Field + Default + Copy + Clone + Zero + One + Float,
    (): GreaterThan<N, 1>,
{
    type ScalarType = T;

    fn len(&self) -> usize
    {
        N
    }
}
//}}}
//{{{ impl: FloatVectorOps for SRVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> FloatVectorOps for SRVector<T, N>
where
    [(); 1usize * N]:,
    T: Float + Default + Copy + Clone + Zero + One,
    (): GreaterThan<N, 1>,
{
}
//}}}
//}}}
//{{{ collection: SCVector
//{{{ type: SCVector
/// A type alias for a column vector of size N.
pub type SCVector<T, const N: usize> = SMatrix<T, N, 1>;
//}}}
//{{{ impl: VectorOps for SCVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> VectorOps for SCVector<T, N>
where
    [(); N * 1]:,
    T: Field + Default + Copy + Clone + Zero + One + Float,
    (): GreaterThan<N, 1>,
{
    type ScalarType = T;

    fn len(&self) -> usize
    {
        N
    }
}
//}}}
//{{{ impl: FloatVectorOps for SCVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> FloatVectorOps for SCVector<T, N>
where
    [(); N * 1]:,
    T: Float + Default + Copy + Clone + Zero + One,
    (): GreaterThan<N, 1>,
{
}
//}}}
//}}}
