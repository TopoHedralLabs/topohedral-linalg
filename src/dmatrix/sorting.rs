//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::Dimension;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: DMatrix
impl<T> DMatrix<T>
where
    T: Copy + Ord,
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
