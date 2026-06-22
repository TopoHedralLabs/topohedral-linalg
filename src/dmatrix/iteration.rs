//! Iterator support for [`DMatrix`]: owned, shared, and mutable element iteration.
//!
//! Implements `IntoIterator` for owned [`DMatrix<T>`], `&DMatrix<T>`, and `&mut DMatrix<T>`,
//! as well as the ergonomic `iter()` and `iter_mut()` shorthand methods. All iterators traverse
//! elements in column-major order, consistent with the underlying storage layout, enabling the
//! matrix to participate in standard Rust iterator chains without any special adaptors.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: into iterator conversion
//{{{ impl: IntoIterator for SMatrix
impl<T> IntoIterator for DMatrix<T>
where
    T: Copy,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.into_iter()
    }
}

//}}}
//{{{ impl: IntoIterator for &a' SMatrix
impl<'a, T> IntoIterator for &'a DMatrix<T>
where
    T: Copy,
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.data.iter()
    }
}
//}}}

//{{{ impl: DMatrix
impl<T> DMatrix<T>
where
    T: Copy,
{
    //{{{ fun: iter
    /// Returns a column-major immutable iterator over all elements of the matrix.
    pub fn iter(&self) -> std::slice::Iter<'_, T>
    {
        self.data.iter()
    }
    //}}}
    //{{{ fun: iter_mut
    /// Returns a column-major mutable iterator over all elements of the matrix.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>
    {
        self.data.iter_mut()
    }
    //}}}
}
//}}}
//}}} // collection: into iterator conversion
