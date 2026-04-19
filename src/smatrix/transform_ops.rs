//! Elementwise transformation operations for static matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::{subviews::MatrixViewMut, SMatrix};
use crate::common::{Field, TransformOps};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

impl<T, const N: usize, const M: usize> TransformOps for SMatrix<T, N, M>
where
    [(); N * M]:,
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

impl<'a, T, const N: usize, const M: usize> TransformOps for MatrixViewMut<'a, T, N, M>
where
    [(); N * M]:,
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
