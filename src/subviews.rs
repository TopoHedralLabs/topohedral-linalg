//! Generic non-owning sub-matrix views.
//!
//! Provides [`MatrixView<'a, Mat>`] and [`MatrixViewMut<'a, Mat>`], which borrow a rectangular
//! region of any matrix type that implements [`Shape`] and `Index<(usize, usize)>`. Both types
//! expose `Index<(usize, usize)>` and column-major iterators; the mutable variant additionally
//! implements `IndexMut`. The view dimensions (nrows, ncols) are always runtime values even when
//! the parent matrix has compile-time dimensions.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::apply_for_all_types;
use crate::common::{tuple_index, Field, MatrixExpr, ReduceOps, ScalarExpr, Shape, TransformOps};
use crate::dmatrix::DMatrix;
use crate::expression::binary_expr::{AddOp, BinopExpr, DivOp, MulOp, SubOp};
use crate::expression::unary_expr::{NegOp, UnaryExpr};
use std::collections::HashSet;
//}}}
//{{{ std imports
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ fn: validate_indices
fn validate_indices(
    indices: &[usize],
    limit: usize,
    axis: &str,
) {
    for &index in indices {
        assert!(
            index < limit,
            "{} index {} out of bounds for dimension {}",
            axis,
            index,
            limit
        );
    }
}
//}}}
//{{{ fn: validate_unique_indices
fn validate_unique_indices(
    indices: &[usize],
    axis: &str,
) {
    let mut found = HashSet::with_capacity(indices.len());
    for &index in indices {
        if !found.insert(index) {
            panic!("duplicate {axis} index {index} in mutable indexed view");
        }
    }
}
//}}}

//{{{ struct: MatrixView
/// Immutable subview of a matrix, borrowing a rectangular region without copying data.
pub struct MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    pub(crate) matrix: &'a Mat,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
//{{{ impl: Shape for MatrixView
impl<'a, Mat> Shape for MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}
//}}}
//{{{ impl: Index for MatrixView
impl<'a, Mat> Index<(usize, usize)> for MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        (r, c): (usize, usize),
    ) -> &Self::Output {
        &self.matrix[(self.start_row + r, self.start_col + c)]
    }
}

impl<'a, Mat> Index<usize> for MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        let (row, col) = tuple_index(index, self.nrows);
        &self.matrix[(self.start_row + row, self.start_col + col)]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &mut MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}
//}}}
//{{{ struct: MatrixViewIter
/// Column-major iterator over the elements of a [`MatrixView`].
pub struct MatrixViewIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    pub(crate) matrix_view: &'a MatrixView<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewIter
impl<'a, Mat> Iterator for MatrixViewIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&(*self.matrix_view)[(row, col)])
        } else {
            None
        }
    }
}
//}}}
//{{{ impl: MatrixView
impl<'a, Mat> MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Sized,
{
    /// Returns a column-major iterator over the elements of this view.
    pub fn iter(&'a self) -> MatrixViewIter<'a, Mat> {
        MatrixViewIter {
            matrix_view: self,
            index: 0,
        }
    }
}
//}}}
//{{{ impl: MatrixExpr for MatrixView
impl<'a, Mat, T> MatrixExpr for MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T>,
    T: Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType {
        let (row, col) = tuple_index(index, self.nrows);
        self[(row, col)]
    }
}
//}}}

//{{{ struct: MatrixViewMut
/// Mutable subview of a matrix, allowing in-place modification of a rectangular region.
pub struct MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix: &'a mut Mat,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows: usize,
    pub(crate) ncols: usize,
}
//}}}
//{{{ impl: Shape for MatrixViewMut
impl<'a, Mat> Shape for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    fn nrows(&self) -> usize {
        self.nrows
    }

    fn ncols(&self) -> usize {
        self.ncols
    }
}
//}}}
//{{{ impl: Index/IndexMut for MatrixViewMut
impl<'a, Mat> Index<(usize, usize)> for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        (r, c): (usize, usize),
    ) -> &Self::Output {
        &self.matrix[(self.start_row + r, self.start_col + c)]
    }
}

impl<'a, Mat> Index<usize> for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        let (row, col) = tuple_index(index, self.nrows);
        &self.matrix[(self.start_row + row, self.start_col + col)]
    }
}

impl<'a, Mat> IndexMut<(usize, usize)> for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    fn index_mut(
        &mut self,
        (r, c): (usize, usize),
    ) -> &mut Self::Output {
        &mut self.matrix[(self.start_row + r, self.start_col + c)]
    }
}

impl<'a, Mat> IndexMut<usize> for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        let (row, col) = tuple_index(index, self.nrows);
        &mut self.matrix[(self.start_row + row, self.start_col + col)]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &mut MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}
