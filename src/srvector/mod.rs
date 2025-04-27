//! Provides a statically-sized column vector type
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::smatrix::SMatrix;
use crate::common::{Field, Float, FloatVectorOps, One, VectorOps, Zero};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: compile-time checks
/// Assertion struct for compile-time checks
struct Assert<const CHECK: bool>;
/// This trait is used to ensure that the compile-time check is true
trait IsTrue {}
impl IsTrue for Assert<true> {}
//}}}
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
    Assert<{ N > 1 }>: IsTrue,
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
    Assert<{ N > 1 }>: IsTrue,
{
}
//}}}
