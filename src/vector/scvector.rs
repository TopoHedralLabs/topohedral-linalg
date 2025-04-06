//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Field, Zero, One, Float};
use crate::matrix::smatrix::{EvaluateSMatrix, SMatrix};
use super::common::{VectorOps, FloatVectorOps};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: SCVector
/// A statically-sized column vector type.
pub type SCVector<T, const N: usize> = SMatrix<T, N, 1>;
//}}}
//{{{ impl: SCector
impl<T, const N: usize> SCVector<T, N>
where
    [(); N * 1]:,
    T: Field + Default + Copy,
{
    /// Creates a new vector from an array
    pub fn from_slice(data: &[T; N]) -> Self
    where
        T: Copy + Default,
    {
        let mut out = Self::default();
        out.data.copy_from_slice(data);
        out
    }
}
//}}}
impl<T, const N: usize> VectorOps for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{

    type ScalarType = T;

    /// Returns the number of elements in the vector
    fn len(&self) -> usize
    {
        N
    }
}

impl<T, const N: usize> FloatVectorOps for SCVector<T, N>
where
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero + Float,
{}