//}}}
//{{{ struct: MatrixViewMutIter
/// Shared (immutable) iterator over the elements of a [`MatrixViewMut`].
pub struct MatrixViewMutIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix_view: &'a MatrixViewMut<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIter
impl<'a, Mat> Iterator for MatrixViewMutIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            Some(&(*self.matrix_view)[(row, col)])
        } else {
            None
        }
    }
}
//}}}
//{{{ struct: MatrixViewMutIterMut
/// Mutable iterator over the elements of a [`MatrixViewMut`].
pub struct MatrixViewMutIterMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix_view: &'a mut MatrixViewMut<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIterMut
impl<'a, Mat> Iterator for MatrixViewMutIterMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a mut Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            unsafe {
                // Convert to a raw pointer and then back to a reference with lifetime 'a.
                // Safe because each element is yielded at most once (index advances).
                let ptr = &mut (*self.matrix_view)[(row, col)] as *mut Mat::Output;
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }
}
//}}}
//{{{ impl: MatrixViewMut
impl<'a, Mat> MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Copy + Sized,
{
    /// Returns a shared column-major iterator over the elements of this mutable view.
    pub fn iter(&'a self) -> MatrixViewMutIter<'a, Mat> {
        MatrixViewMutIter {
            matrix_view: self,
            index: 0,
        }
    }

    /// Returns a mutable column-major iterator over the elements of this mutable view.
    pub fn iter_mut(&'a mut self) -> MatrixViewMutIterMut<'a, Mat> {
        MatrixViewMutIterMut {
            matrix_view: self,
            index: 0,
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
        Rhs: MatrixExpr<ScalarType = Mat::Output>,
    {
        let rhs_nrows = rhs.nrows();
        let rhs_ncols = rhs.ncols();
        if self.nrows != rhs_nrows || self.ncols != rhs_ncols {
            panic!(
                "MatrixViewMut::copy_from dimension mismatch: lhs is {}x{}, rhs is {}x{}",
                self.nrows, self.ncols, rhs_nrows, rhs_ncols
            );
        }
        for index in 0..self.nrows * self.ncols {
            (*self)[index] = rhs.linear_value(index);
        }
    }
}
//}}}
//{{{ impl: MatrixExpr for MatrixViewMut
impl<'a, Mat, T> MatrixExpr for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>,
    T: Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType {
        let (row, col) = tuple_index(index, self.nrows);
        self[(row, col)]
    }
}
//}}}

//{{{ struct: IndexedMatrixView
/// Immutable subview of a matrix selected by row and column index lists.
pub struct IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    pub(crate) matrix: &'a Mat,
    pub(crate) row_indices: Vec<usize>,
    pub(crate) col_indices: Vec<usize>,
}
//}}}
//{{{ impl: Shape for IndexedMatrixView
impl<'a, Mat> Shape for IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    fn nrows(&self) -> usize {
        self.row_indices.len()
    }

    fn ncols(&self) -> usize {
        self.col_indices.len()
    }
}
//}}}
//{{{ impl: Index for IndexedMatrixView
impl<'a, Mat> Index<(usize, usize)> for IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        (r, c): (usize, usize),
    ) -> &Self::Output {
        &self.matrix[(self.row_indices[r], self.col_indices[c])]
    }
}

impl<'a, Mat> Index<usize> for IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        let (row, col) = tuple_index(index, self.nrows());
        &self.matrix[(self.row_indices[row], self.col_indices[col])]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &mut IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}
