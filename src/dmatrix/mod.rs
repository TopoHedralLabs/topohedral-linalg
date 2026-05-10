//! Dynamic matrix type with heap-allocated, column-major storage.
//!
//! Defines [`DMatrix<T>`], a general-purpose 2-D matrix whose dimensions are determined at
//! runtime and whose elements are stored in a contiguous `Vec<T>` in column-major (Fortran) order.
//! Sub-modules add element-wise arithmetic ([`addop`], [`subop`], [`mulop`], [`divop`], [`negop`]),
//! BLAS-backed matrix multiplication ([`matmul`]), standard linear-algebra decompositions
//! ([`lu`], [`qr`], [`eig`], [`symeig`], [`schur`], [`solve`]), and supporting utilities for
//! construction, indexing, iteration, I/O, sub-matrix views, and reduction/transformation
//! operations.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{
    Dimension, EvalInto, Field, Float, FloatVectorOps, IndexValue, LazyExpr, One, VectorOps, Zero,
};
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
//{{{ impl: Clone for DMatrix
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
//}}}
//{{{ impl: DMatrix
impl<T> DMatrix<T>
where
    T: Field + Copy + Ord + Zero + One,
{
    //{{{ fn: sort
    /// Sorts the elements of the matrix in-place along the specified dimension.
    ///
    /// When `dim` is `Dimension::Rows`, each row is sorted independently.
    /// When `dim` is `Dimension::Cols`, each column is sorted independently.
    /// When `dim` is `Dimension::All`, all elements are sorted as a single sequence.
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
    /// Returns a new matrix with elements sorted along the specified dimension, leaving `self` unchanged.
    ///
    /// See [`sort`](DMatrix::sort) for the semantics of `dim`.
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
    /// Consumes `self`, sorts its elements along the specified dimension, and returns the result.
    ///
    /// Prefer this over [`sorted`](DMatrix::sorted) when the original matrix is no longer needed,
    /// as it avoids an extra allocation.
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
} //}}}
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
