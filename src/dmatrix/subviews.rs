//! Sub-matrix view methods for [`DMatrix`].
//!
//! The view types themselves live in [`crate::subviews`]. This module provides the constructor
//! methods (`subview`, `row`, `col`, etc.) on `DMatrix`, plus `to_dmatrix()` on the
//! immutable and mutable view types when parameterised over `DMatrix`.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{Field, One, Shape, Zero};
use crate::subviews::{MatrixView, MatrixViewMut};
//}}}
//{{{ std imports
use std::ops::Index;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: to_dmatrix for DMatrix views
impl<'a, T> MatrixView<'a, DMatrix<T>>
where
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
        DMatrix { data, nrows: self.nrows, ncols: self.ncols }
    }
}

impl<'a, T> MatrixViewMut<'a, DMatrix<T>>
where
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
        DMatrix { data, nrows: self.nrows, ncols: self.ncols }
    }
}
//}}}

//{{{ impl: DMatrix subview methods
impl<'a, T> DMatrix<T>
where
    T: Field + Copy + Zero + One,
{
    //{{{ fun: subview
    /// Creates a subview of the matrix.
    pub fn subview(
        &'a self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, DMatrix<T>>
    {
        MatrixView {
            matrix: self,
            start_row,
            start_col,
            nrows: end_row - start_row + 1,
            ncols: end_col - start_col + 1,
        }
    }
    //}}}
    //{{{ fun: row
    /// Returns an immutable view of a single row.
    pub fn row(
        &'a self,
        row: usize,
    ) -> MatrixView<'a, DMatrix<T>>
    {
        self.subview(row, row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: rows
    /// Returns an immutable view spanning rows `start_row` through `end_row` (inclusive).
    pub fn rows(
        &'a self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixView<'a, DMatrix<T>>
    {
        self.subview(start_row, end_row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: col
    /// Returns an immutable view of a single column.
    pub fn col(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, DMatrix<T>>
    {
        self.subview(0, self.nrows - 1, col, col)
    }
    //}}}
    //{{{ fun: cols
    /// Returns an immutable view spanning columns `start_col` through `end_col` (inclusive).
    pub fn cols(
        &'a self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, DMatrix<T>>
    {
        self.subview(0, self.nrows - 1, start_col, end_col)
    }
    //}}}
    //{{{ fun: subview_mut
    /// Creates a mutable subview of the matrix.
    pub fn subview_mut(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, DMatrix<T>>
    {
        MatrixViewMut {
            matrix: self,
            start_row,
            start_col,
            nrows: end_row - start_row + 1,
            ncols: end_col - start_col + 1,
        }
    }
    //}}}
    //{{{ fun: row_mut
    /// Returns a mutable view of a single row.
    pub fn row_mut(
        &'a mut self,
        row: usize,
    ) -> MatrixViewMut<'a, DMatrix<T>>
    {
        self.subview_mut(row, row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: rows_mut
    /// Returns a mutable view spanning rows `start_row` through `end_row` (inclusive).
    pub fn rows_mut(
        &'a mut self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixViewMut<'a, DMatrix<T>>
    {
        self.subview_mut(start_row, end_row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: col_mut
    /// Returns a mutable view of a single column.
    pub fn col_mut(
        &'a mut self,
        col: usize,
    ) -> MatrixViewMut<'a, DMatrix<T>>
    {
        self.subview_mut(0, self.nrows - 1, col, col)
    }
    //}}}
    //{{{ fun: cols_mut
    /// Returns a mutable view spanning columns `start_col` through `end_col` (inclusive).
    pub fn cols_mut(
        &'a mut self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, DMatrix<T>>
    {
        self.subview_mut(0, self.nrows - 1, start_col, end_col)
    }
    //}}}
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
                "DMatrix::copy_from dimension mismatch: lhs is {}x{}, rhs is {}x{}",
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