//}}}
//{{{ struct: IndexedMatrixViewIter
/// Column-major iterator over the elements of an [`IndexedMatrixView`].
pub struct IndexedMatrixViewIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    pub(crate) matrix_view: &'a IndexedMatrixView<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for IndexedMatrixViewIter
impl<'a, Mat> Iterator for IndexedMatrixViewIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows() * self.matrix_view.ncols() {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows());
            self.index += 1;
            Some(&(*self.matrix_view)[(row, col)])
        } else {
            None
        }
    }
}
//}}}
//{{{ impl: IndexedMatrixView
impl<'a, Mat> IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Copy + Sized,
{
    /// Returns a column-major iterator over the elements of this view.
    pub fn iter(&'a self) -> IndexedMatrixViewIter<'a, Mat> {
        IndexedMatrixViewIter {
            matrix_view: self,
            index: 0,
        }
    }

    /// Copies the view contents into a new heap-allocated [`DMatrix`].
    pub fn to_dmatrix(&self) -> DMatrix<Mat::Output> {
        let nrows = self.nrows();
        let ncols = self.ncols();
        let mut data = Vec::with_capacity(nrows * ncols);
        for j in 0..ncols {
            for i in 0..nrows {
                data.push(self[(i, j)]);
            }
        }
        DMatrix { data, nrows, ncols }
    }
}
//}}}
//{{{ impl: MatrixExpr for IndexedMatrixView
impl<'a, Mat, T> MatrixExpr for IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T>,
    T: Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType {
        let (row, col) = tuple_index(index, self.nrows());
        self[(row, col)]
    }
}
//}}}
//{{{ impl: ReduceOps for IndexedMatrixView
impl<'a, Mat, T> ReduceOps for IndexedMatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T>,
    T: Copy,
{
    type Item = T;
    type Index = (usize, usize);

    fn fold<B, F>(
        &self,
        init: B,
        mut f: F,
    ) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let mut acc = init;
        for col in 0..self.ncols() {
            for row in 0..self.nrows() {
                acc = f(acc, self[(row, col)]);
            }
        }
        acc
    }

    fn fold_indexed<B, F>(
        &self,
        init: B,
        mut f: F,
    ) -> B
    where
        F: FnMut(B, Self::Index, Self::Item) -> B,
    {
        let mut acc = init;
        for col in 0..self.ncols() {
            for row in 0..self.nrows() {
                acc = f(acc, (row, col), self[(row, col)]);
            }
        }
        acc
    }
}
//}}}

//{{{ struct: IndexedMatrixViewMut
/// Mutable subview of a matrix selected by unique row and column index lists.
pub struct IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix: &'a mut Mat,
    pub(crate) row_indices: Vec<usize>,
    pub(crate) col_indices: Vec<usize>,
}
//}}}
//{{{ impl: Shape for IndexedMatrixViewMut
impl<'a, Mat> Shape for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    fn nrows(&self) -> usize {
        self.row_indices.len()
    }

    fn ncols(&self) -> usize {
        self.col_indices.len()
    }
}
//}}}
//{{{ impl: Index/IndexMut for IndexedMatrixViewMut
impl<'a, Mat> Index<(usize, usize)> for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        (r, c): (usize, usize),
    ) -> &Self::Output {
        &self.matrix[(self.row_indices[r], self.col_indices[c])]
    }
}

impl<'a, Mat> Index<usize> for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        let (row, col) = tuple_index(index, self.nrows());
        &self.matrix[(self.row_indices[row], self.col_indices[col])]
    }
}

impl<'a, Mat> IndexMut<(usize, usize)> for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    fn index_mut(
        &mut self,
        (r, c): (usize, usize),
    ) -> &mut Self::Output {
        &mut self.matrix[(self.row_indices[r], self.col_indices[c])]
    }
}

impl<'a, Mat> IndexMut<usize> for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        let (row, col) = tuple_index(index, self.nrows());
        &mut self.matrix[(self.row_indices[row], self.col_indices[col])]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}

