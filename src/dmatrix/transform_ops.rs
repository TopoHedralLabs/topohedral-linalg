//! Elementwise transformation operations for dynamic matrices.
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
