//! Static column-vector type built as an N×1 specialisation of [`SMatrix`].
//!
//! Provides the `SCVector<T, N>` type alias (`SMatrix<T, N, 1>`) along with implementations of
//! [`VectorOps`] and [`FloatVectorOps`] for column vectors. A compile-time assertion requires
//! N > 1. Like its row counterpart in `srvector`, the alias approach means all [`SMatrix`]
//! infrastructure is inherited automatically, and the vector-specific traits add only the
//! operations that are conceptually distinct for 1-D objects.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::smatrix::SMatrix;
use crate::{
    common::{Field, Float, FloatVectorOps, One, VectorOps, Zero},
    GreaterThan,
};
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: SCVector
/// A type alias for a column vector of size N.
pub type SCVector<T, const N: usize> = SMatrix<T, N, 1>;
//}}}
//{{{ impl: VectorOps for SCVector
#[allow(clippy::identity_op)]
impl<T, const N: usize> VectorOps for SCVector<T, N>
where
    [(); N * 1]:,
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
impl<T, const N: usize> FloatVectorOps for SCVector<T, N>
where
    [(); N * 1]:,
    T: Float + Default + Copy + Clone + Zero + One,
    (): GreaterThan<N, 1>,
{
}
//}}}
