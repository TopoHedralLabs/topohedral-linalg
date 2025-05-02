//! This module provides a view into a matrix, allowing for efficient access to a submatrix.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, MatrixOps, tuple_index};
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

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}