impl<'a, Mat> Index<(usize, usize)> for &mut IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    type Output = Mat::Output;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output {
        &(**self)[index]
    }
}
//}}}
//{{{ struct: IndexedMatrixViewMutIter
/// Shared iterator over the elements of an [`IndexedMatrixViewMut`].
pub struct IndexedMatrixViewMutIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix_view: &'a IndexedMatrixViewMut<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for IndexedMatrixViewMutIter
impl<'a, Mat> Iterator for IndexedMatrixViewMutIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows() * self.matrix_view.ncols() {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows());
            self.index += 1;
            Some(&(*self.matrix_view)[(row, col)])
        } else {
            None
        }
    }
}
//}}}
//{{{ struct: IndexedMatrixViewMutIterMut
/// Mutable iterator over the elements of an [`IndexedMatrixViewMut`].
pub struct IndexedMatrixViewMutIterMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix_view: &'a mut IndexedMatrixViewMut<'a, Mat>,
    index: usize,
}
//}}}
//{{{ impl: Iterator for IndexedMatrixViewMutIterMut
impl<'a, Mat> Iterator for IndexedMatrixViewMutIterMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a mut Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matrix_view.nrows() * self.matrix_view.ncols() {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows());
            self.index += 1;
            unsafe {
                // Safe because mutable indexed views reject duplicate row/column indices.
                let ptr = &mut (*self.matrix_view)[(row, col)] as *mut Mat::Output;
                Some(&mut *ptr)
            }
        } else {
            None
        }
    }
}
//}}}
//{{{ impl: IndexedMatrixViewMut
impl<'a, Mat> IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Copy + Sized,
{
    /// Returns a shared column-major iterator over the elements of this mutable view.
    pub fn iter(&'a self) -> IndexedMatrixViewMutIter<'a, Mat> {
        IndexedMatrixViewMutIter {
            matrix_view: self,
            index: 0,
        }
    }

    /// Returns a mutable column-major iterator over the elements of this mutable view.
    pub fn iter_mut(&'a mut self) -> IndexedMatrixViewMutIterMut<'a, Mat> {
        IndexedMatrixViewMutIterMut {
            matrix_view: self,
            index: 0,
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
        Rhs: MatrixExpr<ScalarType = Mat::Output>,
    {
        let rhs_nrows = rhs.nrows();
        let rhs_ncols = rhs.ncols();
        if self.nrows() != rhs_nrows || self.ncols() != rhs_ncols {
            panic!(
                "IndexedMatrixViewMut::copy_from dimension mismatch: lhs is {}x{}, rhs is {}x{}",
                self.nrows(),
                self.ncols(),
                rhs_nrows,
                rhs_ncols
            );
        }
        for index in 0..self.nrows() * self.ncols() {
            (*self)[index] = rhs.linear_value(index);
        }
    }

    /// Copies the view contents into a new heap-allocated [`DMatrix`].
    pub fn to_dmatrix(&self) -> DMatrix<Mat::Output> {
        let nrows = self.nrows();
        let ncols = self.ncols();
        let mut data = Vec::with_capacity(nrows * ncols);
        for j in 0..ncols {
            for i in 0..nrows {
                data.push(self[(i, j)]);
            }
        }
        DMatrix { data, nrows, ncols }
    }
}
//}}}
//{{{ impl: MatrixExpr for IndexedMatrixViewMut
impl<'a, Mat, T> MatrixExpr for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>,
    T: Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType {
        let (row, col) = tuple_index(index, self.nrows());
        self[(row, col)]
    }
}
//}}}
//{{{ impl: ReduceOps for IndexedMatrixViewMut
impl<'a, Mat, T> ReduceOps for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>,
    T: Copy,
{
    type Item = T;
    type Index = (usize, usize);

    fn fold<B, F>(
        &self,
        init: B,
        mut f: F,
    ) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let mut acc = init;
        for col in 0..self.ncols() {
            for row in 0..self.nrows() {
                acc = f(acc, self[(row, col)]);
            }
        }
        acc
    }

    fn fold_indexed<B, F>(
        &self,
        init: B,
        mut f: F,
    ) -> B
    where
        F: FnMut(B, Self::Index, Self::Item) -> B,
    {
        let mut acc = init;
        for col in 0..self.ncols() {
            for row in 0..self.nrows() {
                acc = f(acc, (row, col), self[(row, col)]);
            }
        }
        acc
    }
}
//}}}
//{{{ impl: TransformOps for IndexedMatrixViewMut
impl<'a, Mat, T> TransformOps for IndexedMatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>,
    T: Copy,
{
    type ScalarType = T;

    fn transform<F>(
        &mut self,
        mut f: F,
    ) where
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        for col in 0..self.ncols() {
            for row in 0..self.nrows() {
                let value = self[(row, col)];
                (*self)[(row, col)] = f(value);
            }
        }
    }
}
//}}}

//{{{ collection: lazy elementwise operators for views
macro_rules! impl_view_matrix_rhs_op {
    ($view:ident, [$($bounds:tt)+], $trait:ident, $method:ident, $op:ty) => {
        impl<'a, Mat, T, Rhs> $trait<Rhs> for $view<'a, Mat>
        where
            $($bounds)+,
            T: Field + Copy,
            Rhs: MatrixExpr<ScalarType = T>,
        {
            type Output = BinopExpr<Self, Rhs, T, $op>;

            #[inline]
            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                assert_eq!(self.nrows(), rhs.nrows(), "view expression row dimension mismatch");
                assert_eq!(self.ncols(), rhs.ncols(), "view expression column dimension mismatch");
                let nr = self.nrows();
                let nc = self.ncols();
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, 'b, Mat, T, Rhs> $trait<Rhs> for &'b $view<'a, Mat>
        where
            $($bounds)+,
            T: Field + Copy,
            Rhs: MatrixExpr<ScalarType = T>,
        {
            type Output = BinopExpr<Self, Rhs, T, $op>;

            #[inline]
            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                assert_eq!(self.nrows(), rhs.nrows(), "view expression row dimension mismatch");
                assert_eq!(self.ncols(), rhs.ncols(), "view expression column dimension mismatch");
                let nr = self.nrows();
                let nc = self.ncols();
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, 'b, Mat, T, Rhs> $trait<Rhs> for &'b mut $view<'a, Mat>
        where
            $($bounds)+,
            T: Field + Copy,
            Rhs: MatrixExpr<ScalarType = T>,
        {
            type Output = BinopExpr<Self, Rhs, T, $op>;

            #[inline]
            fn $method(
                self,
                rhs: Rhs,
            ) -> Self::Output
            {
                assert_eq!(self.nrows(), rhs.nrows(), "view expression row dimension mismatch");
                assert_eq!(self.ncols(), rhs.ncols(), "view expression column dimension mismatch");
                let nr = self.nrows();
                let nc = self.ncols();
                BinopExpr {
                    a: self,
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_view_scalar_rhs_op {
    ($scalar:ty, $view:ident, [$($bounds:tt)+], $trait:ident, $method:ident, $op:ty) => {
        impl<'a, Mat> $trait<$scalar> for $view<'a, Mat>
        where
            $($bounds)+,
        {
            type Output = BinopExpr<Self, ScalarExpr<$scalar>, $scalar, $op>;

            #[inline]
            fn $method(
                self,
                rhs: $scalar,
            ) -> Self::Output
            {
                let nr = self.nrows();
                let nc = self.ncols();
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, 'b, Mat> $trait<$scalar> for &'b $view<'a, Mat>
        where
            $($bounds)+,
        {
            type Output = BinopExpr<Self, ScalarExpr<$scalar>, $scalar, $op>;

            #[inline]
            fn $method(
                self,
                rhs: $scalar,
            ) -> Self::Output
            {
                let nr = self.nrows();
                let nc = self.ncols();
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, 'b, Mat> $trait<$scalar> for &'b mut $view<'a, Mat>
        where
            $($bounds)+,
        {
            type Output = BinopExpr<Self, ScalarExpr<$scalar>, $scalar, $op>;

            #[inline]
            fn $method(
                self,
                rhs: $scalar,
            ) -> Self::Output
            {
                let nr = self.nrows();
                let nc = self.ncols();
                BinopExpr {
                    a: self,
                    b: ScalarExpr::new(rhs, nr, nc),
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_view_scalar_lhs_op {
    ($scalar:ty, $view:ident, [$($bounds:tt)+], $trait:ident, $method:ident, $op:ty) => {
        impl<'a, Mat> $trait<$view<'a, Mat>> for $scalar
        where
            $($bounds)+,
        {
            type Output = BinopExpr<ScalarExpr<$scalar>, $view<'a, Mat>, $scalar, $op>;

            #[inline]
            fn $method(
                self,
                rhs: $view<'a, Mat>,
            ) -> Self::Output
            {
                let nr = rhs.nrows();
                let nc = rhs.ncols();
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, 'b, Mat> $trait<&'b $view<'a, Mat>> for $scalar
        where
            $($bounds)+,
        {
            type Output = BinopExpr<ScalarExpr<$scalar>, &'b $view<'a, Mat>, $scalar, $op>;

            #[inline]
            fn $method(
                self,
                rhs: &'b $view<'a, Mat>,
            ) -> Self::Output
            {
                let nr = rhs.nrows();
                let nc = rhs.ncols();
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, 'b, Mat> $trait<&'b mut $view<'a, Mat>> for $scalar
        where
            $($bounds)+,
        {
            type Output = BinopExpr<ScalarExpr<$scalar>, &'b mut $view<'a, Mat>, $scalar, $op>;

            #[inline]
            fn $method(
                self,
                rhs: &'b mut $view<'a, Mat>,
            ) -> Self::Output
            {
                let nr = rhs.nrows();
                let nc = rhs.ncols();
                BinopExpr {
                    a: ScalarExpr::new(self, nr, nc),
                    b: rhs,
                    nrows: nr,
                    ncols: nc,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    };
}

macro_rules! impl_view_neg_op {
    ($view:ident, [$($bounds:tt)+]) => {
        impl<'a, Mat, T> Neg for $view<'a, Mat>
        where
            $($bounds)+,
            T: Field + Copy,
        {
            type Output = UnaryExpr<Self, T, NegOp>;

            #[inline]
            fn neg(self) -> Self::Output
            {
                UnaryExpr::new(self, NegOp)
            }
        }

        impl<'a, 'b, Mat, T> Neg for &'b $view<'a, Mat>
        where
            $($bounds)+,
            T: Field + Copy,
        {
            type Output = UnaryExpr<Self, T, NegOp>;

            #[inline]
            fn neg(self) -> Self::Output
            {
                UnaryExpr::new(self, NegOp)
            }
        }

        impl<'a, 'b, Mat, T> Neg for &'b mut $view<'a, Mat>
        where
            $($bounds)+,
            T: Field + Copy,
        {
            type Output = UnaryExpr<Self, T, NegOp>;

            #[inline]
            fn neg(self) -> Self::Output
            {
                UnaryExpr::new(self, NegOp)
            }
        }
    };
}

macro_rules! impl_view_matrix_rhs_ops {
    ($view:ident, [$($bounds:tt)+]) => {
        impl_view_matrix_rhs_op!($view, [$($bounds)+], Add, add, AddOp);
        impl_view_matrix_rhs_op!($view, [$($bounds)+], Sub, sub, SubOp);
        impl_view_matrix_rhs_op!($view, [$($bounds)+], Mul, mul, MulOp);
        impl_view_matrix_rhs_op!($view, [$($bounds)+], Div, div, DivOp);
        impl_view_neg_op!($view, [$($bounds)+]);
    };
}

impl_view_matrix_rhs_ops!(MatrixView, [Mat: Shape + Index<(usize, usize), Output = T>]);
impl_view_matrix_rhs_ops!(
    MatrixViewMut,
    [Mat: Shape + Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>]
);
impl_view_matrix_rhs_ops!(IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = T>]);
impl_view_matrix_rhs_ops!(
    IndexedMatrixViewMut,
    [Mat: Shape + Index<(usize, usize), Output = T> + IndexMut<(usize, usize)>]
);

macro_rules! impl_matrix_view_scalar_ops {
    ($scalar:ty) => {
        impl_view_scalar_rhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Add, add, AddOp);
        impl_view_scalar_rhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Sub, sub, SubOp);
        impl_view_scalar_rhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Mul, mul, MulOp);
        impl_view_scalar_rhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Div, div, DivOp);
        impl_view_scalar_lhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Add, add, AddOp);
        impl_view_scalar_lhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Sub, sub, SubOp);
        impl_view_scalar_lhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Mul, mul, MulOp);
        impl_view_scalar_lhs_op!($scalar, MatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Div, div, DivOp);
    };
}

macro_rules! impl_matrix_view_mut_scalar_ops {
    ($scalar:ty) => {
        impl_view_scalar_rhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Add, add, AddOp);
        impl_view_scalar_rhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Sub, sub, SubOp);
        impl_view_scalar_rhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Mul, mul, MulOp);
        impl_view_scalar_rhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Div, div, DivOp);
        impl_view_scalar_lhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Add, add, AddOp);
        impl_view_scalar_lhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Sub, sub, SubOp);
        impl_view_scalar_lhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Mul, mul, MulOp);
        impl_view_scalar_lhs_op!($scalar, MatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Div, div, DivOp);
    };
}

macro_rules! impl_indexed_matrix_view_scalar_ops {
    ($scalar:ty) => {
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Add, add, AddOp);
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Sub, sub, SubOp);
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Mul, mul, MulOp);
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Div, div, DivOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Add, add, AddOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Sub, sub, SubOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Mul, mul, MulOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixView, [Mat: Shape + Index<(usize, usize), Output = $scalar>], Div, div, DivOp);
    };
}

macro_rules! impl_indexed_matrix_view_mut_scalar_ops {
    ($scalar:ty) => {
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Add, add, AddOp);
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Sub, sub, SubOp);
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Mul, mul, MulOp);
        impl_view_scalar_rhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Div, div, DivOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Add, add, AddOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Sub, sub, SubOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Mul, mul, MulOp);
        impl_view_scalar_lhs_op!($scalar, IndexedMatrixViewMut, [Mat: Shape + Index<(usize, usize), Output = $scalar> + IndexMut<(usize, usize)>], Div, div, DivOp);
    };
}

apply_for_all_types!(impl_matrix_view_scalar_ops);
apply_for_all_types!(impl_matrix_view_mut_scalar_ops);
apply_for_all_types!(impl_indexed_matrix_view_scalar_ops);
apply_for_all_types!(impl_indexed_matrix_view_mut_scalar_ops);
//}}}

//{{{ trait: SubViewable
/// Immutable subview constructor methods for any matrix type.
///
/// Requires `Shape + Index<(usize, usize)> + Sized` as supertraits so that
/// `MatrixView<'a, Self>` is a valid return type. Only `subview` must be
/// provided; `row`, `rows`, `col`, and `cols` have default implementations
/// that delegate to it.
pub trait SubViewable: Shape + Index<(usize, usize)> + Sized {
    fn subview_range<'a>(
        &'a self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, Self>;

