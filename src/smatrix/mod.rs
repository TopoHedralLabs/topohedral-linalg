//! Static matrix type with compile-time dimensions and stack-allocated storage.
//!
//! Defines [`SMatrix<T, N, M>`], a 2-D matrix whose row count N and column count M are
//! const-generic parameters baked into the type. Elements are stored in a fixed-size array
//! `[T; N*M]` in column-major order, enabling `Copy` semantics and stack allocation for small
//! matrices. Sub-modules add element-wise arithmetic, BLAS-backed matrix multiplication, standard
//! decompositions (LU, QR, eigenvalue, Schur, linear solve), and supporting utilities for
//! construction, indexing, iteration, serialisation, sub-views, and reduction/transformation
//! operations.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{
    Dimension, Field, Float, FloatVectorOps, GreaterThan, IndexValue, LazyExpr, One, VectorOps,
    Zero,
};
use crate::expression::binary_expr::{BinOp, BinopExpr};
use crate::expression::unary_expr::{UnaryExpr, UnaryOp};
//}}}
//{{{ std imports
use std::convert::From;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

// elementwise expressions
mod elementwise;
mod blaslapack;
mod matrix_ops;
mod reduce_ops;
mod transform_ops;
mod construction;
mod indexing;
mod io;
mod iteration;
mod subviews;

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
    T: Field + Copy,
{
    /// The data of the matrix, stored in column-major order.
    pub(crate) data: [T; N * M],
    /// Number of rows (always equal to `N`).
    pub(crate) nrows: usize,
    /// Number of columns (always equal to `M`).
    pub(crate) ncols: usize,
}
//}}}
//{{{ impl SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + Ord,
{
    //{{{ fn: sort
    /// Sorts the elements of the matrix in place along the given dimension.
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
    /// Returns a copy of the matrix with elements sorted along the given dimension.
    pub fn sorted(
        &self,
        dim: Dimension,
    ) -> Self
    {
        let mut out = *self;
        out.sort(dim);
        out
    }
    //}}}
    //{{{ fn: into_sorted
    /// Consumes the matrix and returns it with elements sorted along the given dimension.
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
//{{{ impl: LazyExpr for SMatrix
impl<T, const N: usize, const M: usize> LazyExpr for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type ScalarType = T;
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
  //{{{ impl: From<UnaryExpr> for SMatrix
impl<A, T, Op, const N: usize, const M: usize> From<UnaryExpr<A, T, Op>> for SMatrix<T, N, M>
where
    [(); N * M]:,
    A: IndexValue<usize, Output = T> + crate::common::Shape,
    T: Field + Copy + Zero,
    Op: UnaryOp<T>,
{
    fn from(expr: UnaryExpr<A, T, Op>) -> Self
    {
        let mut out = SMatrix::<T, N, M>::zeros();

        for i in 0..N * M
        {
            out[i] = expr.index_value(i);
        }

        out
    }
} //}}}
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
