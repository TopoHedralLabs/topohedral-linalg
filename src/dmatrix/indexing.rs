//! Index and IndexMut implementations for [`DMatrix`] using (row, col) pairs.
//!
//! Provides `Index<(usize, usize)>` and `IndexMut<(usize, usize)>` for [`DMatrix<T>`], converting
//! the two-dimensional (row, column) subscript into the linear column-major offset
//! `col * nrows + row`. This makes element access syntax idiomatic (`matrix[(i, j)]`) while
//! preserving the underlying column-major layout that BLAS and LAPACK expect.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::{lin_index, MatrixExpr};
//}}}
//{{{ std imports
use std::ops::{Index, IndexMut};
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: Index Pair Indexing
//{{{ impl: Index<(usize, usize)> for SMatrix
impl<T> Index<(usize, usize)> for DMatrix<T>
where
    T: Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let lin_idx = lin_index(index, self.nrows);
        &self.data[lin_idx]
    }
}
//}}}
//{{{ impl: Index<(usize, usize)> for &DMatrix
impl<T> Index<(usize, usize)> for &DMatrix<T>
where
    T: Copy,
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
//{{{ impl: Index<(usize, usize)> for &mut DMatrix
impl<T> Index<(usize, usize)> for &mut DMatrix<T>
where
    T: Copy,
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
//{{{ impl: IndexMut<(usize, usize)> for SMatrix
impl<T> IndexMut<(usize, usize)> for DMatrix<T>
where
    T: Copy,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let lin_idx = lin_index(index, self.nrows);
        &mut self.data[lin_idx]
    }
}
//}}}
//{{{ impl: IndexMut<(usize, usize)> for &mut DMatrix
impl<T> IndexMut<(usize, usize)> for &mut DMatrix<T>
where
    T: Copy,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        &mut (**self)[index]
    }
}
//}}}
//}}}
//{{{ collection: Single integer indexing
//{{{ impl: Index<usize> for SMatrix
impl<T> Index<usize> for DMatrix<T>
where
    T: Copy,
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output
    {
        &self.data[index]
    }
}

//}}}
//{{{ impl: Index<usize> for &DMatrix
impl<T> Index<usize> for &DMatrix<T>
where
    T: Copy,
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output
    {
        &(**self)[index]
    }
}
//}}}
//{{{ impl: Index<usize> for &mut DMatrix
impl<T> Index<usize> for &mut DMatrix<T>
where
    T: Copy,
{
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output
    {
        &(**self)[index]
    }
}
//}}}
//{{{ impl: IndexMut<usize> for SMatrix
impl<T> IndexMut<usize> for DMatrix<T>
where
    T: Copy,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output
    {
        &mut self.data[index]
    }
}

//}}}
//{{{ impl: IndexMut<usize> for &mut DMatrix
impl<T> IndexMut<usize> for &mut DMatrix<T>
where
    T: Copy,
{
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output
    {
        &mut (**self)[index]
    }
}
//}}}
//{{{ impl: MatrixExpr for DMatrix
impl<T> MatrixExpr for DMatrix<T>
where
    T: Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType
    {
        // Safety: expression tree evaluation always iterates 0..nrows*ncols, and
        // data has exactly nrows*ncols elements. Eliminating this bounds check
        // allows LLVM to auto-vectorize expression evaluation loops.
        unsafe { *self.data.get_unchecked(index) }
    }

    #[inline]
    fn eval_into(
        &self,
        out: &mut [T],
    )
    {
        out.copy_from_slice(&self.data);
    }
}
//}}}
//}}}
