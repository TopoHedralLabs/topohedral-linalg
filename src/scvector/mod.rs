//! Provides a statically-sized column vector type
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::smatrix::SMatrix;
use crate::common::{VectorOps, FloatVectorOps, Zero, One, Field, Float};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: compile-time checks
/// Assertion struct for compile-time checks
struct Assert<const check: bool>;
/// This trait is used to ensure that the compile-time check is true
trait IsTrue {}
impl IsTrue for Assert<true> {}
//}}}
//{{{ type: SCVector
/// A type alias for a column vector of size N.
pub type SCVector<T, const N: usize> = SMatrix<T, N, 1>;
//}}}
//{{{ impl: VectorOps for SCVector
impl<T, const N: usize>  VectorOps for SCVector<T, N>
where
    [(); N * 1]:,
    T: Field + Default + Copy + Clone + Zero + One,
    Assert<{N > 1}>: IsTrue,
{

    type ScalarType = T;

    fn len(&self) -> usize
    {
        N
    }
}
//}}}
//{{{ impl: FloatVectorOps for SCVector
impl<T, const N: usize> FloatVectorOps for SCVector<T, N> 
where
    [(); N * 1]:,
    T: Float + Default + Copy + Clone + Zero + One,
    Assert<{N > 1}>: IsTrue,
{}
//}}}


