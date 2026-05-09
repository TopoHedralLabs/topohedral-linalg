//! Static row-vector type built as a 1×N specialisation of [`SMatrix`].
//!
//! Provides the `SRVector<T, N>` type alias (`SMatrix<T, 1, N>`) along with implementations of
//! [`VectorOps`] and [`FloatVectorOps`] for row vectors. A compile-time assertion requires N > 1.
//! Using a type alias rather than a newtype means all [`SMatrix`] methods and trait implementations
//! are available without duplication; the vector-specific traits add only the operations that are
//! conceptually distinct for 1-D objects.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::smatrix::SMatrix;
use crate::common::{Field, Float, FloatVectorOps, GreaterThan, One, VectorOps, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: SCVector
/// A type alias for a column vector of size N.
pub type SRVector<T, const N: usize> = SMatrix<T, 1, N>;
//}}}
//{{{ impl: VectorOps for SCVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> VectorOps for SRVector<T, N>
where
    [(); 1usize * N]:,
    T: Field + Default + Copy + Clone + Zero + One + Float,
    (): GreaterThan<N, 1>,
{
    type ScalarType = T;

    fn len(&self) -> usize
    {
        N
    }
}
//}}}
//{{{ impl: FloatVectorOps for SCVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> FloatVectorOps for SRVector<T, N>
where
    [(); 1usize * N]:,
    T: Float + Default + Copy + Clone + Zero + One,
    (): GreaterThan<N, 1>,
{
}
//}}}