    fn row<'a>(
        &'a self,
        row: usize,
    ) -> MatrixView<'a, Self> {
        self.subview_range(row, row, 0, self.ncols() - 1)
    }

    fn rows_range<'a>(
        &'a self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixView<'a, Self> {
        self.subview_range(start_row, end_row, 0, self.ncols() - 1)
    }

    fn col<'a>(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, Self> {
        self.subview_range(0, self.nrows() - 1, col, col)
    }

    fn cols_range<'a>(
        &'a self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, Self> {
        self.subview_range(0, self.nrows() - 1, start_col, end_col)
    }

    fn rows_indices<'a, I>(
        &'a self,
        row_indices: I,
    ) -> IndexedMatrixView<'a, Self>
    where
        I: AsRef<[usize]>,
    {
        let row_indices = row_indices.as_ref().to_vec();
        validate_indices(&row_indices, self.nrows(), "row");
        IndexedMatrixView {
            matrix: self,
            row_indices,
            col_indices: (0..self.ncols()).collect(),
        }
    }

    fn cols_indices<'a, I>(
        &'a self,
        col_indices: I,
    ) -> IndexedMatrixView<'a, Self>
    where
        I: AsRef<[usize]>,
    {
        let col_indices = col_indices.as_ref().to_vec();
        validate_indices(&col_indices, self.ncols(), "column");
        IndexedMatrixView {
            matrix: self,
            row_indices: (0..self.nrows()).collect(),
            col_indices,
        }
    }

    fn subview_indices<'a, R, C>(
        &'a self,
        row_indices: R,
        col_indices: C,
    ) -> IndexedMatrixView<'a, Self>
    where
        R: AsRef<[usize]>,
        C: AsRef<[usize]>,
    {
        let row_indices = row_indices.as_ref().to_vec();
        let col_indices = col_indices.as_ref().to_vec();
        validate_indices(&row_indices, self.nrows(), "row");
        validate_indices(&col_indices, self.ncols(), "column");
        IndexedMatrixView {
            matrix: self,
            row_indices,
            col_indices,
        }
    }
}
//}}}
//{{{ trait: SubViewableMut
/// Mutable subview constructor methods for any matrix type.
///
/// Extends [`SubViewable`] with `IndexMut<(usize, usize)>`. Only
/// `subview_mut` must be provided; the remaining methods have defaults.
///
/// Default methods bind dimension reads to locals before the `&mut self`
/// call to avoid simultaneous shared + exclusive borrow of `self`.
pub trait SubViewableMut: SubViewable + IndexMut<(usize, usize)> {
    fn subview_range_mut<'a>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, Self>;

