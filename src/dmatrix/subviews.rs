//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, Zero, One, tuple_index};
use super::DMatrix;
//}}}
//{{{ std imports 
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ struct: MatrixView
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
//{{{ impl: Index for MatrixView
impl<'a, T> Index<(usize, usize)> for MatrixView<'a, T>
where 
    T: Field + Copy,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row_loc, col_loc) = index;
        &self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
//}}}
//{{{ struct: MatrixViewIter
pub struct MatrixViewIter<'a, T>
where 
    T: Field + Copy,
{
    pub(crate) matrix_view: &'a MatrixView<'a, T>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewIter
impl<'a, T> Iterator for MatrixViewIter<'a, T> 
where 
    T: Field + Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&self.matrix_view[(row, col)])
        } else {
            None
        }
    }
}
//}}}
//{{{ impl: MatrixView
impl<'a, T> MatrixView<'a, T>
where 
    T: Field + Copy,
{
    pub fn iter(&'a self) -> MatrixViewIter<'a, T> {
        MatrixViewIter {
            matrix_view: self,
            index: 0,
        }
    }

    pub fn to_dmatrix(&self) -> DMatrix<T> {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for i in 0..self.nrows {
            for j in 0..self.ncols {
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
    ) -> MatrixView<'a, T> {
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
    ) -> MatrixView<'a, T> {
        self.subview(row, row, 0, self.ncols-1)
    }
    //}}}
    //{{{ fun: rows
    pub fn rows(
        &'a self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixView<'a, T> {
        self.subview(start_row, end_row, 0, self.ncols-1)
    }
    //}}}
    //{{{ fun: col
    pub fn col(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, T> {
        self.subview(0, self.nrows-1, col, col)
    }
    //}}}
    //{{{ fun: cols
    pub fn cols(
        &'a self,
        start_col: usize,
        end_col: usize
    ) -> MatrixView<'a, T> {
        let ncols = end_col - start_col;
        self.subview(0, self.nrows-1, start_col, end_col)
    }
    //}}}
}
