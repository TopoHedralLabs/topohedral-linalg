//! Iterator support for [`SMatrix`]: owned, shared, and mutable element iteration.
//!
//! Implements `IntoIterator` for owned [`SMatrix<T, N, M>`], `&SMatrix<T, N, M>`, and
//! `&mut SMatrix<T, N, M>`, plus ergonomic `iter()` and `iter_mut()` methods. All iterators
//! traverse elements in column-major order, consistent with the underlying fixed-size array
//! layout, allowing the matrix to participate in standard Rust iterator chains without special
//! adaptors.
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
    T: Field + Copy,
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
    T: Field + Copy,
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
    pub fn iter(&self) -> std::slice::Iter<'_, T>
    {
        self.data.iter()
    }
    //}}}
    //{{{ fun: iter_mut
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>
    {
        self.data.iter_mut()
    }
    //}}}
}
//}}}
