//! In-place element-wise transformation for [`DMatrix`].
//!
//! Implements the [`TransformOps`] trait for [`DMatrix<T>`], providing a `transform` method that
//! applies a caller-supplied closure to every element of the matrix in place in column-major order.
//! No extra allocation is required because the transformation is purely in-place, making this an
//! efficient complement to the lazy arithmetic operators for cases where the transformation cannot
//! be expressed as a simple arithmetic expression.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::{subviews::MatrixViewMut, DMatrix};
use crate::common::{Field, TransformOps};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: TransformOps for DMatrix
impl<T> TransformOps for DMatrix<T>
where
    T: Field + Copy,
{
    type ScalarType = T;

    fn transform<F>(
        &mut self,
        mut f: F,
    ) where
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        for value in &mut self.data
        {
            *value = f(*value);
        }
    }
}
//}}}

//{{{ impl: TransformOps for MatrixViewMut
impl<'a, T> TransformOps for MatrixViewMut<'a, T>
where
    T: Field + Copy,
{
    type ScalarType = T;

    fn transform<F>(
        &mut self,
        mut f: F,
    ) where
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        for col in 0..self.ncols
        {
            for row in 0..self.nrows
            {
                let value = self[(row, col)];
                (*self)[(row, col)] = f(value);
            }
        }
    }
}
//}}}
