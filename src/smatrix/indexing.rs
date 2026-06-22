//! Index and IndexMut implementations for [`SMatrix`] using (row, col) pairs.
//!
//! Provides `Index<(usize, usize)>` and `IndexMut<(usize, usize)>` for [`SMatrix<T, N, M>`],
//! converting the two-dimensional subscript into the linear column-major offset `col * N + row`.
//! The const-generic `N` (row count) is known at compile time, so the offset computation can
//! often be optimised to a single multiply-add instruction.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
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
impl<T, const N: usize, const M: usize> Index<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Copy,
{
    type Output = T;

    fn index(
        &self,
        index: (usize, usize),
    ) -> &Self::Output
    {
        let lin_idx = lin_index(index, N);
        &self.data[lin_idx]
    }
}
//}}}
//{{{ impl: Index<(usize, usize)> for &SMatrix
impl<T, const N: usize, const M: usize> Index<(usize, usize)> for &SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: Index<(usize, usize)> for &mut SMatrix
impl<T, const N: usize, const M: usize> Index<(usize, usize)> for &mut SMatrix<T, N, M>
where
    [(); N * M]:,
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
impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Copy,
{
    fn index_mut(
        &mut self,
        index: (usize, usize),
    ) -> &mut Self::Output
    {
        let lin_idx = lin_index(index, N);
        &mut self.data[lin_idx]
    }
}
//}}}
//{{{ impl: IndexMut<(usize, usize)> for &mut SMatrix
impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for &mut SMatrix<T, N, M>
where
    [(); N * M]:,
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
impl<T, const N: usize, const M: usize> Index<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: Index<usize> for &SMatrix
impl<T, const N: usize, const M: usize> Index<usize> for &SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: Index<usize> for &mut SMatrix
impl<T, const N: usize, const M: usize> Index<usize> for &mut SMatrix<T, N, M>
where
    [(); N * M]:,
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
impl<T, const N: usize, const M: usize> IndexMut<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: IndexMut<usize> for &mut SMatrix
impl<T, const N: usize, const M: usize> IndexMut<usize> for &mut SMatrix<T, N, M>
where
    [(); N * M]:,
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
//{{{ impl: MatrixExpr for SMatrix
impl<T, const N: usize, const M: usize> MatrixExpr for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Copy,
{
    type ScalarType = T;

    #[inline]
    fn linear_value(
        &self,
        index: usize,
    ) -> Self::ScalarType
    {
        self.data[index]
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
