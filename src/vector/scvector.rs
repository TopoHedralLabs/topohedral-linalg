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
use std::ops::{Deref, DerefMut, Index, IndexMut};
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: SCVector
/// A statically-sized column vector type.
#[derive(Copy, Clone, Debug)]
pub struct SCVector<T, const N: usize>(SMatrix<T, N, 1>) where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero;
//}}}

impl<T, const N: usize>  Zero for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    fn zero() -> Self {
        SCVector(SMatrix::zero())
    }
}

impl<T, const N: usize>  One for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    fn one() -> Self {
        SCVector(SMatrix::one())
    }
}

impl<T, const N: usize> Default for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    fn default() -> Self {
        SCVector(SMatrix::default())
    }
}

impl<T, const N: usize> Deref for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    type Target = SMatrix<T, N, 1>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


impl<T, const N: usize> VectorOps for SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    type ScalarType = T;
    
    fn len(&self) -> usize {
        N
    }
}

impl<T, const N: usize> FloatVectorOps for SCVector<T, N>
where
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero + Float,
{}

impl<T, const N: usize> SCVector<T, N> 
where 
    [(); N * 1]:,
    T: Field + Default + Copy + One + Zero,
{
    pub fn from_value(val: T) -> Self {
        SCVector(SMatrix::from_value(val))
    }
    
}