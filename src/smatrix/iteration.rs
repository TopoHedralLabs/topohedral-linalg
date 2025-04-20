//! This module contains functions for iterating over SMatrix objects.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::SMatrix;
use crate::common::Field;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: into iterator conversion
//{{{ impl: IntoIterator for SMatrix
impl<T, const N: usize, const M: usize> IntoIterator for SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy
{
    type Item = T;

    type IntoIter = std::array::IntoIter<T, { N * M }>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.into_iter()
    }
}

//}}}
//{{{ impl: IntoIterator for &a' SMatrix
impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.iter()
    }
}
//}}}

impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    [(); N * M]:,
    T: Field + Copy,
{
    //{{{ fun: iter
    pub fn iter(&self) -> std::slice::Iter<'_, T>{
        return self.data.iter();
    }
    //}}}
    //{{{ fun: iter_mut
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>{
        return self.data.iter_mut();
    }
    //}}}
}
//}}}