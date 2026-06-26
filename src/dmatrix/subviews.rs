//! Sub-matrix view methods for [`DMatrix`].
//!
//! The view types themselves live in [`crate::subviews`]. This module provides
//! [`SubViewable`] and [`SubViewableMut`] implementations for [`DMatrix`], plus
//! `to_dmatrix()` on the view types and the `copy_from` / `set_*` helpers.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::MatrixExpr;
use crate::subviews::{MatrixView, MatrixViewMut, SubViewable, SubViewableMut};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: to_dmatrix for DMatrix views
impl<'a, T> MatrixView<'a, DMatrix<T>>
where
    T: Copy,
{
    /// Copies the view contents into a new heap-allocated [`DMatrix`].
    pub fn to_dmatrix(&self) -> DMatrix<T> {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for j in 0..self.ncols {
            for i in 0..self.nrows {
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

impl<'a, T> MatrixViewMut<'a, DMatrix<T>>
where
    T: Copy,
{
    /// Copies the view contents into a new heap-allocated [`DMatrix`].
    pub fn to_dmatrix(&self) -> DMatrix<T> {
        let mut data = Vec::with_capacity(self.nrows * self.ncols);
        for j in 0..self.ncols {
            for i in 0..self.nrows {
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

//{{{ impl: SubViewable for DMatrix
impl<T> SubViewable for DMatrix<T>
where
    T: Copy,
{
    fn subview_range<'a>(
        &'a self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, DMatrix<T>> {
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
//{{{ impl: SubViewableMut for DMatrix
impl<T> SubViewableMut for DMatrix<T>
where
    T: Copy,
{
    fn subview_range_mut<'a>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, DMatrix<T>> {
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

//{{{ impl: DMatrix copy and set helpers
impl<'a, T> DMatrix<T>
where
    T: Copy,
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
        Rhs: MatrixExpr<ScalarType = T>,
    {
        let rhs_nrows = rhs.nrows();
        let rhs_ncols = rhs.ncols();
        if self.nrows != rhs_nrows || self.ncols != rhs_ncols {
            panic!(
                "DMatrix::copy_from dimension mismatch: lhs is {}x{}, rhs is {}x{}",
                self.nrows, self.ncols, rhs_nrows, rhs_ncols
            );
        }
        rhs.eval_into(&mut self.data);
    }
    //}}}
    //{{{ fun: set_row
    /// Copies `rhs` into the row view at `row`.
    pub fn set_row<Rhs>(
        &'a mut self,
        row: usize,
        rhs: Rhs,
    ) where
        Rhs: MatrixExpr<ScalarType = T>,
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
        Rhs: MatrixExpr<ScalarType = T>,
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
        Rhs: MatrixExpr<ScalarType = T>,
    {
        let mut subview = self.subview_range_mut(start_row, end_row, start_col, end_col);
        subview.copy_from(rhs);
    }
    //}}}
}
//}}}