    fn row_mut<'a>(
        &'a mut self,
        row: usize,
    ) -> MatrixViewMut<'a, Self> {
        let ncols = self.ncols();
        self.subview_range_mut(row, row, 0, ncols - 1)
    }

    fn rows_range_mut<'a>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixViewMut<'a, Self> {
        let ncols = self.ncols();
        self.subview_range_mut(start_row, end_row, 0, ncols - 1)
    }

    fn col_mut<'a>(
        &'a mut self,
        col: usize,
    ) -> MatrixViewMut<'a, Self> {
        let nrows = self.nrows();
        self.subview_range_mut(0, nrows - 1, col, col)
    }

    fn cols_range_mut<'a>(
        &'a mut self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, Self> {
        let nrows = self.nrows();
        self.subview_range_mut(0, nrows - 1, start_col, end_col)
    }

    fn rows_indices_mut<'a, I>(
        &'a mut self,
        row_indices: I,
    ) -> IndexedMatrixViewMut<'a, Self>
    where
        I: AsRef<[usize]>,
    {
        let ncols = self.ncols();
        let nrows = self.nrows();
        let row_indices = row_indices.as_ref().to_vec();
        validate_indices(&row_indices, nrows, "row");
        validate_unique_indices(&row_indices, "row");
        IndexedMatrixViewMut {
            matrix: self,
            row_indices,
            col_indices: (0..ncols).collect(),
        }
    }

    fn cols_indices_mut<'a, I>(
        &'a mut self,
        col_indices: I,
    ) -> IndexedMatrixViewMut<'a, Self>
    where
        I: AsRef<[usize]>,
    {
        let ncols = self.ncols();
        let nrows = self.nrows();
        let col_indices = col_indices.as_ref().to_vec();
        validate_indices(&col_indices, ncols, "column");
        validate_unique_indices(&col_indices, "column");
        IndexedMatrixViewMut {
            matrix: self,
            row_indices: (0..nrows).collect(),
            col_indices,
        }
    }

    fn subview_indices_mut<'a, R, C>(
        &'a mut self,
        row_indices: R,
        col_indices: C,
    ) -> IndexedMatrixViewMut<'a, Self>
    where
        R: AsRef<[usize]>,
        C: AsRef<[usize]>,
    {
        let ncols = self.ncols();
        let nrows = self.nrows();
        let row_indices = row_indices.as_ref().to_vec();
        let col_indices = col_indices.as_ref().to_vec();
        validate_indices(&row_indices, nrows, "row");
        validate_indices(&col_indices, ncols, "column");
        validate_unique_indices(&row_indices, "row");
        validate_unique_indices(&col_indices, "column");
        IndexedMatrixViewMut {
            matrix: self,
            row_indices,
            col_indices,
        }
    }
}
//}}}

