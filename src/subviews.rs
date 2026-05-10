//! Generic non-owning sub-matrix views.
//!
//! Provides [`MatrixView<'a, Mat>`] and [`MatrixViewMut<'a, Mat>`], which borrow a rectangular
//! region of any matrix type that implements [`Shape`] and `Index<(usize, usize)>`. Both types
//! expose `Index<(usize, usize)>` and column-major iterators; the mutable variant additionally
//! implements `IndexMut`. The view dimensions (nrows, ncols) are always runtime values even when
//! the parent matrix has compile-time dimensions.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::{tuple_index, Field, Shape};
//}}}
//{{{ std imports
use std::ops::{Index, IndexMut};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: MatrixView
/// Immutable subview of a matrix, borrowing a rectangular region without copying data.
pub struct MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
{
    pub(crate) matrix:    &'a Mat,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows:     usize,
    pub(crate) ncols:     usize,
}
//}}}
//{{{ impl: Shape for MatrixView
impl<'a, Mat> Shape for MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
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
    ) -> &Self::Output
    {
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
    ) -> &Self::Output
    {
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
    ) -> &Self::Output
    {
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
    ) -> &Self::Output
    {
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
    index:                  usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewIter
impl<'a, Mat> Iterator for MatrixViewIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a Mat::Output;

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
//{{{ impl: MatrixView
impl<'a, Mat> MatrixView<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)>,
    Mat::Output: Sized,
{
    /// Returns a column-major iterator over the elements of this view.
    pub fn iter(&'a self) -> MatrixViewIter<'a, Mat>
    {
        MatrixViewIter {
            matrix_view: self,
            index: 0,
        }
    }
}
//}}}

//{{{ struct: MatrixViewMut
/// Mutable subview of a matrix, allowing in-place modification of a rectangular region.
pub struct MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix:    &'a mut Mat,
    pub(crate) start_row: usize,
    pub(crate) start_col: usize,
    pub(crate) nrows:     usize,
    pub(crate) ncols:     usize,
}
//}}}
//{{{ impl: Shape for MatrixViewMut
impl<'a, Mat> Shape for MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
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
    ) -> &Self::Output
    {
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
    ) -> &Self::Output
    {
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
    ) -> &mut Self::Output
    {
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
    ) -> &mut Self::Output
    {
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
    ) -> &Self::Output
    {
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
    ) -> &Self::Output
    {
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
    index:                  usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIter
impl<'a, Mat> Iterator for MatrixViewMutIter<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a Mat::Output;

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
//{{{ struct: MatrixViewMutIterMut
/// Mutable iterator over the elements of a [`MatrixViewMut`].
pub struct MatrixViewMutIterMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
{
    pub(crate) matrix_view: &'a mut MatrixViewMut<'a, Mat>,
    index:                  usize,
}
//}}}
//{{{ impl: Iterator for MatrixViewMutIterMut
impl<'a, Mat> Iterator for MatrixViewMutIterMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Sized,
{
    type Item = &'a mut Mat::Output;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < self.matrix_view.nrows * self.matrix_view.ncols
        {
            let (row, col) = tuple_index(self.index, self.matrix_view.nrows);
            self.index += 1;
            unsafe {
                // Convert to a raw pointer and then back to a reference with lifetime 'a.
                // Safe because each element is yielded at most once (index advances).
                let ptr = &mut (*self.matrix_view)[(row, col)] as *mut Mat::Output;
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
//{{{ impl: MatrixViewMut
impl<'a, Mat> MatrixViewMut<'a, Mat>
where
    Mat: Shape + Index<(usize, usize)> + IndexMut<(usize, usize)>,
    Mat::Output: Field + Copy + Sized,
{
    /// Returns a shared column-major iterator over the elements of this mutable view.
    pub fn iter(&'a self) -> MatrixViewMutIter<'a, Mat>
    {
        MatrixViewMutIter {
            matrix_view: self,
            index: 0,
        }
    }

    /// Returns a mutable column-major iterator over the elements of this mutable view.
    pub fn iter_mut(&'a mut self) -> MatrixViewMutIterMut<'a, Mat>
    {
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
        Rhs: Shape + Index<(usize, usize), Output = Mat::Output>,
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

//{{{ trait: SubViewable
/// Immutable subview constructor methods for any matrix type.
///
/// Requires `Shape + Index<(usize, usize)> + Sized` as supertraits so that
/// `MatrixView<'a, Self>` is a valid return type. Only `subview` must be
/// provided; `row`, `rows`, `col`, and `cols` have default implementations
/// that delegate to it.
pub trait SubViewable: Shape + Index<(usize, usize)> + Sized
{
    fn subview<'a>(
        &'a self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, Self>;

    fn row<'a>(
        &'a self,
        row: usize,
    ) -> MatrixView<'a, Self>
    {
        self.subview(row, row, 0, self.ncols() - 1)
    }

    fn rows<'a>(
        &'a self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixView<'a, Self>
    {
        self.subview(start_row, end_row, 0, self.ncols() - 1)
    }

    fn col<'a>(
        &'a self,
        col: usize,
    ) -> MatrixView<'a, Self>
    {
        self.subview(0, self.nrows() - 1, col, col)
    }

    fn cols<'a>(
        &'a self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixView<'a, Self>
    {
        self.subview(0, self.nrows() - 1, start_col, end_col)
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
pub trait SubViewableMut: SubViewable + IndexMut<(usize, usize)>
{
    fn subview_mut<'a>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, Self>;

    fn row_mut<'a>(
        &'a mut self,
        row: usize,
    ) -> MatrixViewMut<'a, Self>
    {
        let ncols = self.ncols();
        self.subview_mut(row, row, 0, ncols - 1)
    }

    fn rows_mut<'a>(
        &'a mut self,
        start_row: usize,
        end_row: usize,
    ) -> MatrixViewMut<'a, Self>
    {
        let ncols = self.ncols();
        self.subview_mut(start_row, end_row, 0, ncols - 1)
    }

    fn col_mut<'a>(
        &'a mut self,
        col: usize,
    ) -> MatrixViewMut<'a, Self>
    {
        let nrows = self.nrows();
        self.subview_mut(0, nrows - 1, col, col)
    }

    fn cols_mut<'a>(
        &'a mut self,
        start_col: usize,
        end_col: usize,
    ) -> MatrixViewMut<'a, Self>
    {
        let nrows = self.nrows();
        self.subview_mut(0, nrows - 1, start_col, end_col)
    }
}
//}}}