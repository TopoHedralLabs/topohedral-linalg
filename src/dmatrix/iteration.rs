//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::DMatrix;
use crate::common::Field;
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
    T: Field + Copy,
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

impl<T> DMatrix<T>
where
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
