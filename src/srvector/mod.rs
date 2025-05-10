//! Provides a statically-sized column vector type
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::smatrix::SMatrix;
use crate::common::{Field, Float, FloatVectorOps, One, VectorOps, Zero, AssertGreaterThan, IsTrue};
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
    T: Field + Default + Copy + Clone + Zero + One,
    AssertGreaterThan<N, 1> : IsTrue,
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
    AssertGreaterThan<N, 1> : IsTrue,
{
}
//}}}
