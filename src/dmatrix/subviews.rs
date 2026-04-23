//! This module provides subviews of a matrix.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{tuple_index, Field, One, Shape, Zero};
//}}}
//{{{ std imports
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

// definition of the immutable view
//{{{ struct: MatrixView
/// Immutable subview of a matrix designed to provide access to a submatrix without copying the
/// data. I behaves like a normal Dmatrix but does not own its data. Therefore, the matrix to which
/// it refers must outlive the view. Currently supports:
/// - Indexing by (row, col) tuple and by single index
/// - Iteration over the elements of the view, iteration is performed in column-major order
pub struct MatrixView<'a, T>
where
    T: Field + Copy,
{
    pub(crate) matrix: &'a DMatrix<T>,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
// Indexing for immutable view
//{{{ impl: Index for MatrixView
impl<'a, T> Index<(usize, usize)> for MatrixView<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let (row_loc, col_loc) = index;
        &self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
impl<'a, T> Index<usize> for MatrixView<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output
    {
        let (row_loc, col_loc) = tuple_index(index, self.nrows);
        &self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
//}}}
// Immutable iterator over immutable view
//{{{ struct: MatrixViewIter
pub struct MatrixViewIter<'a, T>
where
    T: Field + Copy,
{
    pub(crate) matrix_view: &'a MatrixView<'a, T>,
    index: usize,
}
//}}}
// Immutable iterator over immutable view
//{{{ impl: Iterator for MatrixViewIter
impl<'a, T> Iterator for MatrixViewIter<'a, T>
where
    T: Field + Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols
        {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&(*self.matrix_view)[(row, col)])
        }
        else
        {
            None
        }
    }
}
//}}}
// Implementation of the MatrixView struct
//{{{ impl: MatrixView
impl<'a, T> MatrixView<'a, T>
where
    T: Field + Copy,
{
    pub fn iter(&'a self) -> MatrixViewIter<'a, T>
    {
        MatrixViewIter {
            matrix_view: self,
            index: 0,
        }
    }

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
//{{{ impl: Shape/Index wrappers for views
impl<'a, T> Shape for MatrixView<'a, T>
where
    T: Field + Copy,
{
    fn nrows(&self) -> usize
    {
        self.nrows
    }

    fn ncols(&self) -> usize
    {
        self.ncols
    }
}

impl<'a, T> Shape for MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    fn nrows(&self) -> usize
    {
        self.nrows
    }

    fn ncols(&self) -> usize
    {
        self.ncols
    }
}

impl<'a, T> Index<(usize, usize)> for &MatrixView<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        &(**self)[index]
    }
}

impl<'a, T> Index<(usize, usize)> for &mut MatrixView<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        &(**self)[index]
    }
}

impl<'a, T> Index<(usize, usize)> for &MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        &(**self)[index]
    }
}

impl<'a, T> Index<(usize, usize)> for &mut MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        &(**self)[index]
    }
}
//}}}

// definition of the mutable view
//{{{ struct: MatrixViewMut
pub struct MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    pub(crate) matrix: &'a mut DMatrix<T>,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
// Indexing for mutable view
//{{{ impl: Index for MatrixViewMut
impl<'a, T> Index<(usize, usize)> for MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let (row_loc, col_loc) = index;
        &self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
