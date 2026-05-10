//! Sub-matrix view methods for [`SMatrix`].
//!
//! The view types themselves live in [`crate::subviews`]. This module provides
//! [`SubViewable`] and [`SubViewableMut`] implementations for [`SMatrix`], plus
//! `to_dmatrix()` on the view types and the `copy_from` / `set_*` helpers.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{Field, One, Shape, Zero};
use crate::dmatrix::DMatrix;
use crate::subviews::{MatrixView, MatrixViewMut, SubViewable, SubViewableMut};
//}}}
//{{{ std imports
use std::ops::Index;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: to_dmatrix for SMatrix views
impl<'a, T, const N: usize, const M: usize> MatrixView<'a, SMatrix<T, N, M>>
where
    [(); N * M]:,
    T: Field + Copy,
{
    /// Copies the view contents into a new heap-allocated [`DMatrix`].
    pub fn to_dmatrix(&self) -> DMatrix<T>
    {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for j in 0..self.ncols
        {
            for i in 0..self.nrows
            {
                data.push(self[(i, j)]);
            }
        }
        DMatrix {
            data,
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }
}

impl<'a, T, const N: usize, const M: usize> MatrixViewMut<'a, SMatrix<T, N, M>>
where
    [(); N * M]:,
    T: Field + Copy,
{
    /// Copies the view contents into a new heap-allocated [`DMatrix`].
    pub fn to_dmatrix(&self) -> DMatrix<T>
    {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for j in 0..self.ncols
        {
            for i in 0..self.nrows
            {
                data.push(self[(i, j)]);
            }
        }
        DMatrix {
            data,
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }
}
//}}}

//{{{ impl: SubViewable for SMatrix
impl<T, const N: usize, const M: usize> SubViewable for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + Zero + One,
{
    fn subview<'a>(
        &'a self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, SMatrix<T, N, M>>
    {
        MatrixView {
            matrix: self,
            start_row,
            start_col,
            nrows: end_row - start_row + 1,
            ncols: end_col - start_col + 1,
        }
    }
}
//}}}
//{{{ impl: SubViewableMut for SMatrix
impl<T, const N: usize, const M: usize> SubViewableMut for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy + Zero + One,
{
    fn subview_mut<'a>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, SMatrix<T, N, M>>
    {
        MatrixViewMut {
            matrix: self,
            start_row,
            start_col,
            nrows: end_row - start_row + 1,
            ncols: end_col - start_col + 1,
        }
    }
}
//}}}

//{{{ impl: SMatrix copy and set helpers
impl<'a, T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Field + Copy + Zero + One,
    [(); N * M]:,
{
    //{{{ fun: copy_from
    /// Copies entries from `rhs` into this matrix.
    ///
    /// # Panics
    ///
    /// Panics when `rhs` dimensions do not match this matrix's dimensions.
    pub fn copy_from<Rhs>(
        &mut self,
        rhs: Rhs,
    ) where
        Rhs: Shape + Index<(usize, usize), Output = T>,
    {
        let rhs_nrows = rhs.nrows();
        let rhs_ncols = rhs.ncols();
        if self.nrows != rhs_nrows || self.ncols != rhs_ncols
        {
            panic!(
                "SMatrix::copy_from dimension mismatch: lhs is {}x{}, rhs is {}x{}",
                self.nrows, self.ncols, rhs_nrows, rhs_ncols
            );
        }
        for i in 0..self.nrows
        {
            for j in 0..self.ncols
            {
                (*self)[(i, j)] = rhs[(i, j)];
            }
        }
    }
    //}}}
    //{{{ fun: set_row
    /// Copies `rhs` into the row view at `row`.
    pub fn set_row<Rhs>(
        &'a mut self,
        row: usize,
        rhs: Rhs,
    ) where
        Rhs: Shape + Index<(usize, usize), Output = T>,
    {
        let mut row_view = self.row_mut(row);
        row_view.copy_from(rhs);
    }
    //}}}
    //{{{ fun: set_col
    /// Copies `rhs` into the column view at `col`.
    pub fn set_col<Rhs>(
        &'a mut self,
        col: usize,
        rhs: Rhs,
    ) where
        Rhs: Shape + Index<(usize, usize), Output = T>,
    {
        let mut col_view = self.col_mut(col);
        col_view.copy_from(rhs);
    }
    //}}}
    //{{{ fun: set_subview
    /// Copies `rhs` into the subview described by `[start_row..=end_row, start_col..=end_col]`.
    pub fn set_subview<Rhs>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
        rhs: Rhs,
    ) where
        Rhs: Shape + Index<(usize, usize), Output = T>,
    {
        let mut subview = self.subview_mut(start_row, end_row, start_col, end_col);
        subview.copy_from(rhs);
    }
    //}}}
}
//}}}
