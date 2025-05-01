//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, MatrixOps};
//}}}
//{{{ std imports 
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

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

pub struct MatrixViewIter<'a, Mat>
{
    pub(crate) matrix_view: &'a MatrixView<'a, Mat>,
    index: usize,
}

impl<'a, Mat> Iterator for MatrixViewIter {
    type Item = &'a <Mat as MatrixOps>::ScalarType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols {
            let row = self.index / self.matrix_view.ncols;
            let col = self.index % self.matrix_view.ncols;
            self.index += 1;
            Some(&self.matrix_view[(row, col)])
        } else {
            None
        }
    }
}