impl<'a, T> Index<usize> for MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output
    {
        let (row_loc, col_loc) = tuple_index(index, self.nrows);
        &self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
//}}}
//{{{ impl IndexMut for MatrixViewMut
impl<'a, T> IndexMut<(usize, usize)> for MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let (row_loc, col_loc) = index;
        &mut (*self.matrix)[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}

impl<'a, T> IndexMut<usize> for MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output
    {
        let (row_loc, col_loc) = tuple_index(index, self.nrows);
        &mut (*self.matrix)[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
//}}}
// Immutable iterator over mutable view
//{{{ struct: MatrixViewMutIter
pub struct MatrixViewMutIter<'a, T>
where
    T: Field + Copy,
{
    pub(crate) matrix_view: &'a MatrixViewMut<'a, T>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIter
impl<'a, T> Iterator for MatrixViewMutIter<'a, T>
where
    T: Field + Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols
        {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&(*self.matrix_view)[(row, col)])
        }
        else
        {
            None
        }
    }
}
//}}}
// Mutable iterator over mutable view
//{{{ struct: MatrixViewMutIterMut
pub struct MatrixViewMutIterMut<'a, T>
where
    T: Field + Copy,
{
    pub(crate) matrix_view: &'a mut MatrixViewMut<'a, T>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIterMut
impl<'a, T> Iterator for MatrixViewMutIterMut<'a, T>
where
    T: Field + Copy,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols
        {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            unsafe {
                // Convert to a raw pointer and then back to a reference with lifetime 'a
                let ptr = &mut (*self.matrix_view)[(row, col)] as *mut T;
                Some(&mut *ptr)
            }
        }
        else
        {
            None
        }
    }
}
//}}}
// Implementation of the MatrixViewMut struct
//{{{ impl: MatrixViewMut
impl<'a, T> MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    pub fn iter(&'a self) -> MatrixViewMutIter<'a, T>
    {
        MatrixViewMutIter {
            matrix_view: self,
            index: 0,
        }
    }

    pub fn iter_mut(&'a mut self) -> MatrixViewMutIterMut<'a, T>
    {
        MatrixViewMutIterMut {
            matrix_view: self,
            index: 0,
        }
    }

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

    /// Copies entries from `rhs` into this mutable view.
    ///
    /// # Panics
    ///
    /// Panics when `rhs` dimensions do not match this view's dimensions.
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
                "MatrixViewMut::copy_from dimension mismatch: lhs is {}x{}, rhs is {}x{}",
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
}
//}}}

// Accessing of subviews the DMatrix struct
//{{{ impl: DMatrix
impl<'a, T> DMatrix<T>
where
    T: Field + Copy + Zero + One,
{
    // Immutable subview of the matrix
    //{{{ fun: subview
    /// Creates a subview of the matrix.
    pub fn subview(
        &'a self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, T>
    {
        let nrows = end_row - start_row + 1;
        let ncols = end_col - start_col + 1;
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
    ) -> MatrixView<'a, T>
    {
        self.subview(row, row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: rows
    pub fn rows(
        &'a self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixView<'a, T>
    {
        self.subview(start_row, end_row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: col
    pub fn col(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, T>
    {
        self.subview(0, self.nrows - 1, col, col)
    }
    //}}}
    //{{{ fun: cols
    pub fn cols(
        &'a self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, T>
    {
        self.subview(0, self.nrows - 1, start_col, end_col)
    }
    //}}}

    // Mutable subview of the matrix
    //{{{ fun: subview_mut
    /// Creates a mutable subview of the matrix.
    pub fn subview_mut(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, T>
    {
        let nrows = end_row - start_row + 1;
        let ncols = end_col - start_col + 1;
        MatrixViewMut {
            matrix: self,
            start_row,
            start_col,
            nrows,
            ncols,
        }
    }
    //}}}
    //{{{ fun: row_mut
    pub fn row_mut(
        &'a mut self,
        row: usize,
    ) -> MatrixViewMut<'a, T>
    {
        self.subview_mut(row, row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: rows_mut
    pub fn rows_mut(
        &'a mut self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixViewMut<'a, T>
    {
        self.subview_mut(start_row, end_row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: col_mut
    pub fn col_mut(
        &'a mut self,
        col: usize,
    ) -> MatrixViewMut<'a, T>
    {
        self.subview_mut(0, self.nrows - 1, col, col)
    }
    //}}}
    //{{{ fun: cols_mut
    pub fn cols_mut(
        &'a mut self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, T>
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
    ///
    /// This is the assignment-style alternative to the unsupported syntax `m.row(row) = rhs`.
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
    ///
    /// Rust does not allow assignment to method-call temporaries, so use this or
    /// `subview_mut(...).copy_from(...)` instead of `m.subview(...) = rhs`.
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
