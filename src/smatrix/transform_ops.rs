//! Elementwise transformation operations for static matrices.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
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
