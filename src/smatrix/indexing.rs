//! This module contains functions for indexing SMatrix objects.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::{lin_index, Field, IndexValue};
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
    T: Field + Copy,
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
//{{{ impl: IndexMut<(usize, usize)> for SMatrix
impl<T, const N: usize, const M: usize> IndexMut<(usize, usize)> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
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
//}}}
//{{{ collection: Single integer indexing
//{{{ impl: Index<usize> for SMatrix
impl<T, const N: usize, const M: usize> Index<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
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
//{{{ impl: IndexMut<usize> for SMatrix
impl<T, const N: usize, const M: usize> IndexMut<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
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
//{{{ impl: IndexValue<usize> for SMatrix
impl<T, const N: usize, const M: usize> IndexValue<usize> for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = T;

    #[inline]
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        self.data[index]
    }
}

//}}}
//{{{ impl: IndexValue for &'a SMatrix
impl<T, const N: usize, const M: usize> IndexValue<usize> for &SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    type Output = T;

    #[inline]
    fn index_value(
        &self,
        index: usize,
    ) -> Self::Output
    {
        self.data[index]
    }
}

//}}}
//}}}
