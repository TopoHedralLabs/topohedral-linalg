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
    T: Copy,
{
    type Item = T;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<[T; N], M>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().flatten()
    }
}

//}}}
//{{{ impl: IntoIterator for &a' SMatrix
impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a SMatrix<T, N, M>
where
    T: Copy,
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
//}}}
//{{{ impl: SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M>
where
    T: Copy,
{
    //{{{ fun: iter
    /// Returns an iterator over shared references to elements in column-major order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }
    //}}}
    //{{{ fun: iter_mut
    /// Returns an iterator over mutable references to elements in column-major order.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }
    //}}}
}
//}}}
//}}}
