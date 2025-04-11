//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::SMatrix;
use crate::common::{Field, IndexValue};
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
    T: Field + Default + Copy,
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
    T: Field + Default + Copy,
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
    T: Field + Default + Copy
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
    T: Field + Default + Copy
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
    T: Field + Default + Copy
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
    T: Field + Default + Copy
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
//{{{ fun: lin_index
#[inline]
fn lin_index(
    idx: (usize, usize),
    N: usize,
) -> usize
{
    idx.0 + idx.1 * N
}
//}}}