//{{{ collection: MaskedView
/// Read-only gather view over entries selected by a boolean matrix expression.
pub struct MaskedView<'a, Mat, Mask>
where
    Mat: Shape + Index<(usize, usize)>,
    Mask: MatrixExpr<ScalarType = bool>,
{
    matrix: &'a Mat,
    mask: Mask,
    selected: usize,
}

impl<'a, Mat, Mask> Shape for MaskedView<'a, Mat, Mask>
where
    Mat: Shape + Index<(usize, usize)>,
    Mask: MatrixExpr<ScalarType = bool>,
{
    fn nrows(&self) -> usize {
        self.selected
    }

    fn ncols(&self) -> usize {
        1
    }
}

/// Iterator over entries selected by a [`MaskedView`], in column-major order.
pub struct MaskedViewIter<'view, 'matrix, Mat, Mask>
where
    Mat: Shape + Index<(usize, usize)>,
    Mask: MatrixExpr<ScalarType = bool>,
{
    view: &'view MaskedView<'matrix, Mat, Mask>,
    index: usize,
}

impl<'view, 'matrix, Mat, Mask> Iterator for MaskedViewIter<'view, 'matrix, Mat, Mask>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Sized,
    Mask: MatrixExpr<ScalarType = bool>,
{
    type Item = &'view Mat::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.view.matrix.nrows() * self.view.matrix.ncols();
        while self.index < len {
            let index = self.index;
            self.index += 1;
            if self.view.mask.linear_value(index) {
                let (row, col) = tuple_index(index, self.view.matrix.nrows());
                return Some(&self.view.matrix[(row, col)]);
            }
        }
        None
    }
}

