//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, MatrixOps, Zero, One};
use crate::subviews::MatrixView;
use super::DMatrix;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


impl<'a, T> DMatrix<T>
where
    T: Field + Copy + Zero + One,
{
    //{{{ fun: subview
    /// Creates a subview of the matrix.
    pub fn subview(
        &'a self,
        start_row: usize,
        start_col: usize,
        nrows: usize,
        ncols: usize,
    ) -> MatrixView<'a, DMatrix<T>> {
        MatrixView {
            matrix: self,
            start_row,
            start_col,
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: row
    pub fn row(
        &'a self,
        row: usize,
    ) -> MatrixView<'a, DMatrix<T>> {
        self.subview(row, 0, 1, self.ncols)
    }
    //}}}
    //{{{ fun: rows
    pub fn rows(
        &'a self,
        start_row: usize,
        nrows: usize,
    ) -> MatrixView<'a, DMatrix<T>> {
        self.subview(start_row, 0, nrows, self.ncols)
    }
    //}}}
    //{{{ fun: col
    pub fn col(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, DMatrix<T>> {
        self.subview(0, col, self.nrows, 1)
    }
    //}}}
    //{{{ fun: cols
    pub fn cols(
        &'a self,
        start_col: usize,
        ncols: usize,
    ) -> MatrixView<'a, DMatrix<T>> {
        self.subview(0, start_col, self.nrows, ncols)
    }
    //}}}
}
