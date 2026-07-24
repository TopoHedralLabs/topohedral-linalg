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
impl<T, const N: usize, const M: usize> IntoIterator for SMatrix<T, N, M> {
    type Item = T;

    type IntoIter = std::iter::Flatten<std::array::IntoIter<[T; N], M>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().flatten()
    }
}

//}}}
//{{{ impl: IntoIterator for &a' SMatrix
impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a SMatrix<T, N, M> {
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
//}}}
//{{{ impl: IntoIterator for &mut SMatrix
impl<'a, T, const N: usize, const M: usize> IntoIterator for &'a mut SMatrix<T, N, M> {
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}
//}}}

//{{{ impl: slice conversions for SMatrix
impl<T, const N: usize, const M: usize> AsRef<[T]> for SMatrix<T, N, M> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize, const M: usize> AsMut<[T]> for SMatrix<T, N, M> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
//}}}
//{{{ impl: SMatrix
impl<T, const N: usize, const M: usize> SMatrix<T, N, M> {
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
