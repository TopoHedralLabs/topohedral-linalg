//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------



//{{{ crate imports 
use crate::common::{Field, Zero, One, Float};
use crate::matrix::smatrix::SMatrix;
use super::common::{VectorOps, FloatVectorOps, Assert, IsTrue};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

pub type SCVector<T, const N: usize> = SMatrix<T, N, 1>;

impl<T, const N: usize> SCVector<T, N>
where
    [();  N * 1]:,
    T: Field + Default + Copy + std::fmt::Display + Clone,
    Assert<{N > 1}>: IsTrue,
{
    pub fn len() -> usize
    {
        N
    }
}

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

impl<T, const N: usize> FloatVectorOps for SCVector<T, N> 
where
    [(); N * 1]:,
    T: Float + Default + Copy + Clone + Zero + One,
    Assert<{N > 1}>: IsTrue,
{}

pub type SRVector<T, const N: usize> = SMatrix<T, 1, N>;

impl<T, const N: usize>  VectorOps for SRVector<T, N>
where
    [(); 1usize * N]:,
    T: Field + Default + Copy + Clone + Zero + One,
    Assert<{N > 1}>: IsTrue,
{

    type ScalarType = T;

    fn len(&self) -> usize
    {
        N
    }
}

impl<T, const N: usize> FloatVectorOps for SRVector<T, N> 
where
    [(); 1usize * N]:,
    T: Float + Default + Copy + Clone + Zero + One,
    Assert<{N > 1}>: IsTrue,
{}