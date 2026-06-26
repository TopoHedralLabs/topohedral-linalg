//! In-place element-wise transformation for [`SMatrix`].
//!
//! Implements the [`TransformOps`] trait for [`SMatrix<T, N, M>`], providing a `transform` method
//! that applies a caller-supplied closure to every element in place in column-major order. No extra
//! allocation is required because the transformation is purely in-place, making this an efficient
//! complement to the lazy arithmetic operators for cases where the transformation cannot be
//! expressed as a simple arithmetic expression.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::SMatrix;
use crate::common::TransformOps;
use crate::subviews::MatrixViewMut;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: TransformOps for SMatrix
impl<T, const N: usize, const M: usize> TransformOps for SMatrix<T, N, M>
where
    T: Copy,
{
    type ScalarType = T;

    fn transform<F>(
        &mut self,
        mut f: F,
    ) where
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        for value in self.as_mut_slice() {
            *value = f(*value);
        }
    }
}
//}}}
//{{{ impl: TransformOps for MatrixViewMut
impl<'a, T, const N: usize, const M: usize> TransformOps for MatrixViewMut<'a, SMatrix<T, N, M>>
where
    T: Copy,
{
    type ScalarType = T;

    fn transform<F>(
        &mut self,
        mut f: F,
    ) where
        F: FnMut(Self::ScalarType) -> Self::ScalarType,
    {
        for col in 0..self.ncols {
            for row in 0..self.nrows {
                let value = self[(row, col)];
                (*self)[(row, col)] = f(value);
            }
        }
    }
}
//}}}
