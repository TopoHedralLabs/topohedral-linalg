
//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------



//{{{ crate imports 
use crate::common::{Field, Zero, One, Float};
use crate::matrix::smatrix::{EvaluateSMatrix, SMatrix};
use super::common::{VectorOps, FloatVectorOps, Assert, IsTrue};
//}}}
//{{{ std imports 
use std::ops::{Deref, DerefMut, Index, IndexMut};
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

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