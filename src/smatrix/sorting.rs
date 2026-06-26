//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::Dimension;

use super::SMatrix;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Copy + Ord,
{
    //{{{ fn: sort
    /// Sorts the elements of the matrix in place along the given dimension.
    pub fn sort(
        &mut self,
        dim: Dimension,
    ) {
        match dim {
            Dimension::Rows => {
                for r in 0..self.nrows {
                    let mut row = Vec::with_capacity(self.ncols);
                    for c in 0..self.ncols {
                        row.push(self[(r, c)]);
                    }
                    row.sort();

                    for (c, value) in row.into_iter().enumerate() {
                        (*self)[(r, c)] = value;
                    }
                }
            }
            Dimension::Cols => {
                for c in 0..self.ncols {
                    self.data[c].sort();
                }
            }
            Dimension::All => {
                self.as_mut_slice().sort();
            }
        }
    }
    //}}}
    //{{{ fn: sorted
    /// Returns a copy of the matrix with elements sorted along the given dimension.
    pub fn sorted(
        &self,
        dim: Dimension,
    ) -> Self {
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
    ) -> Self {
        self.sort(dim);
        self
    }
    //}}}
}
//}}}
