//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, MatrixOps, Zero, One, tuple_index};
use super::DMatrix;
//}}}
//{{{ std imports 
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ struct: MatrixView
pub struct MatrixView<'a, Mat> 
where 
    Mat: MatrixOps + Index<(usize, usize), Output = <Mat as MatrixOps>::ScalarType>,
{
    pub(crate) matrix: &'a Mat,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
//{{{ impl: Index for MatrixView
impl<'a, Mat> Index<(usize, usize)> for MatrixView<'a, Mat>
where 
    Mat: MatrixOps + Index<(usize, usize), Output = <Mat as MatrixOps>::ScalarType>,
{
    type Output = <Mat as MatrixOps>::ScalarType;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row_loc, col_loc) = index;
        &self.matrix[(self.start_row + row_loc, self.start_col + col_loc)]
    }
}
//}}}
//{{{ struct: MatrixViewIter
pub struct MatrixViewIter<'a, Mat>
where 
    Mat: MatrixOps + Index<(usize, usize), Output = <Mat as MatrixOps>::ScalarType>,
{
    pub(crate) matrix_view: &'a MatrixView<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewIter
impl<'a, Mat> Iterator for MatrixViewIter<'a, Mat> 
where 
    Mat: MatrixOps + Index<(usize, usize), Output = <Mat as MatrixOps>::ScalarType>,
{
    type Item = &'a <Mat as MatrixOps>::ScalarType;

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
impl<'a, Mat> MatrixView<'a, Mat>
where 
    Mat: MatrixOps + Index<(usize, usize), Output = <Mat as MatrixOps>::ScalarType>,
{
    pub fn iter(&'a self) -> MatrixViewIter<'a, Mat> {
        MatrixViewIter {
            matrix_view: self,
            index: 0,
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
