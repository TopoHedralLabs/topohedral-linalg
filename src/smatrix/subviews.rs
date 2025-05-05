//! This module provides subviews of a matrix.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::dmatrix::DMatrix;
use super::SMatrix;
use crate::common::{tuple_index, Field, One, Zero};
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
pub struct MatrixView<'a, T, const N: usize, const M: usize>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub(crate) matrix: &'a SMatrix<T, N, M>,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
// Indexing for immutable view
//{{{ impl: Index for MatrixView
impl<'a, T, const N: usize, const M: usize> Index<(usize, usize)> for MatrixView<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
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
impl<'a, T, const N: usize, const M: usize> Index<usize> for MatrixView<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
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
pub struct MatrixViewIter<'a, T, const N: usize, const M: usize>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub(crate) matrix_view: &'a MatrixView<'a, T, N, M>,
    index: usize,
}
//}}}
// Immutable iterator over immutable view
//{{{ impl: Iterator for MatrixViewIter
impl<'a, T, const N: usize, const M: usize> Iterator for MatrixViewIter<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols
        {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&self.matrix_view[(row, col)])
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
impl<'a, T, const N: usize, const M: usize> MatrixView<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub fn iter(&'a self) -> MatrixViewIter<'a, T, N, M>
    {
        MatrixViewIter {
            matrix_view: self,
            index: 0,
        }
    }

    pub fn to_dmatrix(&self) -> DMatrix<T>
    {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for i in 0..self.nrows
        {
            for j in 0..self.ncols
            {
                data.push(self[(j, i)]);
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

// definition of the mutable view
//{{{ struct: MatrixViewMut
pub struct MatrixViewMut<'a, T, const N: usize, const M: usize>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub(crate) matrix: &'a mut SMatrix<T, N, M>,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
// Indexing for mutable view
//{{{ impl: Index for MatrixViewMut
impl<'a, T, const N: usize, const M: usize> Index<(usize, usize)> for MatrixViewMut<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
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
impl<'a, T, const N: usize, const M: usize> Index<usize> for MatrixViewMut<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
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
impl<'a, T, const N: usize, const M: usize> IndexMut<(usize, usize)> for MatrixViewMut<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let (row_loc, col_loc) = index;
        &mut self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}

impl<'a, T, const N: usize, const M: usize> IndexMut<usize> for MatrixViewMut<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output
    {
        let (row_loc, col_loc) = tuple_index(index, self.nrows);
        &mut self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
//}}}
// Immutable iterator over mutable view
//{{{ struct: MatrixViewMutIter
pub struct MatrixViewMutIter<'a, T, const N: usize, const M: usize>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub(crate) matrix_view: &'a MatrixViewMut<'a, T, N, M>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIter
impl<'a, T, const N: usize, const M: usize> Iterator for MatrixViewMutIter<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols
        {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&self.matrix_view[(row, col)])
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
pub struct MatrixViewMutIterMut<'a, T, const N: usize, const M: usize>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub(crate) matrix_view: &'a mut MatrixViewMut<'a, T, N, M>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIterMut
impl<'a, T, const N: usize, const M: usize> Iterator for MatrixViewMutIterMut<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
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
                let ptr = &mut self.matrix_view[(row, col)] as *mut T;
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
impl<'a, T, const N: usize, const M: usize> MatrixViewMut<'a, T, N, M>
where
    T: Field + Copy,
    [(); N * M]:,
{
    pub fn iter(&'a self) -> MatrixViewMutIter<'a, T, N, M>
    {
        MatrixViewMutIter {
            matrix_view: self,
            index: 0,
        }
    }

    pub fn iter_mut(&'a mut self) -> MatrixViewMutIterMut<'a, T, N, M>
    {
        MatrixViewMutIterMut {
            matrix_view: self,
            index: 0,
        }
    }

    pub fn to_dmatrix(&self) -> DMatrix<T>
    {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for i in 0..self.nrows
        {
            for j in 0..self.ncols
            {
                data.push(self[(j, i)]);
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

// Accessing of subviews the DMatrix struct
//{{{ impl: DMatrix
impl<'a, T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Field + Copy + Zero + One,
    [(); N * M]:,
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
    ) -> MatrixView<'a, T, N, M>
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
    ) -> MatrixView<'a, T, N, M>
    {
        self.subview(row, row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: rows
    pub fn rows(
        &'a self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixView<'a, T, N, M>
    {
        self.subview(start_row, end_row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: col
    pub fn col(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, T, N, M>
    {
        self.subview(0, self.nrows - 1, col, col)
    }
    //}}}
    //{{{ fun: cols
    pub fn cols(
        &'a self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, T, N, M>
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
    ) -> MatrixViewMut<'a, T, N, M>
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
    ) -> MatrixViewMut<'a, T, N, M>
    {
        self.subview_mut(row, row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: rows_mut
    pub fn rows_mut(
        &'a mut self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixViewMut<'a, T, N, M>
    {
        self.subview_mut(start_row, end_row, 0, self.ncols - 1)
    }
    //}}}
    //{{{ fun: col_mut
    pub fn col_mut(
        &'a mut self,
        col: usize,
    ) -> MatrixViewMut<'a, T, N, M>
    {
        self.subview_mut(0, self.nrows - 1, col, col)
    }
    //}}}
    //{{{ fun: cols_mut
    pub fn cols_mut(
        &'a mut self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, T, N, M>
    {
        self.subview_mut(0, self.nrows - 1, start_col, end_col)
    }
    //}}}
}
//}}}