impl<'a, Mat, Mask> MaskedView<'a, Mat, Mask>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Copy + Sized,
    Mask: MatrixExpr<ScalarType = bool>,
{
    /// Iterates over selected entries in column-major source order.
    pub fn iter(&self) -> MaskedViewIter<'_, 'a, Mat, Mask> {
        MaskedViewIter {
            view: self,
            index: 0,
        }
    }

    /// Materialises selected entries as a `K × 1` dynamic matrix.
    pub fn to_dmatrix(&self) -> DMatrix<Mat::Output> {
        let data = self.iter().copied().collect();
        DMatrix {
            data,
            nrows: self.selected,
            ncols: 1,
        }
    }
}

/// Adds NumPy-style boolean gather selection to matrix-like values.
pub trait Maskable: Shape + Index<(usize, usize)> + Sized
where
    Self::Output: Copy,
{
    fn masked<Mask>(
        &self,
        mask: Mask,
    ) -> MaskedView<'_, Self, Mask>
    where
        Mask: MatrixExpr<ScalarType = bool>,
    {
        assert_eq!(
            self.size(),
            mask.size(),
            "masked selection dimension mismatch: source is {}x{}, mask is {}x{}",
            self.nrows(),
            self.ncols(),
            mask.nrows(),
            mask.ncols()
        );
        let selected = (0..self.nrows() * self.ncols())
            .filter(|&index| mask.linear_value(index))
            .count();
        MaskedView {
            matrix: self,
            mask,
            selected,
        }
    }
}

impl<X> Maskable for X
where
    X: Shape + Index<(usize, usize)>,
    X::Output: Copy,
{
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_indices_accepts_empty_and_in_bounds_indices() {
        validate_indices(&[], 0, "row");
        validate_indices(&[0, 2, 4], 5, "row");
    }

    #[test]
    #[should_panic(expected = "row index 5 out of bounds for dimension 5")]
    fn validate_indices_rejects_out_of_bounds_indices() {
        validate_indices(&[0, 5], 5, "row");
    }

    #[test]
    fn validate_unique_indices_accepts_empty_and_unique_indices() {
        validate_unique_indices(&[], "column");
        validate_unique_indices(&[3, 1, 4], "column");
    }

    #[test]
    #[should_panic(expected = "duplicate column index 3 in mutable indexed view")]
    fn validate_unique_indices_rejects_duplicates() {
        validate_unique_indices(&[3, 1, 3], "column");
    }
}
//}}}
